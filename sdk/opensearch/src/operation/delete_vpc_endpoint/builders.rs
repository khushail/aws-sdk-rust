// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_vpc_endpoint::_delete_vpc_endpoint_output::DeleteVpcEndpointOutputBuilder;

pub use crate::operation::delete_vpc_endpoint::_delete_vpc_endpoint_input::DeleteVpcEndpointInputBuilder;

/// Fluent builder constructing a request to `DeleteVpcEndpoint`.
///
/// <p>Deletes an Amazon OpenSearch Service-managed interface VPC endpoint.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteVpcEndpointFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_vpc_endpoint::builders::DeleteVpcEndpointInputBuilder,
}
impl DeleteVpcEndpointFluentBuilder {
    /// Creates a new `DeleteVpcEndpoint`.
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
            crate::operation::delete_vpc_endpoint::DeleteVpcEndpoint,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_vpc_endpoint::DeleteVpcEndpointError,
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
        crate::operation::delete_vpc_endpoint::DeleteVpcEndpointOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_vpc_endpoint::DeleteVpcEndpointError,
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
        crate::operation::delete_vpc_endpoint::DeleteVpcEndpointOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_vpc_endpoint::DeleteVpcEndpointError,
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
            crate::operation::delete_vpc_endpoint::DeleteVpcEndpoint,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_vpc_endpoint::DeleteVpcEndpointError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique identifier of the endpoint.</p>
    pub fn vpc_endpoint_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.vpc_endpoint_id(input.into());
        self
    }
    /// <p>The unique identifier of the endpoint.</p>
    pub fn set_vpc_endpoint_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_vpc_endpoint_id(input);
        self
    }
}
