use reqwest::Url;

pub mod change_password;
pub mod dynamic_client_registration;
pub mod login;
pub mod logout;
pub mod mfa;
pub mod passwordless;
pub mod saml;
pub mod signup;
pub mod user_profile;
pub mod ws_federation;

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
    use crate::change_password::*;
    use crate::dynamic_client_registration::*;
    use crate::login::Login;
    use crate::login::Social;
    use crate::logout::Logout;
    use crate::logout::RequestParameters;
    use crate::mfa::*;
    use crate::passwordless::*;
    use crate::saml::*;
    use crate::signup::*;
    use crate::user_profile::*;
    use crate::ws_federation::*;

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
        let change_password = change_password::RequestParameters {
            client_id: None,
            email: String::from("some_awesome_email"),
            connection: String::from("some_awesome_database_connection"),
        };
        let user_profile = user_profile::RequestParameters {
            access_token: String::from("some_awesome_access_token"),
        };
        let challenge_request = ChallengeRequestParameters {
            mfa_token: String::from("some_awesome_mfa_token"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: None,
            challenge_type: None,
            oob_channel: None,
            authenticator_id: None,
        };
        let otp_request = OTPRequestParameters {
            grant_type: String::from("some_awesome_grant_type"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: None,
            mfa_token: String::from("some_awesome_mfa_token"),
            otp: String::from("some_awesome_otp"),
        };
        let oob_request = OOBRequestParameters {
            grant_type: String::from("some_awesome_grant_type"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: None,
            mfa_token: String::from("some_awesome_mfa_token"),
            oob_code: String::from("some_awesome_otp"),
            binding_code: None,
        };
        let recovery_code = RecoveryCodeRequestParameters {
            grant_type: String::from("some_awesome_grant_type"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: None,
            mfa_token: String::from("some_awesome_mfa_token"),
            recovery_code: String::from("some_awesome_mfa_token"),
        };
        let add_authenticator = AddAuthenticatorRequestParameters {
            client_id: String::from("some_awesome_client_id"),
            client_secret: None,
            authenticator_types: String::from("some_awesome_authenticator_type"),
            oob_channel: None,
            phone_number: None,
        };
        let list_authenticators = ListAuthenticatorsRequestParameters {
            access_token: String::from("some_awesome_access_token"),
        };
        let delete_authenticator = DeleteAuthenticatorRequestParameters {
            access_token: String::from("some_awesome_access_token"),
            authenticator_id: String::from("some_awesome_authenticator_id"),
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
        let change_password_parameters =
            change_password::ChangePasswordRequest::collect(change_password);
        let get_user_info = user_profile::UserProfileRequest::collect(user_profile);
        let challenge_request_parameters = mfa::MultiFactorAuthenticationRequest::collect(
            mfa::RequestType::Challenge(challenge_request),
        );
        let otp_request_parameters =
            mfa::MultiFactorAuthenticationRequest::collect(mfa::RequestType::OTP(otp_request));
        let oob_request_parameters =
            mfa::MultiFactorAuthenticationRequest::collect(mfa::RequestType::OOB(oob_request));
        let recovery_code_parameters = mfa::MultiFactorAuthenticationRequest::collect(
            mfa::RequestType::Recovery(recovery_code),
        );
        let add_authenticator_parameters = mfa::MultiFactorAuthenticationRequest::collect(
            mfa::RequestType::AddAuthenticator(add_authenticator),
        );
        let saml_accept_request = saml::AcceptRequestParameters {
            client_id: String::from("some_awesome_client_id"),
            connection: Some(String::from("some_awesome_connection")),
            // connection: None,
        };
        let saml_get_metadata = saml::GetMetadataRequestParameters {
            client_id: String::from("some_awesome_client_id"),
        };

        let saml_idp_flow = saml::IdPFlowRequestParameters {
            connection: String::from("some_awesome_connection"),
            saml_response: String::from("some_awesome_saml_response"),
        };

        let ws_federation_accept_request = ws_federation::AcceptRequestParameters {
            client_id: String::from("some_awesome_client_id"),
            wtrealm: None,
            whr: None,
            wctx: None,
            wreply: None,
        };

        let dynamic_client_registration_request = dynamic_client_registration::RequestParameters {
            client_name: Some(String::from("some_awesome_client_name")),
            redirect_uris: vec![String::from("some_awesome_uri")],
            token_endpoint_auth_method: Some(String::from("some_awesome_auth_method")),
        };

        management.authorize(parameters);
        management.logout(logout_parameters);
        management.passwordless_start(passwordless_code_parameters);
        management.passwordless_login(passwordless_login_parameters);
        management.signup(signup_parameters);
        management.change_password(change_password_parameters);
        management.userinfo(get_user_info);
        management.challenge_request(challenge_request_parameters);
        management.verify_with_otp(otp_request_parameters);
        management.verify_with_oob(oob_request_parameters);
        management.verify_with_recovery_code(recovery_code_parameters);
        management.add_authenticator(add_authenticator_parameters);
        management.list_authenticators(list_authenticators);
        management.delete_authenticator(delete_authenticator);
        // management.accept_request(saml_accept_request);
        // management.get_metadata(saml_get_metadata);
        saml::SAML::accept_request(&management, saml_accept_request);
        saml::SAML::get_metadata(&management, saml_get_metadata);
        management.idp_flow(saml_idp_flow);
        ws_federation::WSFederation::accept_request(&management, ws_federation_accept_request);
        ws_federation::WSFederation::get_metadata(&management);
        management.register(dynamic_client_registration_request);
    }
}
