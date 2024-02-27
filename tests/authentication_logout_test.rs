use auth0::authentication::logout::*;
use auth0::authentication::*;

#[tokio::test]
async fn logout_send_request() {
    let mock = mockito::mock("GET", "/v2/logout")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("returnTo".into(), "some_awesome_return".into()),
            mockito::Matcher::UrlEncoded("client_id".into(), "some_awesome_client_id".into()),
            mockito::Matcher::UrlEncoded("federated".into(), "some_awesome_federated".into()),
        ]))
        .create();
    let base_url = reqwest::Url::parse(&mockito::server_url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let logout = Api::init(base_url, authentication);
    let test_parameters = logout::RequestParameters {
        return_to: Some(String::from("some_awesome_return")),
        client_id: Some(String::from("some_awesome_client_id")),
        federated: Some(String::from("some_awesome_federated")),
    };
    let test_response = logout.logout(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}
