// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Your OIDC IdP workforce configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OidcConfigForResponse {
    /// <p>The OIDC IdP client ID used to configure your private workforce.</p>
    #[doc(hidden)]
    pub client_id: ::std::option::Option<::std::string::String>,
    /// <p>The OIDC IdP issuer used to configure your private workforce.</p>
    #[doc(hidden)]
    pub issuer: ::std::option::Option<::std::string::String>,
    /// <p>The OIDC IdP authorization endpoint used to configure your private workforce.</p>
    #[doc(hidden)]
    pub authorization_endpoint: ::std::option::Option<::std::string::String>,
    /// <p>The OIDC IdP token endpoint used to configure your private workforce.</p>
    #[doc(hidden)]
    pub token_endpoint: ::std::option::Option<::std::string::String>,
    /// <p>The OIDC IdP user information endpoint used to configure your private workforce.</p>
    #[doc(hidden)]
    pub user_info_endpoint: ::std::option::Option<::std::string::String>,
    /// <p>The OIDC IdP logout endpoint used to configure your private workforce.</p>
    #[doc(hidden)]
    pub logout_endpoint: ::std::option::Option<::std::string::String>,
    /// <p>The OIDC IdP JSON Web Key Set (Jwks) URI used to configure your private workforce.</p>
    #[doc(hidden)]
    pub jwks_uri: ::std::option::Option<::std::string::String>,
}
impl OidcConfigForResponse {
    /// <p>The OIDC IdP client ID used to configure your private workforce.</p>
    pub fn client_id(&self) -> ::std::option::Option<&str> {
        self.client_id.as_deref()
    }
    /// <p>The OIDC IdP issuer used to configure your private workforce.</p>
    pub fn issuer(&self) -> ::std::option::Option<&str> {
        self.issuer.as_deref()
    }
    /// <p>The OIDC IdP authorization endpoint used to configure your private workforce.</p>
    pub fn authorization_endpoint(&self) -> ::std::option::Option<&str> {
        self.authorization_endpoint.as_deref()
    }
    /// <p>The OIDC IdP token endpoint used to configure your private workforce.</p>
    pub fn token_endpoint(&self) -> ::std::option::Option<&str> {
        self.token_endpoint.as_deref()
    }
    /// <p>The OIDC IdP user information endpoint used to configure your private workforce.</p>
    pub fn user_info_endpoint(&self) -> ::std::option::Option<&str> {
        self.user_info_endpoint.as_deref()
    }
    /// <p>The OIDC IdP logout endpoint used to configure your private workforce.</p>
    pub fn logout_endpoint(&self) -> ::std::option::Option<&str> {
        self.logout_endpoint.as_deref()
    }
    /// <p>The OIDC IdP JSON Web Key Set (Jwks) URI used to configure your private workforce.</p>
    pub fn jwks_uri(&self) -> ::std::option::Option<&str> {
        self.jwks_uri.as_deref()
    }
}
impl OidcConfigForResponse {
    /// Creates a new builder-style object to manufacture [`OidcConfigForResponse`](crate::types::OidcConfigForResponse).
    pub fn builder() -> crate::types::builders::OidcConfigForResponseBuilder {
        crate::types::builders::OidcConfigForResponseBuilder::default()
    }
}

/// A builder for [`OidcConfigForResponse`](crate::types::OidcConfigForResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OidcConfigForResponseBuilder {
    pub(crate) client_id: ::std::option::Option<::std::string::String>,
    pub(crate) issuer: ::std::option::Option<::std::string::String>,
    pub(crate) authorization_endpoint: ::std::option::Option<::std::string::String>,
    pub(crate) token_endpoint: ::std::option::Option<::std::string::String>,
    pub(crate) user_info_endpoint: ::std::option::Option<::std::string::String>,
    pub(crate) logout_endpoint: ::std::option::Option<::std::string::String>,
    pub(crate) jwks_uri: ::std::option::Option<::std::string::String>,
}
impl OidcConfigForResponseBuilder {
    /// <p>The OIDC IdP client ID used to configure your private workforce.</p>
    pub fn client_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The OIDC IdP client ID used to configure your private workforce.</p>
    pub fn set_client_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_id = input;
        self
    }
    /// <p>The OIDC IdP issuer used to configure your private workforce.</p>
    pub fn issuer(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.issuer = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The OIDC IdP issuer used to configure your private workforce.</p>
    pub fn set_issuer(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.issuer = input;
        self
    }
    /// <p>The OIDC IdP authorization endpoint used to configure your private workforce.</p>
    pub fn authorization_endpoint(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.authorization_endpoint = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The OIDC IdP authorization endpoint used to configure your private workforce.</p>
    pub fn set_authorization_endpoint(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.authorization_endpoint = input;
        self
    }
    /// <p>The OIDC IdP token endpoint used to configure your private workforce.</p>
    pub fn token_endpoint(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.token_endpoint = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The OIDC IdP token endpoint used to configure your private workforce.</p>
    pub fn set_token_endpoint(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.token_endpoint = input;
        self
    }
    /// <p>The OIDC IdP user information endpoint used to configure your private workforce.</p>
    pub fn user_info_endpoint(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.user_info_endpoint = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The OIDC IdP user information endpoint used to configure your private workforce.</p>
    pub fn set_user_info_endpoint(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.user_info_endpoint = input;
        self
    }
    /// <p>The OIDC IdP logout endpoint used to configure your private workforce.</p>
    pub fn logout_endpoint(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.logout_endpoint = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The OIDC IdP logout endpoint used to configure your private workforce.</p>
    pub fn set_logout_endpoint(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.logout_endpoint = input;
        self
    }
    /// <p>The OIDC IdP JSON Web Key Set (Jwks) URI used to configure your private workforce.</p>
    pub fn jwks_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.jwks_uri = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The OIDC IdP JSON Web Key Set (Jwks) URI used to configure your private workforce.</p>
    pub fn set_jwks_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.jwks_uri = input;
        self
    }
    /// Consumes the builder and constructs a [`OidcConfigForResponse`](crate::types::OidcConfigForResponse).
    pub fn build(self) -> crate::types::OidcConfigForResponse {
        crate::types::OidcConfigForResponse {
            client_id: self.client_id,
            issuer: self.issuer,
            authorization_endpoint: self.authorization_endpoint,
            token_endpoint: self.token_endpoint,
            user_info_endpoint: self.user_info_endpoint,
            logout_endpoint: self.logout_endpoint,
            jwks_uri: self.jwks_uri,
        }
    }
}
