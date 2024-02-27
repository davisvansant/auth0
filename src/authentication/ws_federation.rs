use crate::authentication::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

pub mod accept_request;

pub trait WSFederation {
    fn accept_request(&self, request: accept_request::RequestParameters) -> RequestBuilder;
    fn get_metadata(&self) -> RequestBuilder;
}

impl WSFederation for Api {
    fn accept_request(&self, request: accept_request::RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/wsfed/");
        let base_url = self.base_url.join(&endpoint).unwrap();
        let url = base_url.join(&request.client_id).unwrap();

        self.client.get(url).query(&request)
    }

    fn get_metadata(&self) -> RequestBuilder {
        let endpoint = String::from("/wsfed/FederationMetadata/2007-06/FederationMetadata.xml");
        let url = self.base_url.join(&endpoint).unwrap();
        self.client.get(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::*;

    #[test]
    fn accept_request_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let ws_federation = Api::init(base_url, authentication);
        let parameters = ws_federation::accept_request::RequestParameters {
            client_id: String::from("some_awesome_client_id"),
            wtrealm: None,
            whr: None,
            wctx: None,
            wreply: None,
        };
        let request = ws_federation.accept_request(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/wsfed/some_awesome_client_id");
        assert_eq!(request.method().as_str(), reqwest::Method::GET);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().len(), 0);
        assert_eq!(request.body().is_none(), true);
    }

    #[test]
    fn get_metadata_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let ws_federation = Api::init(base_url, authentication);
        let request = ws_federation.get_metadata().build().unwrap();
        let test_url = String::from(
            "https://your_domain/wsfed/FederationMetadata/2007-06/FederationMetadata.xml",
        );
        assert_eq!(request.method().as_str(), reqwest::Method::GET);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().len(), 0);
        assert_eq!(request.body().is_none(), true);
    }
}
