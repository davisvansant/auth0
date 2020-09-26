use authentication::user_profile::*;
use authentication::*;

#[tokio::test]
async fn user_profile_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let user_profile = Api::init(base_url, authentication);
    let test_parameters = user_profile::RequestParameters {
        access_token: String::from("some_awesome_access_token"),
    };
    let test_response = send_request(user_profile.user_info(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}
