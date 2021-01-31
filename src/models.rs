use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Insertable, Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    pub id: Option<i32>,
    pub hour: Option<i32>,
    pub minute: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Identifiable, Insertable, Queryable, Serialize, Deserialize, Debug)]
pub struct WateringTime {
    pub id: Option<i32>,
    pub time: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(
    Identifiable, Associations, Insertable, Queryable, Serialize, Deserialize, Debug, Clone,
)]
#[belongs_to(DeviceType)]
pub struct Device {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub serial_number: Option<String>,
    pub device_type_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub mac: Option<String>,
    pub ip: Option<String>,
    pub show_in_dashboard: Option<i32>,
}

#[derive(Identifiable, Insertable, Associations, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Device)]
#[belongs_to(SensorType)]
pub struct Sensor {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub serial_number: Option<String>,
    pub sensor_type_id: Option<i32>,
    pub device_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Identifiable, Insertable, Associations, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Device)]
#[belongs_to(WateringTime)]
pub struct DeviceWateringTime {
    pub id: Option<i32>,
    pub watering_time_id: Option<i32>,
    pub device_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(
    Identifiable, Insertable, Associations, Queryable, Serialize, Deserialize, Debug, Clone, Copy,
)]
#[belongs_to(Device)]
#[belongs_to(Schedule)]
pub struct DeviceSchedule {
    pub id: Option<i32>,
    pub schedule_id: Option<i32>,
    pub device_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Identifiable, Insertable, Associations, Queryable, Serialize, Deserialize, Debug)]
pub struct SensorType {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Identifiable, Insertable, Associations, Queryable, Serialize, Deserialize, Debug)]
pub struct DeviceType {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Sensor)]
pub struct SensorValue {
    pub id: Option<i32>,
    pub sense_value: Option<String>,
    pub max: Option<i32>,
    pub min: Option<i32>,
    pub sensor_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
