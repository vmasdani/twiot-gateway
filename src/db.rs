use crate::helpermodels::WaterSendBody;
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
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::{
    env,
    time::{Duration, Instant},
};

pub fn init() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let sqlite_connection = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    crate::populate::populate(&sqlite_connection);

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

                        match device_schedules {
                            Ok(device_schedules_res) => {
                                for device_schedule in device_schedules_res {
                                    use schema::devices::dsl::*;
                                    match devices
                                        .filter(id.eq(device_schedule.device_id.unwrap_or(0)))
                                        .first::<Device>(&pool_res)
                                    {
                                        Ok(found_device) => {
                                            tokio::spawn(async move {
                                                println!(
                                                    "Watering {} ({}) for {} secs",
                                                    found_device
                                                        .name
                                                        .unwrap_or("unnamed".to_string()),
                                                    found_device.id.unwrap_or(0),
                                                    schedule_res.watering_secs.unwrap_or(0)
                                                );

                                                let found_device_id = found_device.id.unwrap_or(0);

                                                crate::mqtt::send_single(
                                                    format!("{}/water", found_device_id),
                                                    serde_json::to_string(&WaterSendBody {
                                                        water_on: true,
                                                    })
                                                    .unwrap_or("".to_string()),
                                                )
                                                .await;

                                                for i in 0..schedule_res.watering_secs.unwrap_or(0)
                                                {
                                                    println!(
                                                        "Device ID {}: {} out of {}",
                                                        found_device_id,
                                                        i + 1,
                                                        schedule_res.watering_secs.unwrap_or(0)
                                                    );

                                                    tokio::time::delay_for(Duration::from_secs(1))
                                                        .await;
                                                }

                                                crate::mqtt::send_single(
                                                    format!("{}/water", found_device_id),
                                                    serde_json::to_string(&WaterSendBody {
                                                        water_on: false,
                                                    })
                                                    .unwrap_or("".to_string()),
                                                )
                                                .await;
                                            });
                                        }
                                        Err(e) => println!("{}", e),
                                    }
                                }
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
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }

        tokio::time::delay_for(Duration::from_secs(60)).await;
    }
}
