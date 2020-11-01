mod ip_check;
mod handler;
mod mqtt;
mod router;
mod serve;
mod db;
mod schema;
mod models;

use std::sync::Arc;
use tokio::sync::Mutex;

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
        
    embedded_migrations::run(&conn);
    db::populate(&conn); 

    let client_arc = Arc::new(Mutex::new(client));
    let conn_arc = Arc::new(Mutex::new(conn));

    tokio::join!(
        ip_check::run_loop(),
        mqtt::listen(conn_arc, client_arc, &mut eventloop),
        serve::run_actix()
    );
}
