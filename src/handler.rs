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
        Err(_) => { println!("Error gettings schedules!") }
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
