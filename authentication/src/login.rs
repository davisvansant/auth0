use crate::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// #[derive(Serialize, Deserialize)]
// pub enum ResponseType {
//     Code(String),
//     Token(String),
// }

#[derive(Serialize, Deserialize)]
pub enum AuthenicationType {
    Social(Social),
    Passive(Passive),
    Enterprise(Enterprise),
}

#[derive(Serialize, Deserialize)]
pub struct Social {
    pub response_type: String,
    pub client_id: String,
    // pub connection: Option<String>,
    pub redirect_uri: String,
    // pub state: Option<String>,
    // additional_parameters: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Passive {
    response_type: String,
    client_id: String,
    connection: Option<String>,
    redirect_uri: String,
    scope: Option<Vec<String>>,
    state: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Enterprise {
    response_type: String,
    client_id: String,
    connection: Option<String>,
    redirect_uri: String,
    state: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    parameters: AuthenicationType,
}

impl LoginRequest {
    pub fn collect(parameters: AuthenicationType) -> LoginRequest {
        LoginRequest { parameters }
    }
}

pub trait Login {
    fn authorize(&self, parameters: LoginRequest) -> RequestBuilder;
}

impl Login for Api {
    fn authorize(&self, login_request: LoginRequest) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/authorize");
        let url = self.base_url.join(&endpoint).unwrap();
        let json = serde_json::to_value(login_request.parameters).unwrap();
        let mut parameters = BTreeMap::new();
        let map = json.as_object().unwrap();
        for (_k, v) in map.iter() {
            let values = v.as_object().unwrap();
            for (k, v) in values.iter() {
                parameters.insert(k.as_str(), v.as_str().unwrap());
            }
        }
        client.get(url).query(&parameters)
    }
}
