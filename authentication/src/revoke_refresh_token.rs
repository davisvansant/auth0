use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub token: String,
}

pub trait RevokeRequestToken {
    fn revoke_refresh_token(&self, parameters: RequestParameters) -> RequestBuilder;
}

impl RevokeRequestToken for Api {
    fn revoke_refresh_token(&self, request: RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/revoke");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).json(&request)
    }
}
