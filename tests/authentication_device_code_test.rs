use auth0::authentication::device_code::*;
use auth0::authentication::*;
use mockito::mock;

#[tokio::test]
async fn device_authorization_flow_send_request() {
    let mock = mock("POST", "/oauth/device/code")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body(
            "audience=some_unique_api_id&\
            scope=some_awesome_scope&\
            client_id=some_awesome_application_id",
        )
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let get_device = Api::init(base_url, authentication);
    let test_parameters = device_code::RequestParameters {
        audience: Some(String::from("some_unique_api_id")),
        scope: Some(String::from("some_awesome_scope")),
        client_id: String::from("some_awesome_application_id"),
    };
    let test_response = get_device
        .device_authorization_flow(test_parameters)
        .send()
        .await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}
