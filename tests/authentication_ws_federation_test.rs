use auth0::authentication::ws_federation::*;
use auth0::authentication::*;
use mockito::mock;

#[tokio::test]
async fn accept_request_send_request() {
    let mock = mock("GET", "/wsfed/some_awesome_client_id").create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let ws_federation = Api::init(base_url, authentication);
    let test_parameters = ws_federation::accept_request::RequestParameters {
        client_id: String::from("some_awesome_client_id"),
        wtrealm: None,
        whr: None,
        wctx: None,
        wreply: None,
    };
    let test_response = ws_federation.accept_request(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn get_metadata_send_request() {
    let mock = mock(
        "GET",
        "/wsfed/FederationMetadata/2007-06/FederationMetadata.xml",
    )
    .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let ws_federation = Api::init(base_url, authentication);
    let test_response = ws_federation.get_metadata().send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}
