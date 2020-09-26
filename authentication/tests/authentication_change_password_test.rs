use authentication::change_password::*;
use authentication::*;

#[tokio::test]
async fn change_password_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let change_password = Api::init(base_url, authentication);
    let test_parameters = change_password::RequestParameters {
        client_id: None,
        email: String::from("some_awesome_email"),
        connection: String::from("some_awesome_database_connection"),
    };
    let test_response = send_request(change_password.change_password(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}
