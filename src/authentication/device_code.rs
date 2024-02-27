use crate::authentication::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    pub client_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseValues {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: String,
    pub interval: String,
}

pub trait GetDeviceCode {
    fn device_authorization_flow(&self, request: RequestParameters) -> RequestBuilder;
}

impl GetDeviceCode for Api {
    fn device_authorization_flow(&self, request: RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/oauth/device/code");
        let url = self.base_url.join(&endpoint).unwrap();

        self.client.post(url).form(&request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::*;

    #[test]
    fn device_authorization_flow_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let device_code = Api::init(base_url, authentication);
        let parameters = device_code::RequestParameters {
            audience: Some(String::from("some_unique_api_id")),
            scope: Some(String::from("some_awesome_scope")),
            client_id: String::from("some_awesome_application_id"),
        };
        let request = device_code
            .device_authorization_flow(parameters)
            .build()
            .unwrap();
        let test_url = String::from("https://your_domain/oauth/device/code");
        let test_body = String::from(
            "audience=some_unique_api_id&\
            scope=some_awesome_scope&\
            client_id=some_awesome_application_id",
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
