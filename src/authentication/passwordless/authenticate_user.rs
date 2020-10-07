use crate::authentication::passwordless::*;

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub realm: String,
    pub otp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
