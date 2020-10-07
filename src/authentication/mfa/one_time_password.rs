use crate::authentication::mfa::*;

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub grant_type: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub mfa_token: String,
    pub otp: String,
}
