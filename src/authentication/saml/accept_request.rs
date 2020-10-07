use crate::authentication::saml::*;

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    #[serde(skip_serializing)]
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
}
