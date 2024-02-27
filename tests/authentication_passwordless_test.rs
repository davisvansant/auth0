use auth0::authentication::passwordless::*;
use auth0::authentication::*;

#[tokio::test]
async fn passwordless_start_send_request() {
    let mock = mockito::mock("POST", "/passwordless/start")
        .with_header("content-type", "application/json")
        .match_body(mockito::Matcher::JsonString(
            r#"{"client_id": "some_awesome_client_id",
            "client_secret": "some_awesome_client_secret",
            "connection": "some_awesome_connection",
            "email": "tester@awesome.com"}"#
                .to_string(),
        ))
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
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
    let test_response = passwordless
        .passwordless_start(test_parameters)
        .send()
        .await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn passwordless_login_send_request() {
    let mock = mockito::mock("POST", "/oauth/token")
        .with_header("content-type", "application/json")
        .match_body(mockito::Matcher::JsonString(
            r#"{"grant_type": "some_awesome_grant_type",
            "client_id": "some_awesome_client_id",
            "client_secret": "some_awesome_client_secret",
            "username": "some_awesome_username",
            "realm": "some_awesome_realm",
            "otp": "some_awesome_otp"}"#
                .to_string(),
        ))
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
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
    let test_response = passwordless
        .passwordless_login(test_parameters)
        .send()
        .await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}
