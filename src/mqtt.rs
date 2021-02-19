use crate::router;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
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
    let pool_clone = pool.clone();

    loop {
        let notification = eventloop.poll().await.unwrap();
        println!("Recv : {:?}", notification);

        if let Event::Incoming(Incoming::Publish(p)) = notification {
            println!("Topic: {:?}, Payload: {:?}", p.topic, p.payload);

            if let Ok(payload_str) = String::from_utf8(p.payload.into_iter().map(|i| i).collect()) {
                let topic_clone = p.topic.clone();
                let client = Arc::clone(&client_arc);
                let conn = Arc::clone(&conn_arc);

                let pool_clone_task = pool_clone.clone();

                task::spawn(async move {
                    router::route(pool_clone_task, conn, client, topic_clone, payload_str).await;
                });
            } else {
                println!("Error decoding bytes");
            }
        }
    }
}

pub async fn send_single(topic: String, payload: String) {
    let mut mqttoptions = MqttOptions::new(uuid::Uuid::new_v4().to_string(), "localhost", 1883);
    mqttoptions.set_keep_alive(5);

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe(topic.as_str(), QoS::AtMostOnce).await;

    client
        .publish(topic.as_str(), QoS::AtLeastOnce, false, payload.as_str())
        .await;
    client.disconnect().await;

    loop {
        match eventloop.poll().await {
            Ok(Event::Outgoing(rumqttc::Outgoing::Disconnect)) => {
                println!("Disconnect {}:{}", topic, payload.as_str());
                break;
            }
            _ => {
                println!("Irrelevant");
            }
        }
    }
}
