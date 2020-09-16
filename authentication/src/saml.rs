use crate::Api;
use reqwest::RequestBuilder;
pub use serde::{Deserialize, Serialize};

pub mod accept_request;
pub mod get_metadata;
pub mod identity_provider;

pub trait SAML {
    fn accept_request(&self, request: accept_request::RequestParameters) -> RequestBuilder;
    fn get_metadata(&self, request: get_metadata::RequestParameters) -> RequestBuilder;
    fn idp_flow(&self, request: identity_provider::RequestParameters) -> RequestBuilder;
}

impl SAML for Api {
    fn accept_request(&self, request: accept_request::RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/samlp/");
        let base_url = self.base_url.join(&endpoint).unwrap();
        let url = base_url.join(&request.client_id).unwrap();

        client.get(url).query(&request.connection)
    }

    fn get_metadata(&self, request: get_metadata::RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/samlp/metadata/");
        let base_url = self.base_url.join(&endpoint).unwrap();
        let url = base_url.join(&request.client_id).unwrap();

        client.get(url)
    }

    fn idp_flow(&self, request: identity_provider::RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/login/callback");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).query(&request)
    }
}
