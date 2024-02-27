use auth0::authentication::get_token::*;
use auth0::authentication::*;
use mockito::mock;

#[tokio::test]
async fn authorization_code_flow_send_request() {
    let mock = mock("POST", "/oauth/token")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body(
            "grant_type=some_awesome_grant&\
            client_id=some_awesome_client_id&\
            client_secret=some_awesome_client_secret&\
            code=some_awesome_code",
        )
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let get_token = Api::init(base_url, authentication);
    let test_parameters = get_token::authorization_code_flow::RequestParameters {
        grant_type: String::from("some_awesome_grant"),
        client_id: String::from("some_awesome_client_id"),
        client_secret: String::from("some_awesome_client_secret"),
        code: String::from("some_awesome_code"),
        redirect_uri: None,
    };
    let test_response = get_token
        .authorization_code_flow(test_parameters)
        .send()
        .await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn authorization_code_flow_with_pkce_send_request() {
    let mock = mock("POST", "/oauth/token")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body(
            "grant_type=some_awesome_grant&\
            client_id=some_awesome_client_id&\
            code=some_awesome_code&\
            code_verifier=some_awesome_code_verifier",
        )
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let get_token = Api::init(base_url, authentication);
    let test_parameters = get_token::authorization_code_flow_with_pkce::RequestParameters {
        grant_type: String::from("some_awesome_grant"),
        client_id: String::from("some_awesome_client_id"),
        code: String::from("some_awesome_code"),
        code_verifier: String::from("some_awesome_code_verifier"),
        redirect_uri: None,
    };
    let test_response = get_token
        .authorization_code_flow_with_pkce(test_parameters)
        .send()
        .await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn client_credentials_flow_send_request() {
    let mock = mock("POST", "/oauth/token")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body(
            "grant_type=some_awesome_grant_type&\
            client_id=some_awesome_client_id&\
            client_secret=some_awesome_client_secret&\
            audience=some_awesome_audience_api",
        )
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let get_token = Api::init(base_url, authentication);
    let test_parameters = get_token::client_credentials_flow::RequestParameters {
        grant_type: String::from("some_awesome_grant_type"),
        client_id: String::from("some_awesome_client_id"),
        client_secret: String::from("some_awesome_client_secret"),
        audience: String::from("some_awesome_audience_api"),
    };
    let test_response = get_token
        .client_credentials_flow(test_parameters)
        .send()
        .await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn resource_owner_password_send_request() {
    let mock = mock("POST", "/oauth/token")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body(
            "grant_type=some_awesome_grant_type&\
            client_id=some_awesome_client_id&\
            username=some_awesome_username&\
            password=some_awesome_password&\
            auth0_forwarded_for=some_ip_address",
        )
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let get_token = Api::init(base_url, authentication);
    let test_parameters = get_token::resource_owner_password::RequestParameters {
        grant_type: String::from("some_awesome_grant_type"),
        client_id: String::from("some_awesome_client_id"),
        client_secret: None,
        audience: None,
        username: String::from("some_awesome_username"),
        password: String::from("some_awesome_password"),
        scope: None,
        realm: None,
        auth0_forwarded_for: Some(String::from("some_ip_address")),
    };
    let test_response = get_token
        .resource_owner_password(test_parameters)
        .send()
        .await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn device_authorization_flow_send_request() {
    let mock = mock("POST", "/oauth/token")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body(
            "grant_type=some_awesome_grant_type&\
            client_id=some_awesome_client_id&\
            device_code=some_awesome_device_code",
        )
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let get_token = Api::init(base_url, authentication);
    let test_parameters = get_token::device_authorization_flow::RequestParameters {
        grant_type: String::from("some_awesome_grant_type"),
        client_id: String::from("some_awesome_client_id"),
        device_code: String::from("some_awesome_device_code"),
    };
    let test_response = get_token
        .device_authorization_flow(test_parameters)
        .send()
        .await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn refresh_token_send_request() {
    let mock = mock("POST", "/oauth/token")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body(
            "grant_type=some_awesome_grant_type&\
            client_id=some_awesome_client_id&\
            refresh_token=some_awesome_refresh_token",
        )
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let get_token = Api::init(base_url, authentication);
    let test_parameters = get_token::refresh_token::RequestParameters {
        grant_type: String::from("some_awesome_grant_type"),
        client_id: String::from("some_awesome_client_id"),
        client_secret: None,
        refresh_token: String::from("some_awesome_refresh_token"),
        scope: None,
    };
    let test_response = get_token.refresh_token(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn token_exchange_send_request() {
    let mock = mock("POST", "/oauth/token")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body(
            "grant_type=some_awesome_grant_type&\
            subject_token=some_awesome_subject_token&\
            subject_token_type=some_awesome_subject_token_type&\
            client_id=some_awesome_client_id",
        )
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let get_token = Api::init(base_url, authentication);
    let test_parameters = get_token::token_exchange_for_native_social::RequestParameters {
        grant_type: String::from("some_awesome_grant_type"),
        subject_token: String::from("some_awesome_subject_token"),
        subject_token_type: String::from("some_awesome_subject_token_type"),
        client_id: String::from("some_awesome_client_id"),
        audience: None,
        scope: None,
        auth0_forwarded_for: None,
    };
    let test_response = get_token
        .token_exchange_for_native_social(test_parameters)
        .send()
        .await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}
