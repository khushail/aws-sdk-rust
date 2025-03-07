// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_hsm::_create_hsm_output::CreateHsmOutputBuilder;

pub use crate::operation::create_hsm::_create_hsm_input::CreateHsmInputBuilder;

/// Fluent builder constructing a request to `CreateHsm`.
///
/// <p>Creates a new hardware security module (HSM) in the specified AWS CloudHSM cluster.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateHsmFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_hsm::builders::CreateHsmInputBuilder,
}
impl CreateHsmFluentBuilder {
    /// Creates a new `CreateHsm`.
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
            crate::operation::create_hsm::CreateHsm,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_hsm::CreateHsmError>,
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
        crate::operation::create_hsm::CreateHsmOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_hsm::CreateHsmError>,
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
        crate::operation::create_hsm::CreateHsmOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_hsm::CreateHsmError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_hsm::CreateHsm,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_hsm::CreateHsmError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The identifier (ID) of the HSM's cluster. To find the cluster ID, use <code>DescribeClusters</code>.</p>
    pub fn cluster_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster_id(input.into());
        self
    }
    /// <p>The identifier (ID) of the HSM's cluster. To find the cluster ID, use <code>DescribeClusters</code>.</p>
    pub fn set_cluster_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_id(input);
        self
    }
    /// <p>The Availability Zone where you are creating the HSM. To find the cluster's Availability Zones, use <code>DescribeClusters</code>.</p>
    pub fn availability_zone(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.availability_zone(input.into());
        self
    }
    /// <p>The Availability Zone where you are creating the HSM. To find the cluster's Availability Zones, use <code>DescribeClusters</code>.</p>
    pub fn set_availability_zone(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_availability_zone(input);
        self
    }
    /// <p>The HSM's IP address. If you specify an IP address, use an available address from the subnet that maps to the Availability Zone where you are creating the HSM. If you don't specify an IP address, one is chosen for you from that subnet.</p>
    pub fn ip_address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ip_address(input.into());
        self
    }
    /// <p>The HSM's IP address. If you specify an IP address, use an available address from the subnet that maps to the Availability Zone where you are creating the HSM. If you don't specify an IP address, one is chosen for you from that subnet.</p>
    pub fn set_ip_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ip_address(input);
        self
    }
}
