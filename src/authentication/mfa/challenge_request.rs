use crate::mfa::*;

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub mfa_token: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oob_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_id: Option<String>,
}
