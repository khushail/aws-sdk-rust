// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_rule_version::_update_rule_version_output::UpdateRuleVersionOutputBuilder;

pub use crate::operation::update_rule_version::_update_rule_version_input::UpdateRuleVersionInputBuilder;

/// Fluent builder constructing a request to `UpdateRuleVersion`.
///
/// <p>Updates a rule version resulting in a new rule version. Updates a rule version resulting in a new rule version (version 1, 2, 3 ...). </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateRuleVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_rule_version::builders::UpdateRuleVersionInputBuilder,
}
impl UpdateRuleVersionFluentBuilder {
    /// Creates a new `UpdateRuleVersion`.
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
            crate::operation::update_rule_version::UpdateRuleVersion,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_rule_version::UpdateRuleVersionError,
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
        crate::operation::update_rule_version::UpdateRuleVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_rule_version::UpdateRuleVersionError,
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
        crate::operation::update_rule_version::UpdateRuleVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_rule_version::UpdateRuleVersionError,
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
            crate::operation::update_rule_version::UpdateRuleVersion,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_rule_version::UpdateRuleVersionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The rule to update.</p>
    pub fn rule(mut self, input: crate::types::Rule) -> Self {
        self.inner = self.inner.rule(input);
        self
    }
    /// <p>The rule to update.</p>
    pub fn set_rule(mut self, input: ::std::option::Option<crate::types::Rule>) -> Self {
        self.inner = self.inner.set_rule(input);
        self
    }
    /// <p>The description.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The rule expression.</p>
    pub fn expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.expression(input.into());
        self
    }
    /// <p>The rule expression.</p>
    pub fn set_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_expression(input);
        self
    }
    /// <p>The language.</p>
    pub fn language(mut self, input: crate::types::Language) -> Self {
        self.inner = self.inner.language(input);
        self
    }
    /// <p>The language.</p>
    pub fn set_language(mut self, input: ::std::option::Option<crate::types::Language>) -> Self {
        self.inner = self.inner.set_language(input);
        self
    }
    /// Appends an item to `outcomes`.
    ///
    /// To override the contents of this collection use [`set_outcomes`](Self::set_outcomes).
    ///
    /// <p>The outcomes.</p>
    pub fn outcomes(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.outcomes(input.into());
        self
    }
    /// <p>The outcomes.</p>
    pub fn set_outcomes(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_outcomes(input);
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to assign to the rule version.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags to assign to the rule version.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
