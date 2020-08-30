use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub client_id: String,
    pub email: String,
    pub password: String,
    pub connection: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SignupRequest {
    parameters: RequestParameters,
}

impl SignupRequest {
    pub fn collect(parameters: RequestParameters) -> SignupRequest {
        SignupRequest { parameters }
    }
}

pub trait Signup {
    fn signup(&self, parameters: SignupRequest) -> RequestBuilder;
}

impl Signup for Api {
    fn signup(&self, request: SignupRequest) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/dbconnections/signup");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&request.parameters)
    }
}
