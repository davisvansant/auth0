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

pub trait SAML {
    fn accept_request(&self, parameters: AcceptRequestParameters) -> RequestBuilder;
    fn get_metadata(&self, request: GetMetadataRequestParameters) -> RequestBuilder;
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
}
