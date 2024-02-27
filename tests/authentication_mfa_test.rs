use auth0::authentication::mfa::*;
use auth0::authentication::*;
use mockito::mock;

#[tokio::test]
async fn challenge_send_request() {
    let mock = mock("POST", "/mfa/challenge")
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::JsonString(
            r#"{"mfa_token": "some_awesome_mfa_token",
            "client_id": "some_awesome_client_id"}"#
                .to_string(),
        ))
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let mfa = Api::init(base_url, authentication);
    let test_parameters = mfa::challenge_request::RequestParameters {
        mfa_token: String::from("some_awesome_mfa_token"),
        client_id: String::from("some_awesome_client_id"),
        client_secret: None,
        challenge_type: None,
        oob_channel: None,
        authenticator_id: None,
    };
    let test_response = mfa.challenge_request(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn one_time_password_send_request() {
    let mock = mock("POST", "/oauth/token")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body(
            "grant_type=some_awesome_grant_type&\
            client_id=some_awesome_client_id&\
            mfa_token=some_awesome_mfa_token&\
            otp=some_awesome_otp",
        )
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let mfa = Api::init(base_url, authentication);
    let test_parameters = mfa::one_time_password::RequestParameters {
        grant_type: String::from("some_awesome_grant_type"),
        client_id: String::from("some_awesome_client_id"),
        client_secret: None,
        mfa_token: String::from("some_awesome_mfa_token"),
        otp: String::from("some_awesome_otp"),
    };
    let test_response = mfa.verify_with_otp(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn out_of_band_send_request() {
    let mock = mock("POST", "/oauth/token")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body(
            "grant_type=some_awesome_grant_type&\
            client_id=some_awesome_client_id&\
            mfa_token=some_awesome_mfa_token&\
            oob_code=some_awesome_oob_code",
        )
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let mfa = Api::init(base_url, authentication);
    let test_parameters = mfa::out_of_band::RequestParameters {
        grant_type: String::from("some_awesome_grant_type"),
        client_id: String::from("some_awesome_client_id"),
        client_secret: None,
        mfa_token: String::from("some_awesome_mfa_token"),
        oob_code: String::from("some_awesome_oob_code"),
        binding_code: None,
    };
    let test_response = mfa.verify_with_oob(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn recovery_code_send_request() {
    let mock = mock("POST", "/oauth/token")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body(
            "grant_type=some_awesome_grant_type&\
            client_id=some_awesome_client_id&\
            mfa_token=some_awesome_mfa_token&\
            recovery_code=some_awesome_mfa_token",
        )
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let mfa = Api::init(base_url, authentication);
    let test_parameters = mfa::recovery_code::RequestParameters {
        grant_type: String::from("some_awesome_grant_type"),
        client_id: String::from("some_awesome_client_id"),
        client_secret: None,
        mfa_token: String::from("some_awesome_mfa_token"),
        recovery_code: String::from("some_awesome_mfa_token"),
    };
    let test_response = mfa.verify_with_recovery_code(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn add_authenticator_send_request() {
    let mock = mock("POST", "/mfa/associate")
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::JsonString(
            r#"{"client_id": "some_awesome_client_id",
            "authenticator_types": "some_awesome_authenticator_type"}"#
                .to_string(),
        ))
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let mfa = Api::init(base_url, authentication);
    let test_parameters = mfa::add_authenticator::RequestParameters {
        client_id: String::from("some_awesome_client_id"),
        client_secret: None,
        authenticator_types: String::from("some_awesome_authenticator_type"),
        oob_channel: None,
        phone_number: None,
    };
    let test_response = mfa.add_authenticator(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn list_authenticators_send_request() {
    let mock = mock("GET", "/mfa/authenticators")
        .match_header("authorization", "Bearer some_awesome_access_token")
        .match_header("content-type", "application/json")
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let mfa = Api::init(base_url, authentication);
    let test_parameters = mfa::list_authenticators::RequestParameters {
        access_token: String::from("some_awesome_access_token"),
    };
    let test_response = mfa.list_authenticators(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn delete_authenticator_send_request() {
    let mock = mock(
        "DELETE",
        "/mfa/authenticators/some_awesome_authenticator_id",
    )
    .match_header("authorization", "Bearer some_awesome_access_token")
    .match_header("content-type", "application/json")
    .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let mfa = Api::init(base_url, authentication);
    let test_parameters = mfa::delete_authenticator::RequestParameters {
        access_token: String::from("some_awesome_access_token"),
        authenticator_id: String::from("some_awesome_authenticator_id"),
    };
    let test_response = mfa.delete_authenticator(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}
