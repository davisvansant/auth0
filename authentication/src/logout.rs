use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub return_to: String,
    pub client_id: String,
    pub federated: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogoutRequest {
    parameters: RequestParameters,
}

impl LogoutRequest {
    pub fn collect(parameters: RequestParameters) -> LogoutRequest {
        LogoutRequest { parameters }
    }
}

pub trait Logout {
    fn logout(&self, parameters: LogoutRequest) -> RequestBuilder;
}

impl Logout for Api {
    fn logout(&self, logout_request: LogoutRequest) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/v2/logout");
        let url = self.base_url.join(&endpoint).unwrap();
        let json = serde_json::to_value(logout_request.parameters).unwrap();
        let mut parameters = BTreeMap::new();
        let map = json.as_object().unwrap();
        for (k, v) in map.iter() {
            parameters.insert(k.as_str(), v.as_str().unwrap());
        }
        client.get(url).query(&parameters)
    }
}
