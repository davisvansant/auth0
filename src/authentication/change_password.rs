use crate::authentication::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    pub email: String,
    pub connection: String,
}

pub trait ChangePassword {
    fn change_password(&self, request: RequestParameters) -> RequestBuilder;
}

impl ChangePassword for Api {
    fn change_password(&self, request: RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/dbconnections/change_password");
        let url = self.base_url.join(&endpoint).unwrap();
        self.client.post(url).json(&request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::*;

    #[test]
    fn change_password_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let change_password = Api::init(base_url, authentication);
        let parameters = change_password::RequestParameters {
            client_id: None,
            email: String::from("some_awesome_email"),
            connection: String::from("some_awesome_database_connection"),
        };
        let request = change_password.change_password(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/dbconnections/change_password");
        let test_body = String::from(
            "{\"email\":\"some_awesome_email\",\
            \"connection\":\"some_awesome_database_connection\"}",
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
