use reqwest::Url;

pub mod login;
pub mod logout;

pub enum AuthenicationMethod {
    OAuth2Token(String),
    ClientIDClientSecret(String, String),
    ClientID(String),
}

pub struct Api {
    pub base_url: Url,
    pub authentication: AuthenicationMethod,
}

impl Api {
    pub fn init(base_url: Url, authentication: AuthenicationMethod) -> Api {
        Api {
            base_url,
            authentication,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::login::Login;
    use crate::login::Social;
    #[test]
    fn it_works() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
        let management = Api::init(base_url, authentication);
        let social = Social {
            // response_type: ResponseType::Token(String::from("some_awesome_token")),
            response_type: String::from("some_awesome_token"),
            client_id: "some_awesome_client_id".to_string(),
            // connection: None,
            redirect_uri: "some_new_awesome_url".to_string(),
            // state: None,
            // additional_parameters: None,
        };
        let parameters = login::LoginRequest::collect(login::AuthenicationType::Social(social));
        management.authorize(parameters);
    }
}
