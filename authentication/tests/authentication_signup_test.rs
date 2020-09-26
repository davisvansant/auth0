use authentication::signup::*;
use authentication::*;

#[tokio::test]
async fn signup_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
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
    let test_response = send_request(signup.signup(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}
