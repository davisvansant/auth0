use crate::Api;
use reqwest::RequestBuilder;
pub use serde::{Deserialize, Serialize};

pub mod authorization_code_flow;
pub mod authorization_code_flow_with_pkce;
pub mod implicit_flow;

pub trait AuthorizeApplication {
    fn authorization_code_flow(
        &self,
        request: authorization_code_flow::RequestParameters,
    ) -> RequestBuilder;

    fn authorization_code_flow_with_pkce(
        &self,
        request: authorization_code_flow_with_pkce::RequestParameters,
    ) -> RequestBuilder;

    fn implicit_flow(&self, request: implicit_flow::RequestParameters) -> RequestBuilder;
}

impl AuthorizeApplication for Api {
    fn authorization_code_flow(
        &self,
        request: authorization_code_flow::RequestParameters,
    ) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/authorize");
        let url = self.base_url.join(&endpoint).unwrap();

        client.get(url).query(&request)
    }

    fn authorization_code_flow_with_pkce(
        &self,
        request: authorization_code_flow_with_pkce::RequestParameters,
    ) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/authorize");
        let url = self.base_url.join(&endpoint).unwrap();

        client.get(url).query(&request)
    }

    fn implicit_flow(&self, request: implicit_flow::RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/authorize");
        let url = self.base_url.join(&endpoint).unwrap();

        client.get(url).query(&request)
    }
}
