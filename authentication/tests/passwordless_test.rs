use authentication::passwordless::*;
use authentication::*;

#[tokio::test]
async fn passwordless_start_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let passwordless = Api::init(base_url, authentication);
    let test_parameters = passwordless::get_code_or_link::RequestParameters {
        client_id: String::from("some_awesome_client_id"),
        client_secret: String::from("some_awesome_client_secret"),
        connection: String::from("some_awesome_connection"),
        email: Some(String::from("tester@awesome.com")),
        phone_number: None,
        send: None,
        auth_params: None,
    };
    let test_response = send_request(passwordless.passwordless_start(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}

#[tokio::test]
async fn passwordless_login_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let passwordless = Api::init(base_url, authentication);
    let test_parameters = passwordless::authenticate_user::RequestParameters {
        grant_type: String::from("some_awesome_grant_type"),
        client_id: String::from("some_awesome_client_id"),
        client_secret: String::from("some_awesome_client_secret"),
        username: String::from("some_awesome_username"),
        realm: String::from("some_awesome_realm"),
        otp: String::from("some_awesome_otp"),
        audience: None,
        scope: None,
    };
    let test_response = send_request(passwordless.passwordless_login(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}
