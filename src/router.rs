use std::time::Duration;
use rumqttc::{AsyncClient};
use std::sync::Arc;
use tokio::sync::Mutex;
use diesel::SqliteConnection;
use crate::handler::*;

pub async fn route(conn_arc: Arc<Mutex<SqliteConnection>>, client_arc: Arc<Mutex<AsyncClient>>, route: String, payload: String) {
    println!("Paylaod: {}", payload);

    match route.as_str() {
        // Schedules
        "schedules/req" => schedule_req(conn_arc, client_arc).await,
        "schedules/req/save" => schedule_req_save(conn_arc, payload).await,
        "schedules/req/delete" => schedule_delete(conn_arc, payload).await,
        "schedules/res" => schedule_res(payload).await,
        
        // Watering times
        "watering_times/req" => watering_time_req(conn_arc, client_arc).await,
        "watering_times/req/save" => {
            
        },
        "watering_times/req/delete" => {
            
        },
        "watering_times/res" => watering_time_res(payload).await,

        // Watering (on/off)
        "watering/req" => watering_req(payload).await,

        // Devices

        // Sensors

        // Device Types

        // Sensor Values
        _ => {
        
        }
    }
    
    // tokio::time::delay_for(Duration::from_secs(5)).await;
    
    // println!("Success!");
}