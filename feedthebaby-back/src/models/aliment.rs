use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AlimentInDTO {
    pub name: String,
    pub quantity: i32,
    pub expire_at: String,
    pub aliment_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct AlimentQuantityInDTO {
    pub quantity: i32,
}

#[derive(Serialize, Deserialize)]
pub struct AlimentOutDTO {
    pub id: i64,
    pub name: String,
    pub quantity: i32,
    pub expire_at: String,
    pub aliment_type: String,
}
