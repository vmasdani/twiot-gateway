use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IdBody {
    pub id: i32
}