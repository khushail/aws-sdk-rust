// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_container_api_metadata::_get_container_api_metadata_output::GetContainerApiMetadataOutputBuilder;

pub use crate::operation::get_container_api_metadata::_get_container_api_metadata_input::GetContainerApiMetadataInputBuilder;

/// Fluent builder constructing a request to `GetContainerAPIMetadata`.
///
/// <p>Returns information about Amazon Lightsail containers, such as the current version of the Lightsail Control (lightsailctl) plugin.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetContainerAPIMetadataFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::get_container_api_metadata::builders::GetContainerApiMetadataInputBuilder,
}
impl GetContainerAPIMetadataFluentBuilder {
    /// Creates a new `GetContainerAPIMetadata`.
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
            crate::operation::get_container_api_metadata::GetContainerAPIMetadata,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_container_api_metadata::GetContainerAPIMetadataError,
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
        crate::operation::get_container_api_metadata::GetContainerApiMetadataOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_container_api_metadata::GetContainerAPIMetadataError,
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
        crate::operation::get_container_api_metadata::GetContainerApiMetadataOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_container_api_metadata::GetContainerAPIMetadataError,
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
            crate::operation::get_container_api_metadata::GetContainerAPIMetadata,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_container_api_metadata::GetContainerAPIMetadataError,
        >,
    > {
        self.customize_middleware().await
    }
}
