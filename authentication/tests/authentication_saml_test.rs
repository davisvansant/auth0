use authentication::saml::*;
use authentication::*;

#[tokio::test]
async fn accept_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let saml = Api::init(base_url, authentication);
    let test_parameters = saml::accept_request::RequestParameters {
        client_id: String::from("some_awesome_client_id"),
        connection: Some(String::from("some_awesome_connection")),
    };
    let test_response = send_request(saml.accept_request(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}

#[tokio::test]
async fn get_metadata_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let saml = Api::init(base_url, authentication);
    let test_parameters = saml::get_metadata::RequestParameters {
        client_id: String::from("some_awesome_client_id"),
    };
    let test_response = send_request(saml.get_metadata(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}

#[tokio::test]
async fn idp_flow_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let saml = Api::init(base_url, authentication);
    let test_parameters = saml::identity_provider::RequestParameters {
        connection: String::from("some_awesome_connection"),
        saml_response: String::from("some_awesome_saml_response"),
    };
    let test_response = send_request(saml.idp_flow(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}
