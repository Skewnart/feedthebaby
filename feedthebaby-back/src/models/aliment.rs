use crate::models::enums::persistence_type::PersistenceType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AlimentInDTO {
    pub id: i64,
    pub name: String,
    pub quantity: i32,
    pub expire_at: String,    // , date
    pub aliment_type: String, // 10g, 20g, 25g, enum
    pub persistence_type: PersistenceType,
}

#[derive(Serialize)]
pub struct AlimentOutNewIdDTO {
    pub old_id: i64,
    pub new_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct AlimentOutDTO {
    pub id: i64,
    pub name: String,
    pub quantity: i32,
    pub expire_at: String,
    pub aliment_type: String,
}
