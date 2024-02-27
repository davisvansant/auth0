use crate::authentication::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub client_id: String,
    pub email: String,
    pub password: String,
    pub connection: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<String>,
}

pub trait Signup {
    fn signup(&self, request: RequestParameters) -> RequestBuilder;
}

impl Signup for Api {
    fn signup(&self, request: RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/dbconnections/signup");
        let url = self.base_url.join(&endpoint).unwrap();
        self.client.post(url).json(&request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::*;

    #[test]
    fn signup_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let signup = Api::init(base_url, authentication);
        let parameters = signup::RequestParameters {
            client_id: String::from("some_awesome_client_id"),
            email: String::from("some_awesome_email"),
            password: String::from("some_awesome_password"),
            connection: String::from("some_awesome_connection"),
            username: Some(String::from("some_awesome_username")),
            given_name: None,
            family_name: None,
            name: None,
            nickname: None,
            picture: None,
            user_metadata: None,
        };
        let request = signup.signup(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/dbconnections/signup");
        let test_body = String::from(
            "{\"client_id\":\"some_awesome_client_id\",\
            \"email\":\"some_awesome_email\",\
            \"password\":\"some_awesome_password\",\
            \"connection\":\"some_awesome_connection\",\
            \"username\":\"some_awesome_username\"}",
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
