use crate::models::*;
use actix_rt::time::delay_for;
use chrono::{DateTime, Local, Timelike, Utc};
use diesel::sqlite::SqliteConnection;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenv::dotenv;
use std::{
    env,
    time::{Duration, Instant},
};

pub fn init() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let sqlite_connection = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    sqlite_connection
}

pub async fn poller(pool: Pool<ConnectionManager<SqliteConnection>>) {
    loop {
        println!(
            "[Checking!] {:?}, {}:{}",
            Local::now(),
            Local::now().hour(),
            Local::now().minute()
        );
        tokio::time::delay_for(Duration::from_secs(60)).await;
    }
}
