use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct ErrorResponse {
    pub err_msg: String
}