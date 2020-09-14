use reqwest::Url;

pub mod authorize_application;
pub mod change_password;
pub mod device_code;
pub mod dynamic_client_registration;
pub mod get_token;
pub mod login;
pub mod logout;
pub mod mfa;
pub mod passwordless;
pub mod revoke_refresh_token;
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
    use crate::authorize_application::*;
    use crate::change_password::*;
    // use crate::device_code::*;
    use crate::dynamic_client_registration::*;
    use crate::get_token::*;
    use crate::login::Login;
    // use crate::login::Social;
    use crate::logout::Logout;
    // use crate::logout::RequestParameters;
    use crate::mfa::*;
    use crate::passwordless::*;
    use crate::revoke_refresh_token::*;
    use crate::saml::*;
    use crate::signup::*;
    use crate::user_profile::*;
    // use crate::ws_federation::*;

    #[test]
    fn it_works() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
        let management = Api::init(base_url, authentication);
        let login_social_request = login::social::RequestParamaters {
            response_type: String::from("some_awesome_response_type"),
            client_id: String::from("some_awesome_client_id"),
            connection: None,
            redirect_uri: String::from("some_awesome_redirect_uri"),
            state: None,
            additional_parameters: None,
        };

        let login_passive_request = login::passive::RequestParamaters {
            response_type: String::from("some_awesome_response_type"),
            client_id: String::from("some_awesome_client_id"),
            connection: None,
            redirect_uri: String::from("some_awesome_redirect_uri"),
            scope: None,
            state: Some(String::from("some_awesome_state")),
        };
        let login_enterprise_request = login::enterprise::RequestParamaters {
            response_type: String::from("some_awesome_response_type"),
            client_id: String::from("some_awesome_client_id"),
            connection: None,
            redirect_uri: String::from("some_awesome_redirect_uri"),
            state: None,
        };
        let logout_request = logout::RequestParameters {
            return_to: Some(String::from("some_awesome_return")),
            client_id: Some(String::from("some_awesome_client_id")),
            federated: Some(String::from("some_awesome_federated")),
        };

        let passwordless_start_parameters = passwordless::get_code_or_link::RequestParameters {
            client_id: String::from("some_awesome_client_id"),
            client_secret: String::from("some_awesome_client_secret"),
            connection: String::from("some_awesome_connection"),
            email: Some(String::from("tester@awesome.com")),
            phone_number: None,
            send: None,
            auth_params: None,
        };
        let passwordless_login_parameters = passwordless::authenticate_user::RequestParameters {
            grant_type: String::from("some_awesome_grant_type"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: String::from("some_awesome_client_secret"),
            username: String::from("some_awesome_username"),
            realm: String::from("some_awesome_realm"),
            otp: String::from("some_awesome_otp"),
            audience: None,
            scope: None,
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

        let authorization_code_flow_request =
            authorize_application::AuthorizationCodeFlowRequestParamaters {
                audience: Some(String::from("some_awesome_audience")),
                scope: Some(String::from("some_awesome_scope")),
                response_type: String::from("some_awesome_response_type"),
                client_id: String::from("some_awesome_client_id"),
                state: Some(String::from("some_awesome_state")),
                redirect_uri: None,
                connection: None,
                prompt: None,
            };

        let authorization_code_flow_with_pkce_request =
            authorize_application::AuthorizationCodeFlowWithPKCERequestParamaters {
                audience: Some(String::from("some_awesome_audience")),
                scope: Some(String::from("some_awesome_scope")),
                response_type: String::from("some_awesome_response_type"),
                client_id: String::from("some_awesome_client_id"),
                state: Some(String::from("some_awesome_state")),
                redirect_uri: None,
                code_challenge_method: String::from("some_awesome_code_challenege_method"),
                code_challenge: String::from("some_awesome_code_challenge"),
                connection: None,
                prompt: None,
            };

        let implicit_flow_request = authorize_application::ImplicitFlowRequestParamaters {
            audience: Some(String::from("some_awesome_audience")),
            scope: Some(String::from("some_awesome_scope")),
            response_type: String::from("some_awesome_response_type"),
            client_id: String::from("some_awesome_client_id"),
            state: Some(String::from("some_awesome_state")),
            redirect_uri: None,
            nonce: None,
            connection: None,
            prompt: None,
        };

        let get_device_code_parameters = device_code::RequestParamaters {
            audience: Some(String::from("some_unique_api_id")),
            scope: Some(String::from("some_awesome_scope")),
            client_id: String::from("some_awesome_application_id"),
        };

        let get_token_authorization_code_flow_parameters =
            get_token::AuthorizationCodeFlowRequestParamaters {
                grant_type: String::from("some_awesome_grant"),
                client_id: String::from("some_awesome_client_id"),
                client_secret: String::from("some_awesome_client_secret"),
                code: String::from("some_awesome_code"),
                redirect_uri: None,
            };

        let get_token_authorization_code_flow_with_pkce_parameters =
            get_token::AuthorizationCodeFlowWithPKCERequestParamaters {
                grant_type: String::from("some_awesome_grant"),
                client_id: String::from("some_awesome_client_id"),
                code: String::from("some_awesome_code"),
                code_verifier: String::from("some_awesome_code_verifier"),
                redirect_uri: None,
            };

        let get_token_client_credentials_flow_parameters =
            get_token::ClientCredentialsFlowRequestParamaters {
                grant_type: String::from("some_awesome_grant_type"),
                client_id: String::from("some_awesome_client_id"),
                client_secret: String::from("some_awesome_client_secret"),
                audience: String::from("some_awesome_audience_api"),
            };

        let get_token_resource_owner_password_request =
            get_token::ResourceOwnerPasswordRequestParamaters {
                grant_type: String::from("some_awesome_grant_type"),
                client_id: String::from("some_awesome_client_id"),
                client_secret: None,
                audience: None,
                username: String::from("some_awesome_username"),
                password: String::from("some_awesome_password"),
                scope: None,
                realm: None,
                auth0_forwarded_for: Some(String::from("some_ip_address")),
            };

        let get_token_device_authorization_flow_request =
            get_token::DeviceAuthorizationFlowRequestParamaters {
                grant_type: String::from("some_awesome_grant_type"),
                client_id: String::from("some_awesome_client_id"),
                device_code: String::from("some_awesome_device_code"),
            };

        let get_token_refresh_token_request = get_token::RefreshTokenRequestParamaters {
            grant_type: String::from("some_awesome_grant_type"),
            client_id: String::from("some_awesome_client_id"),
            client_secret: None,
            refresh_token: String::from("some_awesome_fresh_token"),
            scope: None,
        };

        let get_token_token_exchange_for_native_social_request =
            get_token::TokenExchangeForNativeSocialRequestParameters {
                grant_type: String::from("some_awesome_grant_type"),
                subject_token: String::from("some_awesome_subject_token"),
                subject_token_type: String::from("some_awesome_subject_token_type"),
                client_id: String::from("some_awesome_client_id"),
                audience: None,
                scope: None,
                auth0_forwarded_for: None,
            };

        let revoke_refresh_token_revoke_request = revoke_refresh_token::RequestParameters {
            client_id: String::from("some_awesome_client_id"),
            client_secret: Some(String::from("some_awesome_client_secret")),
            token: String::from("some_awesome_token"),
        };

        management.authorize(login_social_request);
        management.authorize(login_passive_request);
        management.authorize(login_enterprise_request);
        // management.logout(logout_parameters);
        management.logout(logout_request);
        management.passwordless_start(passwordless_start_parameters);
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
        // management.authorization_code_flow(authorization_code_flow_request);
        authorize_application::AuthorizeApplication::authorization_code_flow(
            &management,
            authorization_code_flow_request,
        );
        // management.authorization_code_flow_with_pkce(authorization_code_flow_with_pkce_request);
        authorize_application::AuthorizeApplication::authorization_code_flow_with_pkce(
            &management,
            authorization_code_flow_with_pkce_request,
        );
        management.implicit_flow(implicit_flow_request);
        // management.device_authorization_flow(get_device_code_parameters);
        device_code::GetDeviceCode::device_authorization_flow(
            &management,
            get_device_code_parameters,
        );
        get_token::GetToken::authorization_code_flow(
            &management,
            get_token_authorization_code_flow_parameters,
        );
        get_token::GetToken::authorization_code_flow_with_pkce(
            &management,
            get_token_authorization_code_flow_with_pkce_parameters,
        );
        management.client_credentials_flow(get_token_client_credentials_flow_parameters);
        management.resource_owner_password(get_token_resource_owner_password_request);
        get_token::GetToken::device_authorization_flow(
            &management,
            get_token_device_authorization_flow_request,
        );
        management.refresh_token(get_token_refresh_token_request);
        management
            .token_exchange_for_native_social(get_token_token_exchange_for_native_social_request);
        management.revoke_refresh_token(revoke_refresh_token_revoke_request);
    }
}
