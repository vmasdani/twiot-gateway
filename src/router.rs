use crate::{handler::*, models::Device};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use rumqttc::AsyncClient;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct WateringBody {
    device_id: i32,
    output: i32,
}

pub async fn route(
    pool: Pool<ConnectionManager<SqliteConnection>>,
    conn_arc: Arc<Mutex<SqliteConnection>>,
    client_arc: Arc<Mutex<AsyncClient>>,
    route: String,
    payload: String,
) {
    println!("Payload: {}", payload);

    match route.as_str() {
        _ => {} // Schedules
                // "schedules/req" => schedule_req(conn_arc, client_arc).await,
                // "schedules/req/save" => schedule_req_save(conn_arc, payload).await,
                // "schedules/req/delete" => schedule_delete(conn_arc, payload).await,
                // "schedules/res" => schedule_res(payload).await,

                // // Watering times
                // "watering_times/req" => watering_time_req(conn_arc, client_arc).await,
                // "watering_times/req/save" => {

                // },
                // "watering_times/req/delete" => {

                // },
                // "watering_times/res" => watering_time_res(payload).await,

                // // Watering (on/off)
                // "watering/req" => watering_req(payload).await,

                // // Devices

                // // Sensors

                // // Device Types

                // // Sensor Values
                // _ => {

                // }
    }

    // tokio::time::delay_for(Duration::from_secs(5)).await;

    // println!("Success!");
}
