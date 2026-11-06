use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub enum PersistenceType {
    Create,
    Update,
    Delete,
}
impl Serialize for PersistenceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            PersistenceType::Create => "CREATE",
            PersistenceType::Update => "UPDATE",
            PersistenceType::Delete => "DELETE",
        };
        serializer.serialize_str(value)
    }
}
impl<'de> Deserialize<'de> for PersistenceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "CREATE" => Ok(PersistenceType::Create),
            "UPDATE" => Ok(PersistenceType::Update),
            "DELETE" => Ok(PersistenceType::Delete),
            _ => Err(serde::de::Error::custom(format!(
                "unknown persistence type: {value}"
            ))),
        }
    }
}
