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

        client.get(url).form(&request)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn accept_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
        let saml = Api::init(base_url, authentication);
        let parameters = saml::accept_request::RequestParameters {
            client_id: String::from("some_awesome_client_id"),
            connection: Some(String::from("some_awesome_connection")),
        };
        let request = saml.accept_request(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/samlp/some_awesome_client_id");
        let test_body = String::from("connection=some_awesome_connection");
        assert_eq!(request.method().as_str(), reqwest::Method::GET);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().len(), 1);
        assert_eq!(
            request.body().unwrap().as_bytes().unwrap(),
            test_body.as_bytes(),
        );
    }
}
