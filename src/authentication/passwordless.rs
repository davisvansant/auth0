use crate::authentication::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

pub mod authenticate_user;
pub mod get_code_or_link;

pub trait Passwordless {
    fn passwordless_start(&self, request: get_code_or_link::RequestParameters) -> RequestBuilder;
    fn passwordless_login(&self, request: authenticate_user::RequestParameters) -> RequestBuilder;
}

impl Passwordless for Api {
    fn passwordless_start(&self, request: get_code_or_link::RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/passwordless/start");
        let url = self.base_url.join(&endpoint).unwrap();
        self.client.post(url).json(&request)
    }

    fn passwordless_login(&self, request: authenticate_user::RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();
        self.client.post(url).json(&request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::*;

    #[test]
    fn passwordless_start_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let passwordless = Api::init(base_url, authentication);
        let parameters = passwordless::get_code_or_link::RequestParameters {
            client_id: String::from("some_awesome_client_id"),
            client_secret: String::from("some_awesome_client_secret"),
            connection: String::from("some_awesome_connection"),
            email: Some(String::from("tester@awesome.com")),
            phone_number: None,
            send: None,
            auth_params: None,
        };
        let request = passwordless.passwordless_start(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/passwordless/start");
        let test_body = String::from(
            "{\"client_id\":\"some_awesome_client_id\",\
            \"client_secret\":\"some_awesome_client_secret\",\
            \"connection\":\"some_awesome_connection\",\
            \"email\":\"tester@awesome.com\"}",
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
    fn passwordless_login_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let passwordless = Api::init(base_url, authentication);
        let parameters = passwordless::authenticate_user::RequestParameters {
            grant_type: String::from("some_awesome_grant_type"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: String::from("some_awesome_client_secret"),
            username: String::from("some_awesome_username"),
            realm: String::from("some_awesome_realm"),
            otp: String::from("some_awesome_otp"),
            audience: None,
            scope: None,
        };
        let request = passwordless.passwordless_login(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/oauth/token");
        let test_body = String::from(
            "{\"grant_type\":\"some_awesome_grant_type\",\
            \"client_id\":\"some_awesome_client_id\",\
            \"client_secret\":\"some_awesome_client_secret\",\
            \"username\":\"some_awesome_username\",\
            \"realm\":\"some_awesome_realm\",\
            \"otp\":\"some_awesome_otp\"}",
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
