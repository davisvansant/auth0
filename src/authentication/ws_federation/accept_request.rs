use crate::authentication::ws_federation::*;

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    #[serde(skip_serializing)]
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wtrealm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wctx: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wreply: Option<String>,
}
