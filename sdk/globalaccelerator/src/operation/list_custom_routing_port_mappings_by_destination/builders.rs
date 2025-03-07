// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_custom_routing_port_mappings_by_destination::_list_custom_routing_port_mappings_by_destination_output::ListCustomRoutingPortMappingsByDestinationOutputBuilder;

pub use crate::operation::list_custom_routing_port_mappings_by_destination::_list_custom_routing_port_mappings_by_destination_input::ListCustomRoutingPortMappingsByDestinationInputBuilder;

/// Fluent builder constructing a request to `ListCustomRoutingPortMappingsByDestination`.
///
/// <p>List the port mappings for a specific EC2 instance (destination) in a VPC subnet endpoint. The response is the mappings for one destination IP address. This is useful when your subnet endpoint has mappings that span multiple custom routing accelerators in your account, or for scenarios where you only want to list the port mappings for a specific destination instance.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListCustomRoutingPortMappingsByDestinationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::list_custom_routing_port_mappings_by_destination::builders::ListCustomRoutingPortMappingsByDestinationInputBuilder,
}
impl ListCustomRoutingPortMappingsByDestinationFluentBuilder {
    /// Creates a new `ListCustomRoutingPortMappingsByDestination`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::list_custom_routing_port_mappings_by_destination::ListCustomRoutingPortMappingsByDestination, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::list_custom_routing_port_mappings_by_destination::ListCustomRoutingPortMappingsByDestinationError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::list_custom_routing_port_mappings_by_destination::ListCustomRoutingPortMappingsByDestinationOutput, ::aws_smithy_http::result::SdkError<crate::operation::list_custom_routing_port_mappings_by_destination::ListCustomRoutingPortMappingsByDestinationError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::list_custom_routing_port_mappings_by_destination::ListCustomRoutingPortMappingsByDestinationOutput, ::aws_smithy_http::result::SdkError<crate::operation::list_custom_routing_port_mappings_by_destination::ListCustomRoutingPortMappingsByDestinationError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::list_custom_routing_port_mappings_by_destination::ListCustomRoutingPortMappingsByDestination, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::list_custom_routing_port_mappings_by_destination::ListCustomRoutingPortMappingsByDestinationError>
    >{
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_custom_routing_port_mappings_by_destination::paginator::ListCustomRoutingPortMappingsByDestinationPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_custom_routing_port_mappings_by_destination::paginator::ListCustomRoutingPortMappingsByDestinationPaginator{
        crate::operation::list_custom_routing_port_mappings_by_destination::paginator::ListCustomRoutingPortMappingsByDestinationPaginator::new(self.handle, self.inner)
    }
    /// <p>The ID for the virtual private cloud (VPC) subnet.</p>
    pub fn endpoint_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.endpoint_id(input.into());
        self
    }
    /// <p>The ID for the virtual private cloud (VPC) subnet.</p>
    pub fn set_endpoint_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_endpoint_id(input);
        self
    }
    /// <p>The endpoint IP address in a virtual private cloud (VPC) subnet for which you want to receive back port mappings.</p>
    pub fn destination_address(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.destination_address(input.into());
        self
    }
    /// <p>The endpoint IP address in a virtual private cloud (VPC) subnet for which you want to receive back port mappings.</p>
    pub fn set_destination_address(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_destination_address(input);
        self
    }
    /// <p>The number of destination port mappings that you want to return with this call. The default value is 10.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The number of destination port mappings that you want to return with this call. The default value is 10.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token for the next set of results. You receive this token from a previous call.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of results. You receive this token from a previous call.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
