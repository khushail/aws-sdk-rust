// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_environment_configuration::_delete_environment_configuration_output::DeleteEnvironmentConfigurationOutputBuilder;

pub use crate::operation::delete_environment_configuration::_delete_environment_configuration_input::DeleteEnvironmentConfigurationInputBuilder;

/// Fluent builder constructing a request to `DeleteEnvironmentConfiguration`.
///
/// <p>Deletes the draft configuration associated with the running environment.</p>
/// <p>Updating a running environment with any configuration changes creates a draft configuration set. You can get the draft configuration using <code>DescribeConfigurationSettings</code> while the update is in progress or if the update fails. The <code>DeploymentStatus</code> for the draft configuration indicates whether the deployment is in process or has failed. The draft configuration remains in existence until it is deleted with this action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteEnvironmentConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::delete_environment_configuration::builders::DeleteEnvironmentConfigurationInputBuilder,
}
impl DeleteEnvironmentConfigurationFluentBuilder {
    /// Creates a new `DeleteEnvironmentConfiguration`.
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
            crate::operation::delete_environment_configuration::DeleteEnvironmentConfiguration,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_environment_configuration::DeleteEnvironmentConfigurationError,
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
        crate::operation::delete_environment_configuration::DeleteEnvironmentConfigurationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_environment_configuration::DeleteEnvironmentConfigurationError,
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
        crate::operation::delete_environment_configuration::DeleteEnvironmentConfigurationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_environment_configuration::DeleteEnvironmentConfigurationError,
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
            crate::operation::delete_environment_configuration::DeleteEnvironmentConfiguration,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_environment_configuration::DeleteEnvironmentConfigurationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the application the environment is associated with.</p>
    pub fn application_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.application_name(input.into());
        self
    }
    /// <p>The name of the application the environment is associated with.</p>
    pub fn set_application_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_application_name(input);
        self
    }
    /// <p>The name of the environment to delete the draft configuration from.</p>
    pub fn environment_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.environment_name(input.into());
        self
    }
    /// <p>The name of the environment to delete the draft configuration from.</p>
    pub fn set_environment_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_environment_name(input);
        self
    }
}
