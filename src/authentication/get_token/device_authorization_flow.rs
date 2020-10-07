use crate::authentication::get_token::*;

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub grant_type: String,
    pub client_id: String,
    pub device_code: String,
}
