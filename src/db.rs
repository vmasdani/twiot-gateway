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
    use crate::schema::watering_times::dsl::*;
    use crate::schema::schedules::dsl::*;
    
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
}