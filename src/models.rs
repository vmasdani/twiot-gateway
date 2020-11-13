use serde::{Deserialize, Serialize};
use crate::schema::*; 

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

#[derive(Identifiable, Associations, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(DeviceType)]
pub struct Device {
    pub id: Option<i32>,
    pub name: String,
    pub serial_number: String,
    pub device_type_id: i32
}

#[derive(Identifiable, Insertable, Associations, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Device)]
#[belongs_to(SensorType)]
pub struct Sensor {
    pub id: Option<i32>,
    pub name: String,
    pub serial_number: String,
    pub sensor_type_id: i32,
    pub device_id: i32,
}

#[derive(Identifiable, Insertable, Associations, Queryable, Serialize, Deserialize, Debug)]
pub struct SensorType {
    pub id: Option<i32>,
    pub name: String
}

#[derive(Identifiable, Insertable, Associations, Queryable, Serialize, Deserialize, Debug)]
pub struct DeviceType {
    pub id: Option<i32>,
    pub name: String
}

#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Sensor)]
pub struct SensorValue {
    pub id: Option<i32>,
    pub sense_value: String,
    pub max: i32,
    pub min: i32,
    pub sensor_id: i32
}