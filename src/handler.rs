use crate::helpermodels::*;
use crate::models::*;
use diesel::prelude::*;
use diesel::SqliteConnection;
use rumqttc::AsyncClient;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn schedule_req(
    conn_arc: Arc<Mutex<SqliteConnection>>,
    client_arc: Arc<Mutex<AsyncClient>>,
) {
    use crate::schema::schedules::dsl::*;

    let conn = conn_arc.lock().await;
    let schedules_res = schedules.load::<Schedule>(&*conn);

    match schedules_res {
        Ok(schedules_list) => {
            let client = client_arc.lock().await;

            if let Ok(schedules_string) = serde_json::to_string(&schedules_list) {
                client
                    .publish(
                        "schedules/res",
                        rumqttc::QoS::AtLeastOnce,
                        false,
                        schedules_string,
                    )
                    .await;
            }
        }
        Err(_) => println!("Error gettings schedules!"),
    }
}

pub async fn schedule_req_save(
    conn_arc: Arc<Mutex<SqliteConnection>>,
    // client: Arc<Mutex<AsyncClient>>,
    payload: String,
) {
    use crate::schema::schedules::dsl::*;

    let schedule_res = serde_json::from_str::<Schedule>(payload.as_str());

    if let Ok(schedule) = schedule_res {
        println!("Schedule: {:?}", schedule);
        let conn = conn_arc.lock().await;
        diesel::replace_into(schedules)
            .values(&schedule)
            .execute(&*conn);
    }
}

pub async fn schedule_delete(conn_arc: Arc<Mutex<SqliteConnection>>, payload: String) {
    use crate::schema::schedules::dsl::*;
    let idBody = serde_json::from_str::<IdBody>(payload.as_str());

    match idBody {
        Ok(idBody) => {
            println!("ID body: {} - {:?}", idBody.id, idBody);

            let conn = conn_arc.lock().await;

            diesel::delete(schedules.filter(id.eq(idBody.id))).execute(&*conn);
        }
        Err(_) => {
            println!("Error decoding ID body!");
        }
    }
}

pub async fn schedule_res(payload: String) {
    println!("Schedule res received. {}", payload);
}

pub async fn watering_time_req(
    conn_arc: Arc<Mutex<SqliteConnection>>,
    client_arc: Arc<Mutex<AsyncClient>>,
) {
    use crate::schema::watering_times::dsl::{id, watering_times};

    let conn = conn_arc.lock().await;

    let watering_times_res = watering_times.load::<WateringTime>(&*conn);

    if let Ok(watering_times_list) = watering_times_res {
        if let Ok(watering_times_str) = serde_json::to_string(&watering_times_list) {
            println!("{}", watering_times_str);

            let client = client_arc.lock().await;

            client
                .publish(
                    "watering_time/res",
                    rumqttc::QoS::AtLeastOnce,
                    false,
                    watering_times_str,
                )
                .await;
        }
    }
}

pub async fn watering_time_res(payload: String) {
    println!("Watering time res recv: {}", payload);
}

pub async fn watering_req(payload: String) {
    println!("Watering req recv: {}", payload);

    match serde_json::from_str::<WateringBody>(payload.as_str()) {
        Ok(decoded_watering_body) => {
            println!(
                "Decode watering req body success: {}, {:?}, {}",
                decoded_watering_body.watering_type,
                decoded_watering_body.watering_time,
                decoded_watering_body.switch_on
            );
        }
        Err(e) => {
            println!("Watering req payload invalid. {:?}", e);
        }
    }
}
