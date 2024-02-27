use auth0::authentication::saml::*;
use auth0::authentication::*;
use mockito::mock;

#[tokio::test]
async fn accept_send_request() {
    let mock = mock("GET", "/samlp/some_awesome_client_id")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body("connection=some_awesome_connection")
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let saml = Api::init(base_url, authentication);
    let test_parameters = saml::accept_request::RequestParameters {
        client_id: String::from("some_awesome_client_id"),
        connection: Some(String::from("some_awesome_connection")),
    };
    let test_response = saml.accept_request(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn get_metadata_send_request() {
    let mock = mock("GET", "/samlp/metadata/some_awesome_client_id").create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let saml = Api::init(base_url, authentication);
    let test_parameters = saml::get_metadata::RequestParameters {
        client_id: String::from("some_awesome_client_id"),
    };
    let test_response = saml.get_metadata(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn idp_flow_send_request() {
    let mock = mock("POST", "/login/callback")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body(
            "connection=some_awesome_connection&\
            SAMLResponse=some_awesome_saml_response",
        )
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let saml = Api::init(base_url, authentication);
    let test_parameters = saml::identity_provider::RequestParameters {
        connection: String::from("some_awesome_connection"),
        saml_response: String::from("some_awesome_saml_response"),
    };
    let test_response = saml.idp_flow(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}
