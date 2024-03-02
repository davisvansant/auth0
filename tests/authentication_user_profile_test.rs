use auth0::authentication::user_profile::*;
use auth0::authentication::*;
use mockito::Server;

#[tokio::test]
async fn user_profile_send_request() {
    let mut server = Server::new_async().await;

    let mock = server.mock("GET", "/userinfo")
        .match_header("authorization", "Bearer some_awesome_access_token")
        .create();
    let base_url = reqwest::Url::parse(&server.url()).unwrap();
    let authentication = AuthenticationMethod::OAuth2Token(String::from("some_awesome_token"));
    let user_profile = Api::init(base_url, authentication);
    let test_parameters = user_profile::RequestParameters {
        access_token: String::from("some_awesome_access_token"),
    };
    let test_response = user_profile.user_info(test_parameters).send().await;
    mock.assert();
    assert!(mock.matched());
    assert_eq!(test_response.unwrap().status(), reqwest::StatusCode::OK);
}
