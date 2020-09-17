use crate::Api;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::RequestBuilder;
pub use serde::{Deserialize, Serialize};

pub mod authorization_code_flow;
pub mod authorization_code_flow_with_pkce;
pub mod client_credentials_flow;
pub mod device_authorization_flow;
pub mod refresh_token;
pub mod resource_owner_password;
pub mod token_exchange_for_native_social;

pub trait GetToken {
    fn authorization_code_flow(
        &self,
        request: authorization_code_flow::RequestParameters,
    ) -> RequestBuilder;

    fn authorization_code_flow_with_pkce(
        &self,
        request: authorization_code_flow_with_pkce::RequestParameters,
    ) -> RequestBuilder;

    fn client_credentials_flow(
        &self,
        request: client_credentials_flow::RequestParameters,
    ) -> RequestBuilder;

    fn resource_owner_password(
        &self,
        request: resource_owner_password::RequestParameters,
    ) -> RequestBuilder;

    fn device_authorization_flow(
        &self,
        request: device_authorization_flow::RequestParameters,
    ) -> RequestBuilder;

    fn refresh_token(&self, request: refresh_token::RequestParameters) -> RequestBuilder;

    fn token_exchange_for_native_social(
        &self,
        request: token_exchange_for_native_social::RequestParameters,
    ) -> RequestBuilder;
}

impl GetToken for Api {
    fn authorization_code_flow(
        &self,
        request: authorization_code_flow::RequestParameters,
    ) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).form(&request)
    }

    fn authorization_code_flow_with_pkce(
        &self,
        request: authorization_code_flow_with_pkce::RequestParameters,
    ) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).form(&request)
    }

    fn client_credentials_flow(
        &self,
        request: client_credentials_flow::RequestParameters,
    ) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).form(&request)
    }

    fn resource_owner_password(
        &self,
        request: resource_owner_password::RequestParameters,
    ) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        if request.auth0_forwarded_for.is_some() {
            let mut headers = HeaderMap::new();
            let header_key = String::from("auth0-forwarded-for");
            let header_value = &request.auth0_forwarded_for.as_ref().unwrap();
            headers.insert(
                HeaderName::from_bytes(header_key.as_bytes()).unwrap(),
                HeaderValue::from_bytes(header_value.as_bytes()).unwrap(),
            );
            client.post(url).headers(headers).form(&request)
        } else {
            client.post(url).form(&request)
        }
    }

    fn device_authorization_flow(
        &self,
        request: device_authorization_flow::RequestParameters,
    ) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).form(&request)
    }

    fn refresh_token(&self, request: refresh_token::RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).form(&request)
    }

    fn token_exchange_for_native_social(
        &self,
        request: token_exchange_for_native_social::RequestParameters,
    ) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        if request.auth0_forwarded_for.is_some() {
            let mut headers = HeaderMap::new();
            let header_key = String::from("auth0-forwarded-for");
            let header_value = &request.auth0_forwarded_for.as_ref().unwrap();
            headers.insert(
                HeaderName::from_bytes(header_key.as_bytes()).unwrap(),
                HeaderValue::from_bytes(header_value.as_bytes()).unwrap(),
            );
            client.post(url).headers(headers).form(&request)
        } else {
            client.post(url).form(&request)
        }
    }
}
