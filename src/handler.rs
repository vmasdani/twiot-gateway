use rumqttc::{AsyncClient};
use diesel::prelude::*; 
use std::sync::Arc;
use tokio::sync::Mutex;
use diesel::SqliteConnection;
use crate::models::*;
use crate::helpermodels::*;

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
    use crate::schema::schedules::dsl::*;
}

pub async fn schedule_delete(conn_arc: Arc<Mutex<SqliteConnection>>, client: Arc<Mutex<AsyncClient>>, payload: String) {
    use crate::schema::schedules::dsl::*;
    let idBody: Result<IdBody, serde_json::Error> = serde_json::from_str(payload.as_str());
    
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
