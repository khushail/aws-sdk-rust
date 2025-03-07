// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_hosted_configuration_version::_create_hosted_configuration_version_output::CreateHostedConfigurationVersionOutputBuilder;

pub use crate::operation::create_hosted_configuration_version::_create_hosted_configuration_version_input::CreateHostedConfigurationVersionInputBuilder;

/// Fluent builder constructing a request to `CreateHostedConfigurationVersion`.
///
/// <p>Creates a new configuration in the AppConfig hosted configuration store.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateHostedConfigurationVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::create_hosted_configuration_version::builders::CreateHostedConfigurationVersionInputBuilder,
}
impl CreateHostedConfigurationVersionFluentBuilder {
    /// Creates a new `CreateHostedConfigurationVersion`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::create_hosted_configuration_version::CreateHostedConfigurationVersion, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::create_hosted_configuration_version::CreateHostedConfigurationVersionError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::create_hosted_configuration_version::CreateHostedConfigurationVersionOutput, ::aws_smithy_http::result::SdkError<crate::operation::create_hosted_configuration_version::CreateHostedConfigurationVersionError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::create_hosted_configuration_version::CreateHostedConfigurationVersionOutput, ::aws_smithy_http::result::SdkError<crate::operation::create_hosted_configuration_version::CreateHostedConfigurationVersionError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::create_hosted_configuration_version::CreateHostedConfigurationVersion, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::create_hosted_configuration_version::CreateHostedConfigurationVersionError>
    >{
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
    /// <p>The configuration profile ID.</p>
    pub fn configuration_profile_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.configuration_profile_id(input.into());
        self
    }
    /// <p>The configuration profile ID.</p>
    pub fn set_configuration_profile_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_configuration_profile_id(input);
        self
    }
    /// <p>A description of the configuration.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the configuration.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The content of the configuration or the configuration data.</p>
    pub fn content(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.content(input);
        self
    }
    /// <p>The content of the configuration or the configuration data.</p>
    pub fn set_content(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_content(input);
        self
    }
    /// <p>A standard MIME type describing the format of the configuration content. For more information, see <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17">Content-Type</a>.</p>
    pub fn content_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.content_type(input.into());
        self
    }
    /// <p>A standard MIME type describing the format of the configuration content. For more information, see <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17">Content-Type</a>.</p>
    pub fn set_content_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_content_type(input);
        self
    }
    /// <p>An optional locking token used to prevent race conditions from overwriting configuration updates when creating a new version. To ensure your data is not overwritten when creating multiple hosted configuration versions in rapid succession, specify the version number of the latest hosted configuration version.</p>
    pub fn latest_version_number(mut self, input: i32) -> Self {
        self.inner = self.inner.latest_version_number(input);
        self
    }
    /// <p>An optional locking token used to prevent race conditions from overwriting configuration updates when creating a new version. To ensure your data is not overwritten when creating multiple hosted configuration versions in rapid succession, specify the version number of the latest hosted configuration version.</p>
    pub fn set_latest_version_number(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_latest_version_number(input);
        self
    }
    /// <p>An optional, user-defined label for the AppConfig hosted configuration version. This value must contain at least one non-numeric character. For example, "v2.2.0".</p>
    pub fn version_label(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.version_label(input.into());
        self
    }
    /// <p>An optional, user-defined label for the AppConfig hosted configuration version. This value must contain at least one non-numeric character. For example, "v2.2.0".</p>
    pub fn set_version_label(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_version_label(input);
        self
    }
}
