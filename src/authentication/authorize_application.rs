use crate::authentication::Api;
use reqwest::RequestBuilder;
pub use serde::{Deserialize, Serialize};

pub mod authorization_code_flow;
pub mod implicit_flow;
pub mod pkce;

pub trait AuthorizeApplication {
    fn authorization_code_flow(
        &self,
        request: authorization_code_flow::RequestParameters,
    ) -> RequestBuilder;

    fn authorization_code_flow_with_pkce(&self, request: pkce::RequestParameters)
        -> RequestBuilder;

    fn implicit_flow(&self, request: implicit_flow::RequestParameters) -> RequestBuilder;
}

impl AuthorizeApplication for Api {
    fn authorization_code_flow(
        &self,
        request: authorization_code_flow::RequestParameters,
    ) -> RequestBuilder {
        let endpoint = String::from("/authorize");
        let url = self.base_url.join(&endpoint).unwrap();

        self.client.get(url).query(&request)
    }

    fn authorization_code_flow_with_pkce(
        &self,
        request: pkce::RequestParameters,
    ) -> RequestBuilder {
        let endpoint = String::from("/authorize");
        let url = self.base_url.join(&endpoint).unwrap();

        self.client.get(url).query(&request)
    }

    fn implicit_flow(&self, request: implicit_flow::RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/authorize");
        let url = self.base_url.join(&endpoint).unwrap();

        self.client.get(url).query(&request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::*;

    #[test]
    fn authorization_code_flow_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let authorize_application = Api::init(base_url, authentication);
        let parameters = authorize_application::authorization_code_flow::RequestParameters {
            audience: Some(String::from("some_awesome_audience")),
            scope: Some(String::from("some_awesome_scope")),
            response_type: String::from("some_awesome_response_type"),
            client_id: String::from("some_awesome_client_id"),
            state: Some(String::from("some_awesome_state")),
            redirect_uri: None,
            connection: None,
            prompt: None,
        };
        let request = authorize_application
            .authorization_code_flow(parameters)
            .build()
            .unwrap();
        let test_url = String::from(
            "https://your_domain/authorize?\
            audience=some_awesome_audience&\
            scope=some_awesome_scope&\
            response_type=some_awesome_response_type&\
            client_id=some_awesome_client_id&\
            state=some_awesome_state",
        );
        assert_eq!(request.method().as_str(), reqwest::Method::GET);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().is_empty(), true);
        assert_eq!(request.body().is_none(), true);
    }

    #[test]
    fn authorization_code_flow_with_pkce_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let authorize_application = Api::init(base_url, authentication);
        let parameters = authorize_application::pkce::RequestParameters {
            audience: Some(String::from("some_awesome_audience")),
            scope: Some(String::from("some_awesome_scope")),
            response_type: String::from("some_awesome_response_type"),
            client_id: String::from("some_awesome_client_id"),
            state: Some(String::from("some_awesome_state")),
            redirect_uri: None,
            code_challenge_method: String::from("some_awesome_code_challenege_method"),
            code_challenge: String::from("some_awesome_code_challenge"),
            connection: None,
            prompt: None,
        };
        let request = authorize_application
            .authorization_code_flow_with_pkce(parameters)
            .build()
            .unwrap();
        let test_url = String::from(
            "https://your_domain/authorize?\
            audience=some_awesome_audience&\
            scope=some_awesome_scope&\
            response_type=some_awesome_response_type&\
            client_id=some_awesome_client_id&\
            state=some_awesome_state&\
            code_challenge_method=some_awesome_code_challenege_method&\
            code_challenge=some_awesome_code_challenge",
        );
        assert_eq!(request.method().as_str(), reqwest::Method::GET);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().is_empty(), true);
        assert_eq!(request.body().is_none(), true);
    }

    #[test]
    fn implicit_flow_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let authorize_application = Api::init(base_url, authentication);
        let parameters = authorize_application::implicit_flow::RequestParameters {
            audience: Some(String::from("some_awesome_audience")),
            scope: Some(String::from("some_awesome_scope")),
            response_type: String::from("some_awesome_response_type"),
            client_id: String::from("some_awesome_client_id"),
            state: Some(String::from("some_awesome_state")),
            redirect_uri: None,
            nonce: None,
            connection: None,
            prompt: None,
        };
        let request = authorize_application
            .implicit_flow(parameters)
            .build()
            .unwrap();
        let test_url = String::from(
            "https://your_domain/authorize?\
            audience=some_awesome_audience&\
            scope=some_awesome_scope&\
            response_type=some_awesome_response_type&\
            client_id=some_awesome_client_id&\
            state=some_awesome_state",
        );
        assert_eq!(request.method().as_str(), reqwest::Method::GET);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().is_empty(), true);
        assert_eq!(request.body().is_none(), true);
    }
}
