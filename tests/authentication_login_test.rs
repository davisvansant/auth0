use auth0::authentication::login::*;
use auth0::authentication::*;

#[tokio::test]
async fn enterprise_send_request() {
    let mock = mockito::mock("GET", "/authorize")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded(
                "response_type".into(),
                "some_awesome_response_type".into(),
            ),
            mockito::Matcher::UrlEncoded("client_id".into(), "some_awesome_client_id".into()),
            mockito::Matcher::UrlEncoded("redirect_uri".into(), "some_awesome_redirect_uri".into()),
        ]))
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let login = Api::init(base_url, authentication);
    let test_parameters = login::enterprise::RequestParameters {
        response_type: String::from("some_awesome_response_type"),
        client_id: String::from("some_awesome_client_id"),
        connection: None,
        redirect_uri: String::from("some_awesome_redirect_uri"),
        state: None,
    };
    let test_response = login.authorize(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn passive_send_request() {
    let mock = mockito::mock("GET", "/authorize")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded(
                "response_type".into(),
                "some_awesome_response_type".into(),
            ),
            mockito::Matcher::UrlEncoded("client_id".into(), "some_awesome_client_id".into()),
            mockito::Matcher::UrlEncoded("redirect_uri".into(), "some_awesome_redirect_uri".into()),
            mockito::Matcher::UrlEncoded("state".into(), "some_awesome_state".into()),
        ]))
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let login = Api::init(base_url, authentication);
    let test_parameters = login::passive::RequestParameters {
        response_type: String::from("some_awesome_response_type"),
        client_id: String::from("some_awesome_client_id"),
        connection: None,
        redirect_uri: String::from("some_awesome_redirect_uri"),
        scope: None,
        state: Some(String::from("some_awesome_state")),
    };
    let test_response = login.authorize(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn social_send_request() {
    let mock = mockito::mock("GET", "/authorize")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded(
                "response_type".into(),
                "some_awesome_response_type".into(),
            ),
            mockito::Matcher::UrlEncoded("client_id".into(), "some_awesome_client_id".into()),
            mockito::Matcher::UrlEncoded("redirect_uri".into(), "some_awesome_redirect_uri".into()),
        ]))
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let login = Api::init(base_url, authentication);
    let test_parameters = login::social::RequestParameters {
        response_type: String::from("some_awesome_response_type"),
        client_id: String::from("some_awesome_client_id"),
        connection: None,
        redirect_uri: String::from("some_awesome_redirect_uri"),
        state: None,
        additional_parameters: None,
    };
    let test_response = login.authorize(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}
