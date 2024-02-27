use crate::authentication::passwordless::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestParameters {
    pub client_id: String,
    pub client_secret: String,
    pub connection: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send: Option<String>,
    #[serde(rename(serialize = "authParams"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_params: Option<String>,
}
