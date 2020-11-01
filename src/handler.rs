use rumqttc::{AsyncClient};
use diesel::prelude::*; 
use std::sync::Arc;
use tokio::sync::Mutex;
use diesel::SqliteConnection;
use crate::models::*;

pub async fn schedule_req(conn_arc: Arc<Mutex<SqliteConnection>>, client_arc: Arc<Mutex<AsyncClient>>) {
    use crate::schema::schedules::dsl::*;
    
    let conn = conn_arc.lock().await;
    
    if let Ok(schedules_list) = schedules.load::<Schedule>(&*conn) {
        schedules_list.iter().for_each(|schedule| {
            println!("Schedule: {:?}", schedule);
        });
    } else {
        println!("Error loading schedule");
    }

    let client = client_arc.lock().await;
    
}

pub async fn schedule_req_add(conn_arc: Arc<Mutex<SqliteConnection>>, client: Arc<Mutex<AsyncClient>>, payload: String) {

}

pub async fn schedule_res(payload: String) {
    println!("Schedule res received. {}", payload);  
}
