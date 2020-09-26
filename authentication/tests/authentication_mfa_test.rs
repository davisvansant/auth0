use authentication::mfa::*;
use authentication::*;

#[tokio::test]
async fn challenge_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let mfa = Api::init(base_url, authentication);
    let test_parameters = mfa::challenge_request::RequestParameters {
        mfa_token: String::from("some_awesome_mfa_token"),
        client_id: String::from("some_awesome_client_id"),
        client_secret: None,
        challenge_type: None,
        oob_channel: None,
        authenticator_id: None,
    };
    let test_response = send_request(mfa.challenge_request(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}

#[tokio::test]
async fn one_time_password_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let mfa = Api::init(base_url, authentication);
    let test_parameters = mfa::one_time_password::RequestParameters {
        grant_type: String::from("some_awesome_grant_type"),
        client_id: String::from("some_awesome_client_id"),
        client_secret: None,
        mfa_token: String::from("some_awesome_mfa_token"),
        otp: String::from("some_awesome_otp"),
    };
    let test_response = send_request(mfa.verify_with_otp(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}

#[tokio::test]
async fn out_of_band_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let mfa = Api::init(base_url, authentication);
    let test_parameters = mfa::out_of_band::RequestParameters {
        grant_type: String::from("some_awesome_grant_type"),
        client_id: String::from("some_awesome_client_id"),
        client_secret: None,
        mfa_token: String::from("some_awesome_mfa_token"),
        oob_code: String::from("some_awesome_oob_code"),
        binding_code: None,
    };
    let test_response = send_request(mfa.verify_with_oob(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}

#[tokio::test]
async fn recovery_code_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let mfa = Api::init(base_url, authentication);
    let test_parameters = mfa::recovery_code::RequestParameters {
        grant_type: String::from("some_awesome_grant_type"),
        client_id: String::from("some_awesome_client_id"),
        client_secret: None,
        mfa_token: String::from("some_awesome_mfa_token"),
        recovery_code: String::from("some_awesome_mfa_token"),
    };
    let test_response = send_request(mfa.verify_with_recovery_code(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}

#[tokio::test]
async fn add_authenticator_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let mfa = Api::init(base_url, authentication);
    let test_parameters = mfa::add_authenticator::RequestParameters {
        client_id: String::from("some_awesome_client_id"),
        client_secret: None,
        authenticator_types: String::from("some_awesome_authenticator_type"),
        oob_channel: None,
        phone_number: None,
    };
    let test_response = send_request(mfa.add_authenticator(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}

#[tokio::test]
async fn list_authenticators_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let mfa = Api::init(base_url, authentication);
    let test_parameters = mfa::list_authenticators::RequestParameters {
        access_token: String::from("some_awesome_access_token"),
    };
    let test_response = send_request(mfa.list_authenticators(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}

#[tokio::test]
async fn delete_authenticator_send_request() {
    let base_url = reqwest::Url::parse("https://YOUR_DOMAIN").unwrap();
    let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
    let mfa = Api::init(base_url, authentication);
    let test_parameters = mfa::delete_authenticator::RequestParameters {
        access_token: String::from("some_awesome_access_token"),
        authenticator_id: String::from("some_awesome_authenticator_id"),
    };
    let test_response = send_request(mfa.delete_authenticator(test_parameters)).await;
    assert!(test_response.is_err());
    assert!(test_response.unwrap_err().is_request());
}
