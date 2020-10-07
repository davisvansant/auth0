use crate::authentication::saml::*;

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub connection: String,
    // #[serde(rename(serialize = "SAMLResponse"))]
    #[serde(rename = "SAMLResponse")]
    pub saml_response: String,
}
