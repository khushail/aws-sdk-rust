// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_resolver_rule::_update_resolver_rule_output::UpdateResolverRuleOutputBuilder;

pub use crate::operation::update_resolver_rule::_update_resolver_rule_input::UpdateResolverRuleInputBuilder;

/// Fluent builder constructing a request to `UpdateResolverRule`.
///
/// <p>Updates settings for a specified Resolver rule. <code>ResolverRuleId</code> is required, and all other parameters are optional. If you don't specify a parameter, it retains its current value.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateResolverRuleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_resolver_rule::builders::UpdateResolverRuleInputBuilder,
}
impl UpdateResolverRuleFluentBuilder {
    /// Creates a new `UpdateResolverRule`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_resolver_rule::UpdateResolverRule,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_resolver_rule::UpdateResolverRuleError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_resolver_rule::UpdateResolverRuleOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_resolver_rule::UpdateResolverRuleError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_resolver_rule::UpdateResolverRuleOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_resolver_rule::UpdateResolverRuleError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_resolver_rule::UpdateResolverRule,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_resolver_rule::UpdateResolverRuleError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the Resolver rule that you want to update.</p>
    pub fn resolver_rule_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.resolver_rule_id(input.into());
        self
    }
    /// <p>The ID of the Resolver rule that you want to update.</p>
    pub fn set_resolver_rule_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_resolver_rule_id(input);
        self
    }
    /// <p>The new settings for the Resolver rule.</p>
    pub fn config(mut self, input: crate::types::ResolverRuleConfig) -> Self {
        self.inner = self.inner.config(input);
        self
    }
    /// <p>The new settings for the Resolver rule.</p>
    pub fn set_config(
        mut self,
        input: ::std::option::Option<crate::types::ResolverRuleConfig>,
    ) -> Self {
        self.inner = self.inner.set_config(input);
        self
    }
}
