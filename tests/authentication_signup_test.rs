use auth0::authentication::signup::*;
use auth0::authentication::*;
use mockito::mock;

#[tokio::test]
async fn signup_send_request() {
    let mock = mock("POST", "/dbconnections/signup")
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::JsonString(
            r#"{"client_id": "some_awesome_client_id",
            "email": "some_awesome_email",
            "password": "some_awesome_password",
            "connection": "some_awesome_connection",
            "username": "some_awesome_username"}"#
                .to_string(),
        ))
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let signup = Api::init(base_url, authentication);
    let test_parameters = signup::RequestParameters {
        client_id: String::from("some_awesome_client_id"),
        email: String::from("some_awesome_email"),
        password: String::from("some_awesome_password"),
        connection: String::from("some_awesome_connection"),
        username: Some(String::from("some_awesome_username")),
        given_name: None,
        family_name: None,
        name: None,
        nickname: None,
        picture: None,
        user_metadata: None,
    };
    let test_response = signup.signup(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}
