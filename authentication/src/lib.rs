use reqwest::Url;

pub mod login;
pub mod logout;
pub mod passwordless;
pub mod signup;

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
    use crate::passwordless::*;
    use crate::signup::*;
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
        let passwordless_code = passwordless::CodeOrLink {
            client_id: String::from("some_awesome_client_id"),
            client_secret: String::from("some_awesome_client_secret"),
            connection: String::from("some_awesome_connection"),
            email: String::from("tester@awesome.com"),
            phone_number: String::from("000000000"),
            send: String::from("some_awesome_link_to_send"),
            auth_params: String::from("some_awesome_auth_params"),
        };
        let passwordless_login = passwordless::AuthenticateUser {
            grant_type: String::from("some_awesome_grant_type"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: String::from("some_awesome_client_secret"),
            username: String::from("some_awesome_username"),
            realm: String::from("some_awesome_realm"),
            otp: String::from("some_awesome_otp"),
            audience: String::from("some_awesome_audience"),
            scope: String::from("some_awesome_scope"),
        };
        let signup = signup::RequestParameters {
            client_id: String::from("some_awesome_client_id"),
            email: String::from("some_awesome_email"),
            password: String::from("some_awesome_password"),
            connection: String::from("some_awesome_connection"),
            username: Some(String::from("some_awesome_username")),
            given_name: None,
            family_name: None,
            name: None,
            nickname: None,
            picture: None,
            user_metadata: None,
        };

        let parameters = login::LoginRequest::collect(login::AuthenicationType::Social(social));
        let logout_parameters = logout::LogoutRequest::collect(logout);
        let passwordless_code_parameters = passwordless::PasswordlessRequest::collect(
            passwordless::RequestType::CodeOrLink(passwordless_code),
        );
        let passwordless_login_parameters = passwordless::PasswordlessRequest::collect(
            passwordless::RequestType::AuthenticateUser(passwordless_login),
        );
        let signup_parameters = signup::SignupRequest::collect(signup);
        management.authorize(parameters);
        management.logout(logout_parameters);
        management.passwordless_start(passwordless_code_parameters);
        management.passwordless_login(passwordless_login_parameters);
        management.signup(signup_parameters);
    }
}
