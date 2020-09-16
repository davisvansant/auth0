use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    pub client_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseValues {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: String,
    pub interval: String,
}

pub trait GetDeviceCode {
    fn device_authorization_flow(&self, request: RequestParameters) -> RequestBuilder;
}

impl GetDeviceCode for Api {
    fn device_authorization_flow(&self, request: RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/device/code");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).form(&request)
    }
}
