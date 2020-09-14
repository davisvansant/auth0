use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    #[serde(rename(serialize = "returnTo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federated: Option<String>,
}

pub trait Logout {
    fn logout(&self, request: RequestParameters) -> RequestBuilder;
}

impl Logout for Api {
    fn logout(&self, request: RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/v2/logout");
        let url = self.base_url.join(&endpoint).unwrap();
        client.get(url).query(&request)
    }
}
