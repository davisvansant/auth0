use crate::authentication::Api;
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
        let endpoint = String::from("/mfa/challenge");
        let url = self.base_url.join(&endpoint).unwrap();
        self.client.post(url).json(&request)
    }
    fn verify_with_otp(&self, request: one_time_password::RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();
        self.client.post(url).form(&request)
    }
    fn verify_with_oob(&self, request: out_of_band::RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();
        self.client.post(url).form(&request)
    }
    fn verify_with_recovery_code(
        &self,
        request: recovery_code::RequestParameters,
    ) -> RequestBuilder {
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();
        self.client.post(url).form(&request)
    }
    fn add_authenticator(&self, request: add_authenticator::RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/mfa/associate");
        let url = self.base_url.join(&endpoint).unwrap();
        self.client.post(url).json(&request)
    }
    fn list_authenticators(
        &self,
        request: list_authenticators::RequestParameters,
    ) -> RequestBuilder {
        let endpoint = String::from("/mfa/authenticators");
        let url = self.base_url.join(&endpoint).unwrap();
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", &request.access_token);
        let json_header = String::from("application/json");
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&auth_value).unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_str(&json_header).unwrap(),
        );
        self.client.get(url).headers(headers)
    }
    fn delete_authenticator(
        &self,
        request: delete_authenticator::RequestParameters,
    ) -> RequestBuilder {
        let endpoint = String::from("/mfa/authenticators/");
        let base_url = self.base_url.join(&endpoint).unwrap();
        let url = base_url.join(&request.authenticator_id).unwrap();
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", &request.access_token);
        let json_header = String::from("application/json");
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&auth_value).unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_str(&json_header).unwrap(),
        );
        self.client.delete(url).headers(headers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::*;

    #[test]
    fn challenge_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
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

    #[test]
    fn one_time_password_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let mfa = Api::init(base_url, authentication);
        let parameters = mfa::one_time_password::RequestParameters {
            grant_type: String::from("some_awesome_grant_type"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: None,
            mfa_token: String::from("some_awesome_mfa_token"),
            otp: String::from("some_awesome_otp"),
        };
        let request = mfa.verify_with_otp(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/oauth/token");
        let test_body = String::from(
            "grant_type=some_awesome_grant_type&\
            client_id=some_awesome_client_id&\
            mfa_token=some_awesome_mfa_token&\
            otp=some_awesome_otp",
        );
        assert_eq!(request.method().as_str(), reqwest::Method::POST);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().len(), 1);
        assert_eq!(
            request.body().unwrap().as_bytes().unwrap(),
            test_body.as_bytes(),
        );
    }

    #[test]
    fn out_of_band_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let mfa = Api::init(base_url, authentication);
        let parameters = mfa::out_of_band::RequestParameters {
            grant_type: String::from("some_awesome_grant_type"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: None,
            mfa_token: String::from("some_awesome_mfa_token"),
            oob_code: String::from("some_awesome_oob_code"),
            binding_code: None,
        };
        let request = mfa.verify_with_oob(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/oauth/token");
        let test_body = String::from(
            "grant_type=some_awesome_grant_type&\
            client_id=some_awesome_client_id&\
            mfa_token=some_awesome_mfa_token&\
            oob_code=some_awesome_oob_code",
        );
        assert_eq!(request.method().as_str(), reqwest::Method::POST);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().len(), 1);
        assert_eq!(
            request.body().unwrap().as_bytes().unwrap(),
            test_body.as_bytes(),
        );
    }

    #[test]
    fn recovery_code_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let mfa = Api::init(base_url, authentication);
        let parameters = mfa::recovery_code::RequestParameters {
            grant_type: String::from("some_awesome_grant_type"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: None,
            mfa_token: String::from("some_awesome_mfa_token"),
            recovery_code: String::from("some_awesome_mfa_token"),
        };
        let request = mfa.verify_with_recovery_code(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/oauth/token");
        let test_body = String::from(
            "grant_type=some_awesome_grant_type&\
            client_id=some_awesome_client_id&\
            mfa_token=some_awesome_mfa_token&\
            recovery_code=some_awesome_mfa_token",
        );
        assert_eq!(request.method().as_str(), reqwest::Method::POST);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().len(), 1);
        assert_eq!(
            request.body().unwrap().as_bytes().unwrap(),
            test_body.as_bytes(),
        );
    }

    #[test]
    fn add_authenticator_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let mfa = Api::init(base_url, authentication);
        let parameters = mfa::add_authenticator::RequestParameters {
            client_id: String::from("some_awesome_client_id"),
            client_secret: None,
            authenticator_types: String::from("some_awesome_authenticator_type"),
            oob_channel: None,
            phone_number: None,
        };
        let request = mfa.add_authenticator(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/mfa/associate");
        let test_body = String::from(
            "{\"client_id\":\"some_awesome_client_id\",\
            \"authenticator_types\":\"some_awesome_authenticator_type\"}",
        );
        assert_eq!(request.method().as_str(), reqwest::Method::POST);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().len(), 1);
        assert_eq!(
            request.body().unwrap().as_bytes().unwrap(),
            test_body.as_bytes(),
        );
    }

    #[test]
    fn list_authenticators_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let mfa = Api::init(base_url, authentication);
        let parameters = mfa::list_authenticators::RequestParameters {
            access_token: String::from("some_awesome_access_token"),
        };
        let request = mfa.list_authenticators(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/mfa/authenticators");
        assert_eq!(request.method().as_str(), reqwest::Method::GET);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().len(), 2);
        assert_eq!(request.body().is_none(), true);
    }

    #[test]
    fn delete_authenticator_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let mfa = Api::init(base_url, authentication);
        let parameters = mfa::delete_authenticator::RequestParameters {
            access_token: String::from("some_awesome_access_token"),
            authenticator_id: String::from("some_awesome_authenticator_id"),
        };
        let request = mfa.delete_authenticator(parameters).build().unwrap();
        let test_url =
            String::from("https://your_domain/mfa/authenticators/some_awesome_authenticator_id");
        assert_eq!(request.method().as_str(), reqwest::Method::DELETE);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().len(), 2);
        assert_eq!(request.body().is_none(), true);
    }
}
