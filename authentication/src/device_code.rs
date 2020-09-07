use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

pub struct RequestParamaters {
    pub audience: Option<String>,
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
    fn device_authorization_flow(&self, request: RequestParamaters) -> RequestBuilder;
}

impl GetDeviceCode for Api {
    fn device_authorization_flow(&self, request: RequestParamaters) -> RequestBuilder {
        #[derive(Serialize, Deserialize)]
        struct QueryParameters {
            #[serde(skip_serializing_if = "Option::is_none")]
            audience: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            scope: Option<String>,
            client_id: String,
        }

        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/device/code");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).form(&QueryParameters {
            audience: request.audience,
            scope: request.scope,
            client_id: request.client_id,
        })
    }
}
