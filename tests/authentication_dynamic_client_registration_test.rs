use auth0::authentication::dynamic_client_registration::*;
use auth0::authentication::*;
use mockito::mock;

#[tokio::test]
async fn register_send_request() {
    let mock = mock("POST", "/oidc/register")
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::JsonString(
            r#"{"client_name": "some_awesome_client_name",
            "redirect_uris": ["some_awesome_uri"],
            "token_endpoint_auth_method": "some_awesome_auth_method"}"#
                .to_string(),
        ))
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let dynamic_client_registration = Api::init(base_url, authentication);
    let test_parameters = dynamic_client_registration::RequestParameters {
        client_name: Some(String::from("some_awesome_client_name")),
        redirect_uris: vec![String::from("some_awesome_uri")],
        token_endpoint_auth_method: Some(String::from("some_awesome_auth_method")),
    };
    let test_response = dynamic_client_registration
        .register(test_parameters)
        .send()
        .await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}
