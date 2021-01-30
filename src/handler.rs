use crate::models::*;
use crate::{helpermodels::*, schema};
use actix_rt::blocking;
use actix_web::{client::JsonPayloadError, delete, dev, get, post, web, HttpResponse, Responder};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
    replace_into,
};
use diesel::{sql_function_body, SqliteConnection};
use rumqttc::AsyncClient;
use serde::{Deserialize, Serialize};
use std::{any::Any, sync::Arc};
use tokio::sync::{watch, Mutex};

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// Register device

#[derive(Serialize, Deserialize)]
struct DeviceIdentifier {
    mac: String,
    ip: String,
    d_ty: String, // Device Type
}

#[get("/check-resp")]
pub async fn check_resp() -> impl Responder {
    println!("Req sent");
    HttpResponse::Ok().body("OK")
}

#[post("/register-device")]
pub async fn register_device(
    pool: web::Data<DbPool>,
    device_identifier: web::Json<DeviceIdentifier>,
) -> impl Responder {
    // Search for existing mac address
    match pool.get() {
        Ok(pool_res) => {
            let found_device_res = web::block(move || {
                use schema::devices::dsl::*;

                match devices
                    .filter(mac.eq(device_identifier.mac.clone()))
                    .first::<Device>(&pool_res)
                {
                    Ok(device_res) => {
                        println!("Device found {:?}", device_res);
                        Ok(device_res as Device)
                    }
                    Err(_) => {
                        println!(
                            "Device not found with mac {:?}, creating.",
                            device_identifier.mac
                        );

                        // Find device type
                        use schema::device_types::dsl::{device_types, name};

                        let found_device_type: Result<DeviceType, _> = device_types
                            .filter(name.eq(device_identifier.d_ty.as_str()))
                            .first::<DeviceType>(&pool_res);

                        diesel::replace_into(devices)
                            .values(Device {
                                id: None,
                                name: Some(String::from("")),
                                serial_number: Some(String::from("")),
                                device_type_id: match found_device_type {
                                    Ok(device_type) => Some(device_type.id.unwrap_or_default()),
                                    Err(e) => None,
                                },
                                created_at: None,
                                updated_at: None,
                                mac: Some(device_identifier.mac.clone()),
                                ip: Some(device_identifier.ip.clone()),
                            })
                            .execute(&pool_res);

                        let inserted_id =
                            diesel::select(last_insert_rowid).get_result::<i32>(&pool_res);
                        let saved_device = devices
                            .order_by(id.eq(inserted_id.unwrap_or_default()))
                            .first::<Device>(&pool_res);
                        println!("Saved device: {:?}", saved_device);

                        saved_device
                    }
                }
            })
            .await;

            match found_device_res {
                Ok(device_res) => HttpResponse::Created().json(device_res.id),
                Err(e) => HttpResponse::InternalServerError()
                    .body("Error saving device or blocking error"),
            }
        }
        _ => HttpResponse::InternalServerError().body("Failed getting pool"),
    }
}

// Path schedules

