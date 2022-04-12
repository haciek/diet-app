use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)] #[allow(unused)]
pub struct Data {
    pub id: u32,
    pub date: String,
    pub weight: f32,
}

