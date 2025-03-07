// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateGraphqlApiInput {
    /// <p>The API ID.</p>
    #[doc(hidden)]
    pub api_id: ::std::option::Option<::std::string::String>,
    /// <p>The new name for the <code>GraphqlApi</code> object.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon CloudWatch Logs configuration for the <code>GraphqlApi</code> object.</p>
    #[doc(hidden)]
    pub log_config: ::std::option::Option<crate::types::LogConfig>,
    /// <p>The new authentication type for the <code>GraphqlApi</code> object.</p>
    #[doc(hidden)]
    pub authentication_type: ::std::option::Option<crate::types::AuthenticationType>,
    /// <p>The new Amazon Cognito user pool configuration for the <code>~GraphqlApi</code> object.</p>
    #[doc(hidden)]
    pub user_pool_config: ::std::option::Option<crate::types::UserPoolConfig>,
    /// <p>The OpenID Connect configuration for the <code>GraphqlApi</code> object.</p>
    #[doc(hidden)]
    pub open_id_connect_config: ::std::option::Option<crate::types::OpenIdConnectConfig>,
    /// <p>A list of additional authentication providers for the <code>GraphqlApi</code> API.</p>
    #[doc(hidden)]
    pub additional_authentication_providers:
        ::std::option::Option<::std::vec::Vec<crate::types::AdditionalAuthenticationProvider>>,
    /// <p>A flag indicating whether to use X-Ray tracing for the <code>GraphqlApi</code>.</p>
    #[doc(hidden)]
    pub xray_enabled: ::std::option::Option<bool>,
    /// <p>Configuration for Lambda function authorization.</p>
    #[doc(hidden)]
    pub lambda_authorizer_config: ::std::option::Option<crate::types::LambdaAuthorizerConfig>,
}
impl UpdateGraphqlApiInput {
    /// <p>The API ID.</p>
    pub fn api_id(&self) -> ::std::option::Option<&str> {
        self.api_id.as_deref()
    }
    /// <p>The new name for the <code>GraphqlApi</code> object.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The Amazon CloudWatch Logs configuration for the <code>GraphqlApi</code> object.</p>
    pub fn log_config(&self) -> ::std::option::Option<&crate::types::LogConfig> {
        self.log_config.as_ref()
    }
    /// <p>The new authentication type for the <code>GraphqlApi</code> object.</p>
    pub fn authentication_type(&self) -> ::std::option::Option<&crate::types::AuthenticationType> {
        self.authentication_type.as_ref()
    }
    /// <p>The new Amazon Cognito user pool configuration for the <code>~GraphqlApi</code> object.</p>
    pub fn user_pool_config(&self) -> ::std::option::Option<&crate::types::UserPoolConfig> {
        self.user_pool_config.as_ref()
    }
    /// <p>The OpenID Connect configuration for the <code>GraphqlApi</code> object.</p>
    pub fn open_id_connect_config(
        &self,
    ) -> ::std::option::Option<&crate::types::OpenIdConnectConfig> {
        self.open_id_connect_config.as_ref()
    }
    /// <p>A list of additional authentication providers for the <code>GraphqlApi</code> API.</p>
    pub fn additional_authentication_providers(
        &self,
    ) -> ::std::option::Option<&[crate::types::AdditionalAuthenticationProvider]> {
        self.additional_authentication_providers.as_deref()
    }
    /// <p>A flag indicating whether to use X-Ray tracing for the <code>GraphqlApi</code>.</p>
    pub fn xray_enabled(&self) -> ::std::option::Option<bool> {
        self.xray_enabled
    }
    /// <p>Configuration for Lambda function authorization.</p>
    pub fn lambda_authorizer_config(
        &self,
    ) -> ::std::option::Option<&crate::types::LambdaAuthorizerConfig> {
        self.lambda_authorizer_config.as_ref()
    }
}
impl UpdateGraphqlApiInput {
    /// Creates a new builder-style object to manufacture [`UpdateGraphqlApiInput`](crate::operation::update_graphql_api::UpdateGraphqlApiInput).
    pub fn builder() -> crate::operation::update_graphql_api::builders::UpdateGraphqlApiInputBuilder
    {
        crate::operation::update_graphql_api::builders::UpdateGraphqlApiInputBuilder::default()
    }
}

