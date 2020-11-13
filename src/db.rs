use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use crate::models::*;
use std::env;

pub fn init() -> SqliteConnection {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let sqlite_connection = SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));

    sqlite_connection
}

pub fn populate(connection: &SqliteConnection) {
    // Check watering time
    
    use crate::schema::watering_times::dsl::{watering_times};
    if let Ok(watering_times_result) = watering_times.load::<WateringTime>(connection) {
        println!("Showing {} watering times.", watering_times_result.len());

        watering_times_result.iter().for_each(|watering_time| {
            println!("Watering time: {:?}", watering_time);
        });

        if watering_times_result.len() == 0 {
            println!("No watering time detected! Creating...");

            let new_watering_time = WateringTime {
                id: None,
                time: 30
            };

            diesel::insert_into(watering_times)
                .values(&new_watering_time)
                .execute(connection);
        }
    }

    // Check schedule
    use crate::schema::schedules::dsl::{schedules};
    if let Ok(schedules_result) = schedules.load::<Schedule>(connection) {
        println!("Showing {} schedules.", schedules_result.len());

        schedules_result.iter().for_each(|schedule| {
            println!("Schedule: {:?}", schedule);
        });

        if schedules_result.len() == 0 {
            println!("No schedules found. Creating...");

            let new_schedule = Schedule {
                id: None,
                hour: 12,
                minute: 0
            };

            diesel::insert_into(schedules).values(&new_schedule).execute(connection);
        }
    }

    // Check device types
    
    vec!["Node", "Gateway"].into_iter().for_each(|device_name| {
        use crate::schema::device_types::dsl::{device_types, name};
        use diesel::result::Error;

        match device_types.filter(name.eq(String::from(device_name))).first(connection) as Result<DeviceType, _> {
            Ok(_) => {
                println!("Device type {} found!", device_name);
            }
            Err(_) => {
                println!("Device type {} not found! Creating...", device_name);

                let new_device_type = DeviceType {
                    id: None,
                    name: String::from(device_name)
                };

                diesel::replace_into(device_types).values(&new_device_type).execute(connection);
            }
        }
    });
}