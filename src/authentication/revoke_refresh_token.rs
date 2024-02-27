use crate::authentication::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub token: String,
}

pub trait RevokeRequestToken {
    fn revoke_refresh_token(&self, parameters: RequestParameters) -> RequestBuilder;
}

impl RevokeRequestToken for Api {
    fn revoke_refresh_token(&self, request: RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/oauth/revoke");
        let url = self.base_url.join(&endpoint).unwrap();

        self.client.post(url).json(&request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::*;

    #[test]
    fn revoke_refresh_token_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let revoke_refresh_token = Api::init(base_url, authentication);
        let parameters = revoke_refresh_token::RequestParameters {
            client_id: String::from("some_awesome_client_id"),
            client_secret: Some(String::from("some_awesome_client_secret")),
            token: String::from("some_awesome_token"),
        };
        let request = revoke_refresh_token
            .revoke_refresh_token(parameters)
            .build()
            .unwrap();
        let test_url = String::from("https://your_domain/oauth/revoke");
        let test_body = String::from(
            "{\"client_id\":\"some_awesome_client_id\",\
            \"client_secret\":\"some_awesome_client_secret\",\
            \"token\":\"some_awesome_token\"}",
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
