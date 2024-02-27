use crate::authentication::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    #[serde(rename(serialize = "returnTo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federated: Option<String>,
}

pub trait Logout {
    fn logout(&self, request: RequestParameters) -> RequestBuilder;
}

impl Logout for Api {
    fn logout(&self, request: RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/v2/logout");
        let url = self.base_url.join(&endpoint).unwrap();
        self.client.get(url).query(&request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::*;

    #[test]
    fn logout_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let logout = Api::init(base_url, authentication);
        let parameters = logout::RequestParameters {
            return_to: Some(String::from("some_awesome_return")),
            client_id: Some(String::from("some_awesome_client_id")),
            federated: Some(String::from("some_awesome_federated")),
        };
        let request = logout.logout(parameters).build().unwrap();
        let test_url =
            String::from("https://your_domain/v2/logout?returnTo=some_awesome_return&client_id=some_awesome_client_id&federated=some_awesome_federated");
        assert_eq!(request.method().as_str(), reqwest::Method::GET);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().is_empty(), true);
        assert_eq!(request.body().is_none(), true);
    }
}
