use auth0::authentication::authorize_application::*;
use auth0::authentication::*;
use mockito::{mock, Matcher};

#[tokio::test]
async fn authorization_code_flow_send_request() {
    let mock = mock("GET", "/authorize")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("audience".into(), "some_awesome_audience".into()),
            Matcher::UrlEncoded("scope".into(), "some_awesome_scope".into()),
            Matcher::UrlEncoded("response_type".into(), "some_awesome_response_type".into()),
            Matcher::UrlEncoded("client_id".into(), "some_awesome_client_id".into()),
            Matcher::UrlEncoded("state".into(), "some_awesome_state".into()),
        ]))
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let authorize_application = Api::init(base_url, authentication);
    let test_parameters = authorize_application::authorization_code_flow::RequestParameters {
        audience: Some(String::from("some_awesome_audience")),
        scope: Some(String::from("some_awesome_scope")),
        response_type: String::from("some_awesome_response_type"),
        client_id: String::from("some_awesome_client_id"),
        state: Some(String::from("some_awesome_state")),
        redirect_uri: None,
        connection: None,
        prompt: None,
    };
    let test_response = authorize_application
        .authorization_code_flow(test_parameters)
        .send()
        .await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn authorization_code_flow_with_pkce_send_request() {
    let mock = mock("GET", "/authorize")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("audience".into(), "some_awesome_audience".into()),
            Matcher::UrlEncoded("scope".into(), "some_awesome_scope".into()),
            Matcher::UrlEncoded("response_type".into(), "some_awesome_response_type".into()),
            Matcher::UrlEncoded("client_id".into(), "some_awesome_client_id".into()),
            Matcher::UrlEncoded("state".into(), "some_awesome_state".into()),
            Matcher::UrlEncoded(
                "code_challenge_method".into(),
                "some_awesome_code_challenege_method".into(),
            ),
            Matcher::UrlEncoded(
                "code_challenge".into(),
                "some_awesome_code_challenge".into(),
            ),
        ]))
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let authorize_application = Api::init(base_url, authentication);
    let test_parameters = authorize_application::pkce::RequestParameters {
        audience: Some(String::from("some_awesome_audience")),
        scope: Some(String::from("some_awesome_scope")),
        response_type: String::from("some_awesome_response_type"),
        client_id: String::from("some_awesome_client_id"),
        state: Some(String::from("some_awesome_state")),
        redirect_uri: None,
        code_challenge_method: String::from("some_awesome_code_challenege_method"),
        code_challenge: String::from("some_awesome_code_challenge"),
        connection: None,
        prompt: None,
    };
    let test_response = authorize_application
        .authorization_code_flow_with_pkce(test_parameters)
        .send()
        .await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn implicit_flow_send_request() {
    let mock = mock("GET", "/authorize")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("audience".into(), "some_awesome_audience".into()),
            Matcher::UrlEncoded("scope".into(), "some_awesome_scope".into()),
            Matcher::UrlEncoded("response_type".into(), "some_awesome_response_type".into()),
            Matcher::UrlEncoded("client_id".into(), "some_awesome_client_id".into()),
            Matcher::UrlEncoded("state".into(), "some_awesome_state".into()),
        ]))
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let authorize_application = Api::init(base_url, authentication);
    let test_parameters = authorize_application::implicit_flow::RequestParameters {
        audience: Some(String::from("some_awesome_audience")),
        scope: Some(String::from("some_awesome_scope")),
        response_type: String::from("some_awesome_response_type"),
        client_id: String::from("some_awesome_client_id"),
        state: Some(String::from("some_awesome_state")),
        redirect_uri: None,
        nonce: None,
        connection: None,
        prompt: None,
    };
    let test_response = authorize_application
        .implicit_flow(test_parameters)
        .send()
        .await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}
