use std::time::Duration;
use rumqttc::{AsyncClient};
use std::sync::Arc;
use tokio::sync::Mutex;
use diesel::SqliteConnection;
use crate::handler::*;

pub async fn route(conn_arc: Arc<Mutex<SqliteConnection>>, client_arc: Arc<Mutex<AsyncClient>>, route: String, payload: String) {
    println!("Paylaod: {}", payload);

    match route.as_str() {
        "schedules/req" => schedule_req(conn_arc, client_arc).await,
        "schedules/req/add" => schedule_req_add(conn_arc, client_arc, payload).await,
        "schedules/req/delete" => schedule_delete(conn_arc, client_arc, payload).await,
        "schedules/res" => schedule_res(payload).await,

        "watering_times/req" => {
        
        }
        "watering_times/res" => {
        
        }
        
        "watering/req" => {
        
        }
        _ => {
        
        }
    }
    
    tokio::time::delay_for(Duration::from_secs(5)).await;
    
    println!("Success!");
}