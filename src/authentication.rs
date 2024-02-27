use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

pub mod authorize_application;
pub mod change_password;
pub mod device_code;
pub mod dynamic_client_registration;
pub mod get_token;
pub mod login;
pub mod logout;
pub mod mfa;
pub mod passwordless;
pub mod revoke_refresh_token;
pub mod saml;
pub mod signup;
pub mod user_profile;
pub mod ws_federation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    OAuth2Token(String),
    ClientIDClientSecret(String, String),
    ClientID(String),
}

#[derive(Debug, Clone)]
pub struct Api {
    pub base_url: Url,
    pub authentication: Option<AuthenticationMethod>,
    client: Client,
}

impl Api {
    /// Create a new instance of the API client.
    pub fn new(base_url: Url) -> Api {
        Api {
            base_url,
            authentication: None,
            client: Api::build_client(),
        }
    }

    /// Set the authentication method for the API client.
    pub fn set_authentication_method(&mut self, authentication: Option<AuthenticationMethod>) {
        self.authentication = authentication;
    }

    /// Initialize the API client with a base URL and an authentication method.
    pub fn init(base_url: Url, authentication: AuthenticationMethod) -> Api {
        Api {
            base_url,
            authentication: Some(authentication),
            client: Api::build_client(),
        }
    }

    fn build_client() -> Client {
        static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
        reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authentication_api_init() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let api = Api::init(base_url, authentication);
        let request = api
            .client
            .request(reqwest::Method::GET, api.base_url)
            .build()
            .unwrap();
        let test_url = String::from("https://your_domain/");
        assert_eq!(request.method(), reqwest::Method::GET);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().len(), 0);
        assert_eq!(request.body().is_none(), true);
    }
}
