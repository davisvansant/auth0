use crate::authentication::Api;
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
        let endpoint = String::from("/samlp/");
        let base_url = self.base_url.join(&endpoint).unwrap();
        let url = base_url.join(&request.client_id).unwrap();

        self.client.get(url).form(&request)
    }

    fn get_metadata(&self, request: get_metadata::RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/samlp/metadata/");
        let base_url = self.base_url.join(&endpoint).unwrap();
        let url = base_url.join(&request.client_id).unwrap();

        self.client.get(url)
    }

    fn idp_flow(&self, request: identity_provider::RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/login/callback");
        let url = self.base_url.join(&endpoint).unwrap();

        self.client.post(url).form(&request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::*;

    #[test]
    fn accept_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
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

    #[test]
    fn get_metadata_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let saml = Api::init(base_url, authentication);
        let parameters = saml::get_metadata::RequestParameters {
            client_id: String::from("some_awesome_client_id"),
        };
        let request = saml.get_metadata(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/samlp/metadata/some_awesome_client_id");
        assert_eq!(request.method().as_str(), reqwest::Method::GET);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().len(), 0);
        assert_eq!(request.body().is_none(), true);
    }

    #[test]
    fn idp_flow_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let saml = Api::init(base_url, authentication);
        let parameters = saml::identity_provider::RequestParameters {
            connection: String::from("some_awesome_connection"),
            saml_response: String::from("some_awesome_saml_response"),
        };
        let request = saml.idp_flow(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/login/callback");
        let test_body = String::from(
            "connection=some_awesome_connection&\
            SAMLResponse=some_awesome_saml_response",
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
