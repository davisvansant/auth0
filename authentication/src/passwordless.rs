use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

pub mod authenticate_user;
pub mod get_code_or_link;

pub trait Passwordless {
    fn passwordless_start(&self, request: get_code_or_link::RequestParameters) -> RequestBuilder;
    fn passwordless_login(&self, request: authenticate_user::RequestParameters) -> RequestBuilder;
}

impl Passwordless for Api {
    fn passwordless_start(&self, request: get_code_or_link::RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/passwordless/start");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&request)
    }

    fn passwordless_login(&self, request: authenticate_user::RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oath/token");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&request)
    }
}
