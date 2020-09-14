use crate::Api;
use reqwest::RequestBuilder;
pub use serde::{Deserialize, Serialize};

pub mod enterprise;
pub mod passive;
pub mod social;

pub trait Login {
    fn authorize<T: Serialize>(&self, request: T) -> RequestBuilder;
}

impl Login for Api {
    fn authorize<T: Serialize>(&self, request: T) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/authorize");
        let url = self.base_url.join(&endpoint).unwrap();
        client.get(url).query(&request)
    }
}
