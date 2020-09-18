use crate::Api;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::RequestBuilder;
pub use serde::{Deserialize, Serialize};

pub mod add_authenticator;
pub mod challenge_request;
pub mod delete_authenticator;
pub mod list_authenticators;
pub mod one_time_password;
pub mod out_of_band;
pub mod recovery_code;

pub trait MultiFactorAuthentication {
    fn challenge_request(&self, request: challenge_request::RequestParameters) -> RequestBuilder;
    fn verify_with_otp(&self, request: one_time_password::RequestParameters) -> RequestBuilder;
    fn verify_with_oob(&self, request: out_of_band::RequestParameters) -> RequestBuilder;
    fn verify_with_recovery_code(
        &self,
        request: recovery_code::RequestParameters,
    ) -> RequestBuilder;
    fn add_authenticator(&self, request: add_authenticator::RequestParameters) -> RequestBuilder;
    fn list_authenticators(
        &self,
        request: list_authenticators::RequestParameters,
    ) -> RequestBuilder;
    fn delete_authenticator(
        &self,
        request: delete_authenticator::RequestParameters,
    ) -> RequestBuilder;
}

impl MultiFactorAuthentication for Api {
    fn challenge_request(&self, request: challenge_request::RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/mfa/challenge");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&request)
    }
    fn verify_with_otp(&self, request: one_time_password::RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&request)
    }
    fn verify_with_oob(&self, request: out_of_band::RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&request)
    }
    fn verify_with_recovery_code(
        &self,
        request: recovery_code::RequestParameters,
    ) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&request)
    }
    fn add_authenticator(&self, request: add_authenticator::RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/mfa/associate");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&request)
    }
    fn list_authenticators(
        &self,
        request: list_authenticators::RequestParameters,
    ) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/mfa/authenticators");
        let url = self.base_url.join(&endpoint).unwrap();
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", &request.access_token);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&auth_value).unwrap(),
        );
        client.get(url).headers(headers)
    }
    fn delete_authenticator(
        &self,
        request: delete_authenticator::RequestParameters,
    ) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/mfa/authenticators");
        let base_url = self.base_url.join(&endpoint).unwrap();
        let url = base_url.join(&request.authenticator_id).unwrap();
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", &request.access_token);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&auth_value).unwrap(),
        );
        client.delete(url).headers(headers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn challenge_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
        let mfa = Api::init(base_url, authentication);
        let parameters = mfa::challenge_request::RequestParameters {
            mfa_token: String::from("some_awesome_mfa_token"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: None,
            challenge_type: None,
            oob_channel: None,
            authenticator_id: None,
        };
        let request = mfa.challenge_request(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/mfa/challenge");
        let test_body = String::from(
            "{\"mfa_token\":\"some_awesome_mfa_token\",\
            \"client_id\":\"some_awesome_client_id\"}",
        );
        assert_eq!(request.method().as_str(), reqwest::Method::POST);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().len(), 1);
        assert_eq!(
            request.body().unwrap().as_bytes().unwrap(),
            test_body.as_bytes(),
        );
    }
}
