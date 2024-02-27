use auth0::authentication::revoke_refresh_token::*;
use auth0::authentication::*;
use mockito::mock;

#[tokio::test]
async fn revoke_refresh_token_send_request() {
    let mock = mock("POST", "/oauth/revoke")
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::JsonString(
            r#"{"client_id": "some_awesome_client_id",
            "client_secret": "some_awesome_client_secret",
            "token": "some_awesome_token"}"#
                .to_string(),
        ))
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let revoke_refresh_token = Api::init(base_url, authentication);
    let test_parameters = revoke_refresh_token::RequestParameters {
        client_id: String::from("some_awesome_client_id"),
        client_secret: Some(String::from("some_awesome_client_secret")),
        token: String::from("some_awesome_token"),
    };
    let test_response = revoke_refresh_token
        .revoke_refresh_token(test_parameters)
        .send()
        .await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}
