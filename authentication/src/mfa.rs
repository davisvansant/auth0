use crate::Api;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RequestType {
    Challenge(ChallengeRequestParameters),
    OTP(OTPRequestParameters),
    OOB(OOBRequestParameters),
    Recovery(RecoveryCodeRequestParameters),
    AddAuthenticator(AddAuthenticatorRequestParameters),
    ListAuthenticators(ListAuthenticatorsRequestParameters),
}

#[derive(Serialize, Deserialize)]
pub struct ChallengeRequestParameters {
    pub mfa_token: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oob_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OTPRequestParameters {
    pub grant_type: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub mfa_token: String,
    pub otp: String,
}

#[derive(Serialize, Deserialize)]
pub struct OOBRequestParameters {
    pub grant_type: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub mfa_token: String,
    pub oob_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_code: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RecoveryCodeRequestParameters {
    pub grant_type: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub mfa_token: String,
    pub recovery_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddAuthenticatorRequestParameters {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub authenticator_types: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oob_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ListAuthenticatorsRequestParameters {
    pub access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteAuthenticatorRequestParameters {
    pub access_token: String,
    pub authenticator_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct MultiFactorAuthenticationRequest {
    parameters: RequestType,
}

impl MultiFactorAuthenticationRequest {
    pub fn collect(parameters: RequestType) -> MultiFactorAuthenticationRequest {
        MultiFactorAuthenticationRequest { parameters }
    }
}

pub trait MultiFactorAuthentication {
    fn challenge_request(&self, parameters: MultiFactorAuthenticationRequest) -> RequestBuilder;
    fn verify_with_otp(&self, request: MultiFactorAuthenticationRequest) -> RequestBuilder;
    fn verify_with_oob(&self, request: MultiFactorAuthenticationRequest) -> RequestBuilder;
    fn verify_with_recovery_code(
        &self,
        request: MultiFactorAuthenticationRequest,
    ) -> RequestBuilder;
    fn add_authenticator(&self, request: MultiFactorAuthenticationRequest) -> RequestBuilder;
    fn list_authenticators(&self, request: ListAuthenticatorsRequestParameters) -> RequestBuilder;
    fn delete_authenticator(&self, request: DeleteAuthenticatorRequestParameters)
        -> RequestBuilder;
}

impl MultiFactorAuthentication for Api {
    fn challenge_request(&self, request: MultiFactorAuthenticationRequest) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/mfa/challenge");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&request.parameters)
    }
    fn verify_with_otp(&self, request: MultiFactorAuthenticationRequest) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&request.parameters)
    }
    fn verify_with_oob(&self, request: MultiFactorAuthenticationRequest) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&request.parameters)
    }
    fn verify_with_recovery_code(
        &self,
        request: MultiFactorAuthenticationRequest,
    ) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&request.parameters)
    }
    fn add_authenticator(&self, request: MultiFactorAuthenticationRequest) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/mfa/associate");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&request.parameters)
    }
    fn list_authenticators(&self, request: ListAuthenticatorsRequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/mfa/authenticators");
        let url = self.base_url.join(&endpoint).unwrap();
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", &request.access_token);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&auth_value).unwrap(),
        );
        client.get(url).headers(headers)
    }
    fn delete_authenticator(
        &self,
        request: DeleteAuthenticatorRequestParameters,
    ) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/mfa/authenticators");
        let base_url = self.base_url.join(&endpoint).unwrap();
        let url = base_url.join(&request.authenticator_id).unwrap();
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", &request.access_token);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&auth_value).unwrap(),
        );
        client.delete(url).headers(headers)
    }
}
