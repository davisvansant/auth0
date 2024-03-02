use crate::authentication::Api;
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
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        self.client.post(url).form(&request)
    }

    fn authorization_code_flow_with_pkce(
        &self,
        request: authorization_code_flow_with_pkce::RequestParameters,
    ) -> RequestBuilder {
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        self.client.post(url).form(&request)
    }

    fn client_credentials_flow(
        &self,
        request: client_credentials_flow::RequestParameters,
    ) -> RequestBuilder {
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        self.client.post(url).form(&request)
    }

    fn resource_owner_password(
        &self,
        request: resource_owner_password::RequestParameters,
    ) -> RequestBuilder {
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
            self.client.post(url).headers(headers).form(&request)
        } else {
            self.client.post(url).form(&request)
        }
    }

    fn device_authorization_flow(
        &self,
        request: device_authorization_flow::RequestParameters,
    ) -> RequestBuilder {
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        self.client.post(url).form(&request)
    }

    fn refresh_token(&self, request: refresh_token::RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        self.client.post(url).form(&request)
    }

    fn token_exchange_for_native_social(
        &self,
        request: token_exchange_for_native_social::RequestParameters,
    ) -> RequestBuilder {
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
            self.client.post(url).headers(headers).form(&request)
        } else {
            self.client.post(url).form(&request)
        }
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
        let get_token = Api::init(base_url, authentication);
        let parameters = get_token::authorization_code_flow::RequestParameters {
            grant_type: String::from("some_awesome_grant"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: String::from("some_awesome_client_secret"),
            code: String::from("some_awesome_code"),
            redirect_uri: None,
        };
        let request = get_token
            .authorization_code_flow(parameters)
            .build()
            .unwrap();
        let test_url = String::from("https://your_domain/oauth/token");
        let test_body = String::from(
            "grant_type=some_awesome_grant&\
            client_id=some_awesome_client_id&\
            client_secret=some_awesome_client_secret&\
            code=some_awesome_code",
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
    fn authorization_code_flow_with_pkce_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let get_token = Api::init(base_url, authentication);
        let parameters = get_token::authorization_code_flow_with_pkce::RequestParameters {
            grant_type: String::from("some_awesome_grant"),
            client_id: String::from("some_awesome_client_id"),
            code: String::from("some_awesome_code"),
            code_verifier: String::from("some_awesome_code_verifier"),
            redirect_uri: None,
        };
        let request = get_token
            .authorization_code_flow_with_pkce(parameters)
            .build()
            .unwrap();
        let test_url = String::from("https://your_domain/oauth/token");
        let test_body = String::from(
            "grant_type=some_awesome_grant&\
            client_id=some_awesome_client_id&\
            code=some_awesome_code&\
            code_verifier=some_awesome_code_verifier",
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
    fn client_credentials_flow_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let get_token = Api::init(base_url, authentication);
        let parameters = get_token::client_credentials_flow::RequestParameters {
            grant_type: String::from("some_awesome_grant_type"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: String::from("some_awesome_client_secret"),
            audience: String::from("some_awesome_audience_api"),
        };
        let request = get_token
            .client_credentials_flow(parameters)
            .build()
            .unwrap();
        let test_url = String::from("https://your_domain/oauth/token");
        let test_body = String::from(
            "grant_type=some_awesome_grant_type&\
            client_id=some_awesome_client_id&\
            client_secret=some_awesome_client_secret&\
            audience=some_awesome_audience_api",
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
    fn resource_owner_password_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let get_token = Api::init(base_url, authentication);
        let parameters = get_token::resource_owner_password::RequestParameters {
            grant_type: String::from("some_awesome_grant_type"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: None,
            audience: None,
            username: String::from("some_awesome_username"),
            password: String::from("some_awesome_password"),
            scope: None,
            realm: None,
            auth0_forwarded_for: Some(String::from("some_ip_address")),
        };
        let request = get_token
            .resource_owner_password(parameters)
            .build()
            .unwrap();
        let test_url = String::from("https://your_domain/oauth/token");
        let test_body = String::from(
            "grant_type=some_awesome_grant_type&\
            client_id=some_awesome_client_id&\
            username=some_awesome_username&\
            password=some_awesome_password&\
            auth0_forwarded_for=some_ip_address",
        );
        assert_eq!(request.method().as_str(), reqwest::Method::POST);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().len(), 2);
        assert_eq!(
            request.body().unwrap().as_bytes().unwrap(),
            test_body.as_bytes(),
        );
    }

    #[test]
    fn device_authorization_flow_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let get_token = Api::init(base_url, authentication);
        let parameters = get_token::device_authorization_flow::RequestParameters {
            grant_type: String::from("some_awesome_grant_type"),
            client_id: String::from("some_awesome_client_id"),
            device_code: String::from("some_awesome_device_code"),
        };
        let request = get_token
            .device_authorization_flow(parameters)
            .build()
            .unwrap();
        let test_url = String::from("https://your_domain/oauth/token");
        let test_body = String::from(
            "grant_type=some_awesome_grant_type&\
            client_id=some_awesome_client_id&\
            device_code=some_awesome_device_code",
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
    fn refresh_token_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let get_token = Api::init(base_url, authentication);
        let parameters = get_token::refresh_token::RequestParameters {
            grant_type: String::from("some_awesome_grant_type"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: None,
            refresh_token: String::from("some_awesome_refresh_token"),
            scope: None,
        };
        let request = get_token.refresh_token(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/oauth/token");
        let test_body = String::from(
            "grant_type=some_awesome_grant_type&\
            client_id=some_awesome_client_id&\
            refresh_token=some_awesome_refresh_token",
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
    fn token_exchange_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let get_token = Api::init(base_url, authentication);
        let parameters = get_token::token_exchange_for_native_social::RequestParameters {
            grant_type: String::from("some_awesome_grant_type"),
            subject_token: String::from("some_awesome_subject_token"),
            subject_token_type: String::from("some_awesome_subject_token_type"),
            client_id: String::from("some_awesome_client_id"),
            audience: None,
            scope: None,
            auth0_forwarded_for: None,
        };
        let request = get_token
            .token_exchange_for_native_social(parameters)
            .build()
            .unwrap();
        let test_url = String::from("https://your_domain/oauth/token");
        let test_body = String::from(
            "grant_type=some_awesome_grant_type&\
            subject_token=some_awesome_subject_token&\
            subject_token_type=some_awesome_subject_token_type&\
            client_id=some_awesome_client_id",
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
