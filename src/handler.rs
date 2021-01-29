use crate::models::*;
use crate::{helpermodels::*, schema};
use actix_rt::blocking;
use actix_web::{client::JsonPayloadError, delete, dev, get, post, web, HttpResponse, Responder};
use diesel::SqliteConnection;
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
    replace_into,
};
use rumqttc::AsyncClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{watch, Mutex};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// Register device

#[derive(Serialize, Deserialize)]
struct DeviceIdentifier {
    mac: String,
    ip: String,
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
                    Err(e) => {
                        println!(
                            "Device not found with mac {:?}, creating.",
                            device_identifier.mac
                        );

                        diesel::replace_into(devices)
                            .values(Device {
                                id: None,
                                name: Some(String::from("")),
                                serial_number: Some(String::from("")),
                                device_type_id: None,
                                created_at: None,
                                updated_at: None,
                                mac: Some(device_identifier.mac.clone()),
                                ip: Some(device_identifier.ip.clone()),
                            })
                            .execute(&pool_res);

                        let saved_device = devices.order_by(id.desc()).first::<Device>(&pool_res);
                        println!("Saved device: {:?}", saved_device);

                        saved_device
                    }
                }
            })
            .await;

            match found_device_res {
                Ok(device_res) => HttpResponse::Created().json(device_res),
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
                    Ok(_) => schedules.order_by(id.desc()).first::<Schedule>(&pool_res),
                    Err(e) => Err(e),
                }
            })
            .await;

            match schedule_res {
                Ok(schedule_res_body) => HttpResponse::Ok().json(schedule_res_body),
                _ => HttpResponse::InternalServerError().body("Error getting schedules"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error getting pool"),
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
                    Ok(_) => watering_times
                        .order_by(id.desc())
                        .first::<WateringTime>(&pool_res),
                    Err(e) => Err(e),
                }
            })
            .await;

            match watering_time_res {
                Ok(watering_time_body) => HttpResponse::Ok().json(watering_time_body),
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
