use crate::authentication::get_token::*;

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub grant_type: String,
    pub subject_token: String,
    pub subject_token_type: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing)]
    pub auth0_forwarded_for: Option<String>,
}
