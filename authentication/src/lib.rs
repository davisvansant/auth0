use reqwest::Url;

pub mod login;
pub mod logout;
pub mod passwordless;

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
    use crate::logout::Logout;
    use crate::logout::RequestParameters;
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
        let logout = RequestParameters {
            return_to: String::from("some_awesome_return"),
            client_id: String::from("some_awesome_client_id"),
            federated: String::from("some_awesome_federated"),
        };
        let parameters = login::LoginRequest::collect(login::AuthenicationType::Social(social));
        let logout_parameters = logout::LogoutRequest::collect(logout);
        management.authorize(parameters);
        management.logout(logout_parameters);
    }
}