/// A builder for [`UpdateGraphqlApiInput`](crate::operation::update_graphql_api::UpdateGraphqlApiInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateGraphqlApiInputBuilder {
    pub(crate) api_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) log_config: ::std::option::Option<crate::types::LogConfig>,
    pub(crate) authentication_type: ::std::option::Option<crate::types::AuthenticationType>,
    pub(crate) user_pool_config: ::std::option::Option<crate::types::UserPoolConfig>,
    pub(crate) open_id_connect_config: ::std::option::Option<crate::types::OpenIdConnectConfig>,
    pub(crate) additional_authentication_providers:
        ::std::option::Option<::std::vec::Vec<crate::types::AdditionalAuthenticationProvider>>,
    pub(crate) xray_enabled: ::std::option::Option<bool>,
    pub(crate) lambda_authorizer_config:
        ::std::option::Option<crate::types::LambdaAuthorizerConfig>,
}
impl UpdateGraphqlApiInputBuilder {
    /// <p>The API ID.</p>
    pub fn api_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.api_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The API ID.</p>
    pub fn set_api_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.api_id = input;
        self
    }
    /// <p>The new name for the <code>GraphqlApi</code> object.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The new name for the <code>GraphqlApi</code> object.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The Amazon CloudWatch Logs configuration for the <code>GraphqlApi</code> object.</p>
    pub fn log_config(mut self, input: crate::types::LogConfig) -> Self {
        self.log_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Amazon CloudWatch Logs configuration for the <code>GraphqlApi</code> object.</p>
    pub fn set_log_config(mut self, input: ::std::option::Option<crate::types::LogConfig>) -> Self {
        self.log_config = input;
        self
    }
    /// <p>The new authentication type for the <code>GraphqlApi</code> object.</p>
    pub fn authentication_type(mut self, input: crate::types::AuthenticationType) -> Self {
        self.authentication_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The new authentication type for the <code>GraphqlApi</code> object.</p>
    pub fn set_authentication_type(
        mut self,
        input: ::std::option::Option<crate::types::AuthenticationType>,
    ) -> Self {
        self.authentication_type = input;
        self
    }
    /// <p>The new Amazon Cognito user pool configuration for the <code>~GraphqlApi</code> object.</p>
    pub fn user_pool_config(mut self, input: crate::types::UserPoolConfig) -> Self {
        self.user_pool_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The new Amazon Cognito user pool configuration for the <code>~GraphqlApi</code> object.</p>
    pub fn set_user_pool_config(
        mut self,
        input: ::std::option::Option<crate::types::UserPoolConfig>,
    ) -> Self {
        self.user_pool_config = input;
        self
    }
    /// <p>The OpenID Connect configuration for the <code>GraphqlApi</code> object.</p>
    pub fn open_id_connect_config(mut self, input: crate::types::OpenIdConnectConfig) -> Self {
        self.open_id_connect_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The OpenID Connect configuration for the <code>GraphqlApi</code> object.</p>
    pub fn set_open_id_connect_config(
        mut self,
        input: ::std::option::Option<crate::types::OpenIdConnectConfig>,
    ) -> Self {
        self.open_id_connect_config = input;
        self
    }
    /// Appends an item to `additional_authentication_providers`.
    ///
    /// To override the contents of this collection use [`set_additional_authentication_providers`](Self::set_additional_authentication_providers).
    ///
    /// <p>A list of additional authentication providers for the <code>GraphqlApi</code> API.</p>
    pub fn additional_authentication_providers(
        mut self,
        input: crate::types::AdditionalAuthenticationProvider,
    ) -> Self {
        let mut v = self.additional_authentication_providers.unwrap_or_default();
        v.push(input);
        self.additional_authentication_providers = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of additional authentication providers for the <code>GraphqlApi</code> API.</p>
    pub fn set_additional_authentication_providers(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::AdditionalAuthenticationProvider>,
        >,
    ) -> Self {
        self.additional_authentication_providers = input;
        self
    }
    /// <p>A flag indicating whether to use X-Ray tracing for the <code>GraphqlApi</code>.</p>
    pub fn xray_enabled(mut self, input: bool) -> Self {
        self.xray_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>A flag indicating whether to use X-Ray tracing for the <code>GraphqlApi</code>.</p>
    pub fn set_xray_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.xray_enabled = input;
        self
    }
    /// <p>Configuration for Lambda function authorization.</p>
    pub fn lambda_authorizer_config(mut self, input: crate::types::LambdaAuthorizerConfig) -> Self {
        self.lambda_authorizer_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Configuration for Lambda function authorization.</p>
    pub fn set_lambda_authorizer_config(
        mut self,
        input: ::std::option::Option<crate::types::LambdaAuthorizerConfig>,
    ) -> Self {
        self.lambda_authorizer_config = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateGraphqlApiInput`](crate::operation::update_graphql_api::UpdateGraphqlApiInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_graphql_api::UpdateGraphqlApiInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_graphql_api::UpdateGraphqlApiInput {
                api_id: self.api_id,
                name: self.name,
                log_config: self.log_config,
                authentication_type: self.authentication_type,
                user_pool_config: self.user_pool_config,
                open_id_connect_config: self.open_id_connect_config,
                additional_authentication_providers: self.additional_authentication_providers,
                xray_enabled: self.xray_enabled,
                lambda_authorizer_config: self.lambda_authorizer_config,
            },
        )
    }
}
