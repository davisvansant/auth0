use crate::login::*;

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub response_type: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
    pub redirect_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
