use crate::Api;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserProfileRequest {
    parameters: RequestParameters,
}

impl UserProfileRequest {
    pub fn collect(parameters: RequestParameters) -> UserProfileRequest {
        UserProfileRequest { parameters }
    }
}

pub trait UserInfo {
    fn userinfo(&self, parameters: UserProfileRequest) -> RequestBuilder;
}

impl UserInfo for Api {
    fn userinfo(&self, login_request: UserProfileRequest) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/userinfo");
        let url = self.base_url.join(&endpoint).unwrap();
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", &login_request.parameters.access_token);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&auth_value).unwrap(),
        );
        client.get(url).headers(headers)
    }
}
