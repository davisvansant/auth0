use authentication::dynamic_client_registration::*;
use authentication::*;

#[tokio::test]
async fn register_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
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
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}
