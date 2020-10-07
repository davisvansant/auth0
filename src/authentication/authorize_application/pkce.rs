use crate::authentication::authorize_application::*;

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    pub response_type: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    pub code_challenge_method: String,
    pub code_challenge: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}
