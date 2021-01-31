mod db;
mod handler;
mod helpermodels;
mod ip_check;
mod models;
mod mqtt;
mod router;
mod schema;
mod serve;
mod populate;

use diesel::{
    r2d2::{self, ConnectionManager},
    SqliteConnection,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

embed_migrations!();

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let (client, mut eventloop) = mqtt::init().await;
    let conn = db::init();
    let manager = ConnectionManager::<SqliteConnection>::new("twiot-gateway.sqlite3");
    let pool = r2d2::Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create db pool.");

    embedded_migrations::run(&conn).expect("Error running migrations.");
    populate::populate(&conn);

    let client_arc = Arc::new(Mutex::new(client));
    let conn_arc = Arc::new(Mutex::new(conn));

    let mqtt_pool_clone = pool.clone();
    let pool_clone = pool.clone();

    tokio::join!(
        ip_check::run_loop(),
        mqtt::listen(mqtt_pool_clone, conn_arc, client_arc, &mut eventloop),
        serve::run_actix(pool_clone)
    );
}
