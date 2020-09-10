use crate::Api;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
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

pub struct ClientCredentialsFlowRequestParamaters {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub audience: String,
}

pub struct ResourceOwnerPasswordRequestParamaters {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub audience: Option<String>,
    pub username: String,
    pub password: String,
    pub scope: Option<String>,
    pub realm: Option<String>,
    pub auth0_forwarded_for: Option<String>,
}

pub struct DeviceAuthorizationFlowRequestParamaters {
    pub grant_type: String,
    pub client_id: String,
    pub device_code: String,
}

pub struct RefreshTokenRequestParamaters {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub refresh_token: String,
    pub scope: Option<String>,
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

    fn client_credentials_flow(
        &self,
        request: ClientCredentialsFlowRequestParamaters,
    ) -> RequestBuilder;

    fn resource_owner_password(
        &self,
        request: ResourceOwnerPasswordRequestParamaters,
    ) -> RequestBuilder;

    fn device_authorization_flow(
        &self,
        request: DeviceAuthorizationFlowRequestParamaters,
    ) -> RequestBuilder;

    fn refresh_token(&self, request: RefreshTokenRequestParamaters) -> RequestBuilder;
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

    fn client_credentials_flow(
        &self,
        request: ClientCredentialsFlowRequestParamaters,
    ) -> RequestBuilder {
        #[derive(Serialize, Deserialize)]
        struct QueryParameters {
            grant_type: String,
            client_id: String,
            client_secret: String,
            audience: String,
        }

        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).form(&QueryParameters {
            grant_type: request.grant_type,
            client_id: request.client_id,
            client_secret: request.client_secret,
            audience: request.audience,
        })
    }

    fn resource_owner_password(
        &self,
        request: ResourceOwnerPasswordRequestParamaters,
    ) -> RequestBuilder {
        #[derive(Serialize, Deserialize)]
        struct QueryParameters {
            grant_type: String,
            client_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            client_secret: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            audience: Option<String>,
            username: String,
            password: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            scope: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            realm: Option<String>,
        }

        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        if request.auth0_forwarded_for.is_some() {
            let mut headers = HeaderMap::new();
            let header_key = String::from("auth0-forwarded-for");
            let header_value = request.auth0_forwarded_for.unwrap();
            headers.insert(
                HeaderName::from_bytes(header_key.as_bytes()).unwrap(),
                HeaderValue::from_bytes(header_value.as_bytes()).unwrap(),
            );
            client.post(url).headers(headers).form(&QueryParameters {
                grant_type: request.grant_type,
                client_id: request.client_id,
                client_secret: request.client_secret,
                audience: request.audience,
                username: request.username,
                password: request.password,
                scope: request.scope,
                realm: request.realm,
            })
        } else {
            client.post(url).form(&QueryParameters {
                grant_type: request.grant_type,
                client_id: request.client_id,
                client_secret: request.client_secret,
                audience: request.audience,
                username: request.username,
                password: request.password,
                scope: request.scope,
                realm: request.realm,
            })
        }
    }

    fn device_authorization_flow(
        &self,
        request: DeviceAuthorizationFlowRequestParamaters,
    ) -> RequestBuilder {
        #[derive(Serialize, Deserialize)]
        struct FormParameters {
            pub grant_type: String,
            pub client_id: String,
            pub device_code: String,
        }

        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).form(&FormParameters {
            grant_type: request.grant_type,
            client_id: request.client_id,
            device_code: request.device_code,
        })
    }

    fn refresh_token(&self, request: RefreshTokenRequestParamaters) -> RequestBuilder {
        #[derive(Serialize, Deserialize)]
        struct FormParameters {
            pub grant_type: String,
            pub client_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub client_secret: Option<String>,
            pub refresh_token: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub scope: Option<String>,
        }

        let client = reqwest::Client::new();
        let endpoint = String::from("/oauth/token");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).form(&FormParameters {
            grant_type: request.grant_type,
            client_id: request.client_id,
            client_secret: request.client_secret,
            refresh_token: request.refresh_token,
            scope: request.scope,
        })
    }
}
