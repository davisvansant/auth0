use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    pub email: String,
    pub connection: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    parameters: RequestParameters,
}

impl ChangePasswordRequest {
    pub fn collect(parameters: RequestParameters) -> ChangePasswordRequest {
        ChangePasswordRequest { parameters }
    }
}

pub trait ChangePassword {
    fn change_password(&self, parameters: ChangePasswordRequest) -> RequestBuilder;
}

impl ChangePassword for Api {
    fn change_password(&self, request: ChangePasswordRequest) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/dbconnections/change_password");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&request.parameters)
    }
}
