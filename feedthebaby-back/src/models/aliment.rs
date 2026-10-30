use serde::{Deserialize, Serialize};
use crate::models::enums::PersistenceType::PersistenceType;

#[derive(Serialize, Deserialize)]
pub struct AlimentInDTO {
    pub id: i64,
    pub name: String,
    pub quantity: i32,
    pub expire_at: String,              // , date
    pub aliment_type: String,           // 10g, 20g, 25g, enum
    pub persistence_type: PersistenceType
}

#[derive(Serialize, Deserialize)]
pub struct AlimentOutDTO {
    pub id: i64,
    pub name: String,
    pub quantity: i32,
    pub expire_at: String,
    pub aliment_type: String,
}
