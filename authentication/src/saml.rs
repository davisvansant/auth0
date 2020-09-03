use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

pub struct AcceptRequestParameters {
    pub client_id: String,
    pub connection: Option<String>,
}

pub struct GetMetadataRequestParameters {
    pub client_id: String,
}

pub struct IdPFlowRequestParameters {
    pub connection: String,
    pub saml_response: String,
}

pub trait SAML {
    fn accept_request(&self, parameters: AcceptRequestParameters) -> RequestBuilder;
    fn get_metadata(&self, request: GetMetadataRequestParameters) -> RequestBuilder;
    fn idp_flow(&self, request: IdPFlowRequestParameters) -> RequestBuilder;
}

impl SAML for Api {
    fn accept_request(&self, request: AcceptRequestParameters) -> RequestBuilder {
        #[derive(Serialize, Deserialize)]
        struct QueryParameter {
            #[serde(skip_serializing_if = "Option::is_none")]
            connection: Option<String>,
        }

        let client = reqwest::Client::new();
        let endpoint = String::from("/samlp/");
        let base_url = self.base_url.join(&endpoint).unwrap();
        let url = base_url.join(&request.client_id).unwrap();

        client.get(url).query(&QueryParameter {
            connection: request.connection,
        })
    }

    fn get_metadata(&self, request: GetMetadataRequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/samlp/metadata/");
        let base_url = self.base_url.join(&endpoint).unwrap();
        let url = base_url.join(&request.client_id).unwrap();

        client.get(url)
    }

    fn idp_flow(&self, request: IdPFlowRequestParameters) -> RequestBuilder {
        #[derive(Serialize, Deserialize)]
        struct QueryParameters {
            connection: String,
            #[serde(rename = "SAMLResponse")]
            saml_response: String,
        }

        let client = reqwest::Client::new();
        let endpoint = String::from("/login/callback");
        let url = self.base_url.join(&endpoint).unwrap();

        client.post(url).query(&QueryParameters {
            connection: request.connection,
            saml_response: request.saml_response,
        })
    }
}