#[get("/schedules")]
pub async fn all_schedules(pool: web::Data<DbPool>) -> impl Responder {
    match pool.get() {
        Ok(pool_res) => {
            let schedules = web::block(move || {
                use schema::schedules::dsl::*;
                schedules.load::<Schedule>(&pool_res)
            })
            .await;

            match schedules {
                Ok(schedules_res) => HttpResponse::Ok().json(schedules_res),
                _ => HttpResponse::InternalServerError().body("Error getting schedules"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error getting pool"),
    }
}

#[get("/schedules-view")]
pub async fn view_schedules(pool: web::Data<DbPool>) -> impl Responder {
    match pool.get() {
        Ok(pool_res) => {
            let schedules: Result<Vec<ScheduleView>, _> = web::block(move || {
                use schema::schedules::dsl::*;

                match schedules.load::<Schedule>(&pool_res) {
                    Ok(schedules_res) => Ok(schedules_res
                        .into_iter()
                        .map(|schedule| {
                            let found_device_schedules: Result<Vec<DeviceSchedule>, _> =
                                DeviceSchedule::belonging_to(&schedule).load(&pool_res);

                            ScheduleView {
                                schedule,
                                devices: match found_device_schedules {
                                    Ok(found_device_schedules_res) => found_device_schedules_res
                                        .into_iter()
                                        .map(|device_schedule| {
                                            use schema::devices::dsl::*;

                                            match devices
                                                .filter(id.eq(device_schedule.device_id))
                                                .first::<Device>(&pool_res)
                                            {
                                                Ok(dev) => Some(dev as Device),
                                                _ => None,
                                            }
                                        })
                                        .filter(|dev| dev.is_some())
                                        .map(|dev| dev.unwrap())
                                        .collect(),
                                    _ => vec![],
                                },
                            }
                        })
                        .collect()),
                    Err(e) => Err(e),
                }
            })
            .await;

            match schedules {
                Ok(schedules_res) => HttpResponse::Ok().json(schedules_res),
                Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/schedules/{schedule_id}")]
pub async fn get_schedule(pool: web::Data<DbPool>, schedule_id: web::Path<i32>) -> impl Responder {
    match pool.get() {
        Ok(pool_res) => {
            let schedule = web::block(move || {
                use schema::schedules::dsl::*;
                schedules
                    .filter(id.eq(schedule_id.into_inner()))
                    .first::<Schedule>(&pool_res)
            })
            .await;

            match schedule {
                Ok(schedules_res) => HttpResponse::Ok().json(schedules_res),
                _ => HttpResponse::NotFound().body("Schedule not found"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error getting pool"),
    }
}

#[post("/schedules")]
pub async fn post_schedule(
    pool: web::Data<DbPool>,
    schedule: web::Json<Schedule>,
) -> impl Responder {
    match pool.get() {
        Ok(pool_res) => {
            let schedule_res = web::block(move || {
                use schema::schedules::dsl::*;
                match diesel::replace_into(schedules)
                    .values(&schedule.into_inner())
                    .execute(&pool_res)
                {
                    Ok(_) => {
                        let last_id =
                            diesel::select(last_insert_rowid).get_result::<i32>(&pool_res);

                        schedules
                            .order_by(id.eq(last_id.unwrap_or_default()))
                            .first::<Schedule>(&pool_res)
                    }
                    Err(e) => Err(e),
                }
            })
            .await;

            match schedule_res {
                Ok(schedule_res_body) => HttpResponse::Created().json(schedule_res_body),
                Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/schedules-save")]
pub async fn save_schedule(
    pool: web::Data<DbPool>,
    schedule: web::Json<ScheduleView>,
) -> impl Responder {
    match pool.get() {
        Ok(pool_res) => {
            // let created =

            HttpResponse::Created().body("OK")
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/schedules/{schedule_id}")]
pub async fn delete_schedule(
    pool: web::Data<DbPool>,
    schedule_id: web::Path<i32>,
) -> impl Responder {
    match pool.get() {
        Ok(pool_res) => {
            let schedule = web::block(move || {
                use schema::schedules::dsl::*;
                diesel::delete(schedules.filter(id.eq(schedule_id.into_inner()))).execute(&pool_res)
            })
            .await;

            HttpResponse::Ok().body("OK")
        }
        Err(_) => HttpResponse::InternalServerError().body("Error getting pool"),
    }
}

// Path watering times

#[get("/wateringtimes")]
pub async fn all_wateringtimes(pool: web::Data<DbPool>) -> impl Responder {
    match pool.get() {
        Ok(pool_res) => {
            let watering_times = web::block(move || {
                use schema::watering_times::dsl::*;
                watering_times.load::<WateringTime>(&pool_res)
            })
            .await;

            match watering_times {
                Ok(watering_times_res) => HttpResponse::Ok().json(watering_times_res),
                _ => HttpResponse::InternalServerError().body("Error getting watering times"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error getting pool"),
    }
}

#[get("/wateringtimes/{wateringtime_id}")]
pub async fn get_wateringtime(
    pool: web::Data<DbPool>,
    wateringtime_id: web::Path<i32>,
) -> impl Responder {
    match pool.get() {
        Ok(pool_res) => {
            let watering_time = web::block(move || {
                use schema::watering_times::dsl::*;
                watering_times
                    .filter(id.eq(wateringtime_id.into_inner()))
                    .first::<WateringTime>(&pool_res)
            })
            .await;

            match watering_time {
                Ok(watering_time_res) => HttpResponse::Ok().json(watering_time_res),
                _ => HttpResponse::NotFound().body("Schedule not found"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error getting pool"),
    }
}

#[post("/wateringtimes")]
pub async fn post_wateringtime(
    pool: web::Data<DbPool>,
    watering_time: web::Json<WateringTime>,
) -> impl Responder {
    match pool.get() {
        Ok(pool_res) => {
            let watering_time_res = web::block(move || {
                use schema::watering_times::dsl::*;
                match diesel::replace_into(watering_times)
                    .values(&watering_time.into_inner())
                    .execute(&pool_res)
                {
                    Ok(_) => {
                        let last_id =
                            diesel::select(last_insert_rowid).get_result::<i32>(&pool_res);

                        watering_times
                            .filter(id.eq(last_id.unwrap_or_default()))
                            .first::<WateringTime>(&pool_res)
                    }
                    Err(e) => Err(e),
                }
            })
            .await;

            match watering_time_res {
                Ok(watering_time_body) => HttpResponse::Created().json(watering_time_body),
                _ => HttpResponse::InternalServerError().body("Error getting schedules"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error getting pool"),
    }
}

#[delete("/wateringtimes/{wateringtime_id}")]
pub async fn delete_wateringtime(
    pool: web::Data<DbPool>,
    wateringtime_id: web::Path<i32>,
) -> impl Responder {
    match pool.get() {
        Ok(pool_res) => {
            let schedule = web::block(move || {
                use schema::watering_times::dsl::*;
                diesel::delete(watering_times.filter(id.eq(wateringtime_id.into_inner())))
                    .execute(&pool_res)
            })
            .await;

            HttpResponse::Ok().body("OK")
        }
        Err(_) => HttpResponse::InternalServerError().body("Error getting pool"),
    }
}

// Devices
#[get("/devices")]
pub async fn all_devices(pool: web::Data<DbPool>) -> impl Responder {
    match pool.get() {
        Ok(pool_res) => {
            match web::block(move || {
                use schema::devices::dsl::*;
                devices.load::<Device>(&pool_res)
            })
            .await
            {
                Ok(devices) => HttpResponse::Ok().json(devices),
                Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
