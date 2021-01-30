use serde::{Deserialize, Serialize};

use crate::models::{Device, Schedule, WateringTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct IdBody {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WateringBody {
    pub watering_type: String, // timed, instant
    pub watering_time: Option<i32>,
    pub switch_on: bool, // on/off
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WateringTimeView {
    pub watering_time: WateringTime,
    pub devices: Vec<Device>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScheduleView {
    pub schedule: Schedule,
    pub devices: Vec<Device>,
}

