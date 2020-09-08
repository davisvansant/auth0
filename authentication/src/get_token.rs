use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

pub struct AuthorizationCodeFlowRequestParamaters {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub redirect_uri: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthorizationCodeFlowResponseValues {
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
    pub token_type: String,
    pub expires_in: String,
}

pub struct AuthorizationCodeFlowWithPKCERequestParamaters {
    pub grant_type: String,
    pub client_id: String,
    pub code: String,
    pub code_verifier: String,
    pub redirect_uri: Option<String>,
}

pub trait GetToken {
    fn authorization_code_flow(
        &self,
        request: AuthorizationCodeFlowRequestParamaters,
    ) -> RequestBuilder;

    fn authorization_code_flow_with_pkce(
        &self,
        request: AuthorizationCodeFlowWithPKCERequestParamaters,
    ) -> RequestBuilder;
}

impl GetToken for Api {
    fn authorization_code_flow(
        &self,
        request: AuthorizationCodeFlowRequestParamaters,
    ) -> RequestBuilder {
        #[derive(Serialize, Deserialize)]
        struct QueryParameters {
            pub grant_type: String,
            pub client_id: String,
            pub client_secret: String,
            pub code: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub redirect_uri: Option<String>,
        }

        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).form(&QueryParameters {
            grant_type: request.grant_type,
            client_id: request.client_id,
            client_secret: request.client_secret,
            code: request.code,
            redirect_uri: request.redirect_uri,
        })
    }

    fn authorization_code_flow_with_pkce(
        &self,
        request: AuthorizationCodeFlowWithPKCERequestParamaters,
    ) -> RequestBuilder {
        #[derive(Serialize, Deserialize)]
        struct QueryParameters {
            grant_type: String,
            client_id: String,
            code: String,
            code_verifier: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub redirect_uri: Option<String>,
        }

        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).form(&QueryParameters {
            grant_type: request.grant_type,
            client_id: request.client_id,
            code: request.code,
            code_verifier: request.code_verifier,
            redirect_uri: request.redirect_uri,
        })
    }
}
