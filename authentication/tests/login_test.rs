use authentication::login::*;
use authentication::*;

#[tokio::test]
async fn enterprise_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let login = Api::init(base_url, authentication);
    let test_parameters = login::enterprise::RequestParameters {
        response_type: String::from("some_awesome_response_type"),
        client_id: String::from("some_awesome_client_id"),
        connection: None,
        redirect_uri: String::from("some_awesome_redirect_uri"),
        state: None,
    };
    let test_response = send_request(login.authorize(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}

#[tokio::test]
async fn passive_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let login = Api::init(base_url, authentication);
    let test_parameters = login::passive::RequestParameters {
        response_type: String::from("some_awesome_response_type"),
        client_id: String::from("some_awesome_client_id"),
        connection: None,
        redirect_uri: String::from("some_awesome_redirect_uri"),
        scope: None,
        state: Some(String::from("some_awesome_state")),
    };
    let test_response = send_request(login.authorize(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}

#[tokio::test]
async fn social_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let login = Api::init(base_url, authentication);
    let test_parameters = login::social::RequestParameters {
        response_type: String::from("some_awesome_response_type"),
        client_id: String::from("some_awesome_client_id"),
        connection: None,
        redirect_uri: String::from("some_awesome_redirect_uri"),
        state: None,
        additional_parameters: None,
    };
    let test_response = send_request(login.authorize(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}
