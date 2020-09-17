pub use crate::get_token::*;

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub audience: String,
}
