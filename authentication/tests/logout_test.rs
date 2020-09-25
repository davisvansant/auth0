use authentication::logout::*;
use authentication::*;

#[tokio::test]
async fn logout_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let logout = Api::init(base_url, authentication);
    let test_parameters = logout::RequestParameters {
        return_to: Some(String::from("some_awesome_return")),
        client_id: Some(String::from("some_awesome_client_id")),
        federated: Some(String::from("some_awesome_federated")),
    };
    let test_response = send_request(logout.logout(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}
