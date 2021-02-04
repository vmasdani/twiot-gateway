use crate::models::*;
use crate::schema;
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
        let cur_hour = Local::now().hour();
        let cur_min = Local::now().minute();

        println!("[Checking!] {:?}, {}:{}", Local::now(), cur_hour, cur_min);

        use schema::schedules::dsl::*;

        match pool.get() {
            Ok(pool_res) => {
                let found_schedule = schedules
                    .filter(hour.eq(cur_hour as i32))
                    .filter(minute.eq(cur_min as i32))
                    .first::<Schedule>(&pool_res);

                println!("Found schedule:");
                println!("{:?}", found_schedule);

                match found_schedule {
                    Ok(schedule_res) => {
                        let device_schedules = DeviceSchedule::belonging_to(&schedule_res)
                            .load::<DeviceSchedule>(&pool_res);

                        println!("Device schedule:");
                        println!("{:?}", device_schedules);
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }

        tokio::time::delay_for(Duration::from_secs(60)).await;
    }
}
