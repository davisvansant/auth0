use crate::Api;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub access_token: String,
}

pub trait UserInfo {
    fn user_info(&self, request: RequestParameters) -> RequestBuilder;
}

impl UserInfo for Api {
    fn user_info(&self, request: RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/userinfo");
        let url = self.base_url.join(&endpoint).unwrap();
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", &request.access_token);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&auth_value).unwrap(),
        );
        client.get(url).headers(headers)
    }
}
