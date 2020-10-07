use crate::authentication::mfa::*;

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub access_token: String,
}
