use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub plain_pwd: String,
}
