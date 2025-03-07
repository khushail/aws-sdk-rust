// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_environment::_update_environment_output::UpdateEnvironmentOutputBuilder;

pub use crate::operation::update_environment::_update_environment_input::UpdateEnvironmentInputBuilder;

/// Fluent builder constructing a request to `UpdateEnvironment`.
///
/// <p>Updates an environment.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateEnvironmentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_environment::builders::UpdateEnvironmentInputBuilder,
}
impl UpdateEnvironmentFluentBuilder {
    /// Creates a new `UpdateEnvironment`.
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
            crate::operation::update_environment::UpdateEnvironment,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_environment::UpdateEnvironmentError,
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
        crate::operation::update_environment::UpdateEnvironmentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_environment::UpdateEnvironmentError,
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
        crate::operation::update_environment::UpdateEnvironmentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_environment::UpdateEnvironmentError,
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
            crate::operation::update_environment::UpdateEnvironment,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_environment::UpdateEnvironmentError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The application ID.</p>
    pub fn application_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The application ID.</p>
    pub fn set_application_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The environment ID.</p>
    pub fn environment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.environment_id(input.into());
        self
    }
    /// <p>The environment ID.</p>
    pub fn set_environment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_environment_id(input);
        self
    }
    /// <p>The name of the environment.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the environment.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A description of the environment.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the environment.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Appends an item to `Monitors`.
    ///
    /// To override the contents of this collection use [`set_monitors`](Self::set_monitors).
    ///
    /// <p>Amazon CloudWatch alarms to monitor during the deployment process.</p>
    pub fn monitors(mut self, input: crate::types::Monitor) -> Self {
        self.inner = self.inner.monitors(input);
        self
    }
    /// <p>Amazon CloudWatch alarms to monitor during the deployment process.</p>
    pub fn set_monitors(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Monitor>>,
    ) -> Self {
        self.inner = self.inner.set_monitors(input);
        self
    }
}
