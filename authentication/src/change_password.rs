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

pub trait ChangePassword {
    fn change_password(&self, request: RequestParameters) -> RequestBuilder;
}

impl ChangePassword for Api {
    fn change_password(&self, request: RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/dbconnections/change_password");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&request)
    }
}
