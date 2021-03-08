use diesel::SqliteConnection;

use diesel::prelude::*;

use crate::models::{DeviceType, Schedule, WateringTime};

pub fn populate(connection: &SqliteConnection) {
    // Check watering time

    use crate::schema::watering_times::dsl::watering_times;
    match watering_times.load::<WateringTime>(connection) {
        Ok(watering_times_result) => {
            println!("[populate] Showing {} watering times.", watering_times_result.len());

            watering_times_result.iter().for_each(|watering_time| {
                println!("[populate] Watering time: {:?}", watering_time);
            });

            if watering_times_result.len() == 0 {
                println!("[populate] No watering time detected! Creating...");

                let new_watering_time = WateringTime {
                    id: None,
                    time: Some(30),
                    created_at: None,
                    updated_at: None,
                };

                diesel::insert_into(watering_times)
                    .values(&new_watering_time)
                    .execute(connection);
            }
        }
        Err(e) => println!("[populate] {:?}", e),
    }

    // Check schedule
    use crate::schema::schedules::dsl::schedules;
    match schedules.load::<Schedule>(connection) {
        Ok(schedules_result) => {
            println!("[populate] Showing {} schedules.", schedules_result.len());

            schedules_result.iter().for_each(|schedule| {
                println!("[populate] Schedule: {:?}", schedule);
            });

            if schedules_result.len() == 0 {
                println!("[populate] No schedules found. Creating...");

                let new_schedule = Schedule {
                    id: None,
                    hour: Some(12),
                    minute: Some(0),
                    created_at: None,
                    updated_at: None,
                    watering_secs: Some(10),
                };

                diesel::insert_into(schedules)
                    .values(&new_schedule)
                    .execute(connection);
            }
        }
        Err(e) => {
            println!("[populate] {:?}", e);
        }
    }

    // // Insert one schedule
    // let new_schedule = Schedule {
    //     id: None,
    //     hour: Some(23),
    //     minute: Some(29),
    //     created_at: None,
    //     updated_at: None,
    // };

    // diesel::insert_into(schedules)
    //     .values(&new_schedule)
    //     .execute(connection);

    // Check device types

    vec!["Node", "Gateway"].into_iter().for_each(|device_name| {
        use crate::schema::device_types::dsl::*;

        match device_types
            .filter(name.eq(device_name))
            .first::<DeviceType>(connection)
        {
            Ok(_) => {
                println!("[populate] Device type {} found!", device_name);
            }
            Err(_) => {
                println!("[populate] Device type {} not found! Creating...", device_name);

                let new_device_type = DeviceType {
                    id: None,
                    name: Some(device_name.to_string()),
                    created_at: None,
                    updated_at: None,
                };

                diesel::replace_into(device_types)
                    .values(&new_device_type)
                    .execute(connection);
            }
        }
    });
}
