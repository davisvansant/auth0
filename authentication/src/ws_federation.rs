use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

pub mod accept_request;

pub trait WSFederation {
    fn accept_request(&self, request: accept_request::RequestParameters) -> RequestBuilder;
    fn get_metadata(&self) -> RequestBuilder;
}

impl WSFederation for Api {
    fn accept_request(&self, request: accept_request::RequestParameters) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/wsfed/");
        let base_url = self.base_url.join(&endpoint).unwrap();
        let url = base_url.join(&request.client_id).unwrap();

        client.get(url).query(&request)
    }

    fn get_metadata(&self) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/wsfed/FederationMetadata/2007-06/FederationMetadata.xml");
        let url = self.base_url.join(&endpoint).unwrap();
        client.get(url)
    }
}
