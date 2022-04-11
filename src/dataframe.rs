use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub id: u32,
    pub date: String,
    pub weight: f32,
}

