use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    pub redirect_uris: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_method: Option<String>,
}

pub trait DynamicClientRegistration {
    fn register(&self, parameters: RequestParameters) -> RequestBuilder;
}

impl DynamicClientRegistration for Api {
    fn register(&self, request: RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oidc/register");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).json(&request)
    }
}
