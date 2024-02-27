use crate::authentication::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    pub redirect_uris: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_method: Option<String>,
}

pub trait DynamicClientRegistration {
    fn register(&self, parameters: RequestParameters) -> RequestBuilder;
}

impl DynamicClientRegistration for Api {
    fn register(&self, request: RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/oidc/register");
        let url = self.base_url.join(&endpoint).unwrap();

        self.client.post(url).json(&request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::*;

    #[test]
    fn register_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let dynamic_client_registration = Api::init(base_url, authentication);
        let parameters = dynamic_client_registration::RequestParameters {
            client_name: Some(String::from("some_awesome_client_name")),
            redirect_uris: vec![String::from("some_awesome_uri")],
            token_endpoint_auth_method: Some(String::from("some_awesome_auth_method")),
        };
        let request = dynamic_client_registration
            .register(parameters)
            .build()
            .unwrap();
        let test_url = String::from("https://your_domain/oidc/register");
        let test_body = String::from(
            "{\"client_name\":\"some_awesome_client_name\",\
            \"redirect_uris\":[\"some_awesome_uri\"],\
            \"token_endpoint_auth_method\":\"some_awesome_auth_method\"}",
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
