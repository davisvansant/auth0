use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthorizationCodeFlowRequestParamaters {
    pub audience: Option<String>,
    pub scope: Option<String>,
    pub response_type: String,
    pub client_id: String,
    pub state: Option<String>,
    pub redirect_uri: Option<String>,
    pub connection: Option<String>,
    pub prompt: Option<String>,
}

// #[derive(Serialize, Deserialize)]
pub struct AuthorizationCodeFlowWithPKCERequestParamaters {
    pub audience: Option<String>,
    pub scope: Option<String>,
    pub response_type: String,
    pub client_id: String,
    pub state: Option<String>,
    pub redirect_uri: Option<String>,
    pub code_challenge_method: String,
    pub code_challenge: String,
    pub connection: Option<String>,
    pub prompt: Option<String>,
}

pub struct ImplicitFlowRequestParamaters {
    pub audience: Option<String>,
    pub scope: Option<String>,
    pub response_type: String,
    pub client_id: String,
    pub state: Option<String>,
    pub redirect_uri: Option<String>,
    pub nonce: Option<String>,
    pub connection: Option<String>,
    pub prompt: Option<String>,
}

pub trait AuthorizeApplication {
    fn authorization_code_flow(
        &self,
        request: AuthorizationCodeFlowRequestParamaters,
    ) -> RequestBuilder;

    fn authorization_code_flow_with_pkce(
        &self,
        request: AuthorizationCodeFlowWithPKCERequestParamaters,
    ) -> RequestBuilder;

    fn implicit_flow(&self, request: ImplicitFlowRequestParamaters) -> RequestBuilder;
}

impl AuthorizeApplication for Api {
    fn authorization_code_flow(
        &self,
        request: AuthorizationCodeFlowRequestParamaters,
    ) -> RequestBuilder {
        #[derive(Serialize, Deserialize)]
        struct QueryParameters {
            #[serde(skip_serializing_if = "Option::is_none")]
            audience: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            scope: Option<String>,
            response_type: String,
            client_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            state: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            redirect_uri: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            connection: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            prompt: Option<String>,
        }

        let client = reqwest::Client::new();
        let endpoint = String::from("/authorize");
        let url = self.base_url.join(&endpoint).unwrap();

        client.get(url).query(&QueryParameters {
            audience: request.audience,
            scope: request.scope,
            response_type: request.response_type,
            client_id: request.client_id,
            state: request.state,
            redirect_uri: request.redirect_uri,
            connection: request.connection,
            prompt: request.prompt,
        })
    }

    fn authorization_code_flow_with_pkce(
        &self,
        request: AuthorizationCodeFlowWithPKCERequestParamaters,
    ) -> RequestBuilder {
        #[derive(Serialize, Deserialize)]
        struct QueryParameters {
            #[serde(skip_serializing_if = "Option::is_none")]
            audience: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            scope: Option<String>,
            response_type: String,
            client_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            state: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            redirect_uri: Option<String>,
            code_challenge_method: String,
            code_challenge: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            connection: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            prompt: Option<String>,
        }

        let client = reqwest::Client::new();
        let endpoint = String::from("/authorize");
        let url = self.base_url.join(&endpoint).unwrap();

        client.get(url).query(&QueryParameters {
            audience: request.audience,
            scope: request.scope,
            response_type: request.response_type,
            client_id: request.client_id,
            state: request.state,
            redirect_uri: request.redirect_uri,
            code_challenge_method: request.code_challenge_method,
            code_challenge: request.code_challenge,
            connection: request.connection,
            prompt: request.prompt,
        })
    }

    fn implicit_flow(&self, request: ImplicitFlowRequestParamaters) -> RequestBuilder {
        #[derive(Serialize, Deserialize)]
        struct QueryParameters {
            #[serde(skip_serializing_if = "Option::is_none")]
            audience: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            scope: Option<String>,
            response_type: String,
            client_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            state: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            redirect_uri: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            nonce: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            connection: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            prompt: Option<String>,
        }

        let client = reqwest::Client::new();
        let endpoint = String::from("/authorize");
        let url = self.base_url.join(&endpoint).unwrap();

        client.get(url).query(&QueryParameters {
            audience: request.audience,
            scope: request.scope,
            response_type: request.response_type,
            client_id: request.client_id,
            state: request.state,
            redirect_uri: request.redirect_uri,
            nonce: request.nonce,
            connection: request.connection,
            prompt: request.prompt,
        })
    }
}
