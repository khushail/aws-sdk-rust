// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_distribution_configuration::_update_distribution_configuration_output::UpdateDistributionConfigurationOutputBuilder;

pub use crate::operation::update_distribution_configuration::_update_distribution_configuration_input::UpdateDistributionConfigurationInputBuilder;

/// Fluent builder constructing a request to `UpdateDistributionConfiguration`.
///
/// <p>Updates a new distribution configuration. Distribution configurations define and configure the outputs of your pipeline.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDistributionConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_distribution_configuration::builders::UpdateDistributionConfigurationInputBuilder,
}
impl UpdateDistributionConfigurationFluentBuilder {
    /// Creates a new `UpdateDistributionConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::update_distribution_configuration::UpdateDistributionConfiguration, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::update_distribution_configuration::UpdateDistributionConfigurationError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::update_distribution_configuration::UpdateDistributionConfigurationOutput, ::aws_smithy_http::result::SdkError<crate::operation::update_distribution_configuration::UpdateDistributionConfigurationError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::update_distribution_configuration::UpdateDistributionConfigurationOutput, ::aws_smithy_http::result::SdkError<crate::operation::update_distribution_configuration::UpdateDistributionConfigurationError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::update_distribution_configuration::UpdateDistributionConfiguration, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::update_distribution_configuration::UpdateDistributionConfigurationError>
    >{
        self.customize_middleware().await
    }
    /// <p>The Amazon Resource Name (ARN) of the distribution configuration that you want to update.</p>
    pub fn distribution_configuration_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.distribution_configuration_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the distribution configuration that you want to update.</p>
    pub fn set_distribution_configuration_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_distribution_configuration_arn(input);
        self
    }
    /// <p>The description of the distribution configuration.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the distribution configuration.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Appends an item to `distributions`.
    ///
    /// To override the contents of this collection use [`set_distributions`](Self::set_distributions).
    ///
    /// <p>The distributions of the distribution configuration.</p>
    pub fn distributions(mut self, input: crate::types::Distribution) -> Self {
        self.inner = self.inner.distributions(input);
        self
    }
    /// <p>The distributions of the distribution configuration.</p>
    pub fn set_distributions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Distribution>>,
    ) -> Self {
        self.inner = self.inner.set_distributions(input);
        self
    }
    /// <p>The idempotency token of the distribution configuration.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The idempotency token of the distribution configuration.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}
