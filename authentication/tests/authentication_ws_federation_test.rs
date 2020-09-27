use authentication::ws_federation::*;
use authentication::*;

#[tokio::test]
async fn accept_request_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let ws_federation = Api::init(base_url, authentication);
    let test_parameters = ws_federation::accept_request::RequestParameters {
        client_id: String::from("some_awesome_client_id"),
        wtrealm: None,
        whr: None,
        wctx: None,
        wreply: None,
    };
    let test_response = ws_federation.accept_request(test_parameters).send().await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}

#[tokio::test]
async fn get_metadata_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let ws_federation = Api::init(base_url, authentication);
    let test_response = ws_federation.get_metadata().send().await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}
