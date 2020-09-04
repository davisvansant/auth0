use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

pub struct AcceptRequestParameters {
    pub client_id: String,
    pub wtrealm: Option<String>,
    pub whr: Option<String>,
    pub wctx: Option<String>,
    pub wreply: Option<String>,
}

pub trait WSFederation {
    fn accept_request(&self, parameters: AcceptRequestParameters) -> RequestBuilder;
    fn get_metadata(&self) -> RequestBuilder;
}

impl WSFederation for Api {
    fn accept_request(&self, request: AcceptRequestParameters) -> RequestBuilder {
        #[derive(Serialize, Deserialize)]
        struct QueryParameters {
            #[serde(skip_serializing_if = "Option::is_none")]
            wtrealm: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            whr: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            wctx: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            wreply: Option<String>,
        }

        let client = reqwest::Client::new();
        let endpoint = String::from("/wsfed/");
        let base_url = self.base_url.join(&endpoint).unwrap();
        let url = base_url.join(&request.client_id).unwrap();

        client.get(url).query(&QueryParameters {
            wtrealm: request.wtrealm,
            whr: request.whr,
            wctx: request.wctx,
            wreply: request.wreply,
        })
    }

    fn get_metadata(&self) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/wsfed/FederationMetadata/2007-06/FederationMetadata.xml");
        let url = self.base_url.join(&endpoint).unwrap();
        client.get(url)
    }
}
