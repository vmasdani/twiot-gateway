use serde::{Deserialize, Serialize};
use crate::schema::{schedules, watering_times}; 

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug)]
pub struct Schedule {
    pub id: Option<i32>,
    pub hour: i32,
    pub minute: i32
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug)]
pub struct WateringTime {
    pub id: Option<i32>,
    pub time: i32
}