use crate::authentication::mfa::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestParameters {
    pub access_token: String,
}
