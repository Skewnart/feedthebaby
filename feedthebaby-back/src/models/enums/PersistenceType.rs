use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub enum PersistenceType {
    CREATE,
    UPDATE,
    DELETE
}
impl Serialize for PersistenceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let value = match self {
            PersistenceType::CREATE => "CREATE",
            PersistenceType::UPDATE => "UPDATE",
            PersistenceType::DELETE => "DELETE",
        };
        serializer.serialize_str(value)
    }
}
impl<'de> Deserialize<'de> for PersistenceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "CREATE" => Ok(PersistenceType::CREATE),
            "UPDATE" => Ok(PersistenceType::UPDATE),
            "DELETE" => Ok(PersistenceType::DELETE),
            _ => Err(serde::de::Error::custom(format!(
                "unknown persistence type: {value}"
            ))),
        }
    }
}