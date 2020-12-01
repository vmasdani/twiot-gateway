use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IdBody {
    pub id: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WateringBody {
    pub watering_type: String, // timed, instant
    pub watering_time: Option<i32>,
    pub switch_on: bool // on/off
}