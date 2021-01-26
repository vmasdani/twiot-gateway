use crate::router;
use diesel::{SqliteConnection, r2d2::{ConnectionManager, Pool}};
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};
use std::sync::Arc;
use tokio::sync::Mutex;

use tokio::task;

pub async fn init() -> (AsyncClient, EventLoop) {
    let (client, eventloop) =
        AsyncClient::new(MqttOptions::new("twiot-sub-client", "localhost", 1883), 10);

    client.subscribe("#", QoS::AtMostOnce).await.unwrap();

    (client, eventloop)
}

pub async fn listen(
    pool: Pool<ConnectionManager<SqliteConnection>>,
    conn_arc: Arc<Mutex<SqliteConnection>>,
    client_arc: Arc<Mutex<AsyncClient>>,
    eventloop: &mut EventLoop,
) {
    loop {
        let notification = eventloop.poll().await.unwrap();
        println!("Recv : {:?}", notification);

        if let Event::Incoming(Incoming::Publish(p)) = notification {
            println!("Topic: {:?}, Payload: {:?}", p.topic, p.payload);
            if let Ok(payload_str) = String::from_utf8(p.payload.into_iter().map(|i| i).collect()) {
                let topic_clone = p.topic.clone();
                let client = Arc::clone(&client_arc);
                let conn = Arc::clone(&conn_arc);

                task::spawn(async move {
                    router::route(conn, client, topic_clone, payload_str).await;
                });
            } else {
                println!("Error decoding bytes");
            }
        }
    }
}
