use serde::{Deserialize, Serialize};

use crate::models::{Device, DeviceSchedule, Schedule, WateringTime};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScheduleView {
    pub schedule: Option<Schedule>,
    pub device_schedule_views: Vec<DeviceScheduleView>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceScheduleView {
    pub device_schedule: Option<DeviceSchedule>,
    pub device: Option<Device>,
    pub schedule: Option<Schedule>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SchedulePostBody {
    pub schedule_views: Vec<ScheduleView>,
    pub schedule_delete_ids: Vec<i32>,
    pub device_schedule_delete_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WaterSendBody {
    pub water_on: bool,
}