use auth0::authentication::change_password::*;
use auth0::authentication::*;
use mockito::mock;

#[tokio::test]
async fn change_password_send_request() {
    let mock = mock("POST", "/dbconnections/change_password")
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::JsonString(
            r#"{"email": "some_awesome_email",
            "connection": "some_awesome_database_connection"}"#
                .to_string(),
        ))
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let change_password = Api::init(base_url, authentication);
    let test_parameters = change_password::RequestParameters {
        client_id: None,
        email: String::from("some_awesome_email"),
        connection: String::from("some_awesome_database_connection"),
    };
    let test_response = change_password
        .change_password(test_parameters)
        .send()
        .await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}
