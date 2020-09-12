use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RequestType {
    CodeOrLink(CodeOrLink),
    AuthenticateUser(AuthenticateUser),
}

#[derive(Serialize, Deserialize)]
pub struct CodeOrLink {
    pub client_id: String,
    pub client_secret: String,
    pub connection: String,
    pub email: String,
    pub phone_number: String,
    pub send: String,
    #[serde(rename(serialize = "authParams"))]
    pub auth_params: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthenticateUser {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub realm: String,
    pub otp: String,
    pub audience: String,
    pub scope: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordlessRequest {
    parameters: RequestType,
}

impl PasswordlessRequest {
    pub fn collect(parameters: RequestType) -> PasswordlessRequest {
        PasswordlessRequest { parameters }
    }
}

pub trait Passwordless {
    fn passwordless_start(&self, parameters: PasswordlessRequest) -> RequestBuilder;
    fn passwordless_login(&self, parameters: PasswordlessRequest) -> RequestBuilder;
}

impl Passwordless for Api {
    fn passwordless_start(&self, passwordless_request: PasswordlessRequest) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/passwordless/start");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&passwordless_request.parameters)
    }

    fn passwordless_login(&self, passwordless_request: PasswordlessRequest) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/oath/token");
        let url = self.base_url.join(&endpoint).unwrap();
        client.post(url).json(&passwordless_request.parameters)
    }
}
