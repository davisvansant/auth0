use crate::authentication::Api;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestParameters {
    pub access_token: String,
}

pub trait UserInfo {
    fn user_info(&self, request: RequestParameters) -> RequestBuilder;
}

impl UserInfo for Api {
    fn user_info(&self, request: RequestParameters) -> RequestBuilder {
        let endpoint = String::from("/userinfo");
        let url = self.base_url.join(&endpoint).unwrap();
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", &request.access_token);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&auth_value).unwrap(),
        );
        self.client.get(url).headers(headers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::*;

    #[test]
    fn get_user_info_build_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
        let user_profile = Api::init(base_url, authentication);
        let parameters = user_profile::RequestParameters {
            access_token: String::from("some_awesome_access_token"),
        };
        let request = user_profile.user_info(parameters).build().unwrap();
        let test_url = String::from("https://your_domain/userinfo");

        assert_eq!(request.method().as_str(), reqwest::Method::GET);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().len(), 1);
        assert!(request
            .headers()
            .contains_key(reqwest::header::AUTHORIZATION));
        assert_eq!(
            request.headers()[reqwest::header::AUTHORIZATION],
            "Bearer some_awesome_access_token",
        );
        assert_eq!(request.body().is_none(), true);
    }
}
