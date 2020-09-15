use crate::mfa::*;

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub access_token: String,
}
