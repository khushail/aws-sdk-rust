// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_skill_group::_update_skill_group_output::UpdateSkillGroupOutputBuilder;

pub use crate::operation::update_skill_group::_update_skill_group_input::UpdateSkillGroupInputBuilder;

/// Fluent builder constructing a request to `UpdateSkillGroup`.
///
/// <p>Updates skill group details by skill group ARN.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateSkillGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_skill_group::builders::UpdateSkillGroupInputBuilder,
}
impl UpdateSkillGroupFluentBuilder {
    /// Creates a new `UpdateSkillGroup`.
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
            crate::operation::update_skill_group::UpdateSkillGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_skill_group::UpdateSkillGroupError,
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
        crate::operation::update_skill_group::UpdateSkillGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_skill_group::UpdateSkillGroupError,
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
        crate::operation::update_skill_group::UpdateSkillGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_skill_group::UpdateSkillGroupError,
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
            crate::operation::update_skill_group::UpdateSkillGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_skill_group::UpdateSkillGroupError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ARN of the skill group to update. </p>
    pub fn skill_group_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.skill_group_arn(input.into());
        self
    }
    /// <p>The ARN of the skill group to update. </p>
    pub fn set_skill_group_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_skill_group_arn(input);
        self
    }
    /// <p>The updated name for the skill group.</p>
    pub fn skill_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.skill_group_name(input.into());
        self
    }
    /// <p>The updated name for the skill group.</p>
    pub fn set_skill_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_skill_group_name(input);
        self
    }
    /// <p>The updated description for the skill group.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The updated description for the skill group.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
}
