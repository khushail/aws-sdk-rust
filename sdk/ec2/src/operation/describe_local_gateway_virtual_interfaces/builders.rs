// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_local_gateway_virtual_interfaces::_describe_local_gateway_virtual_interfaces_output::DescribeLocalGatewayVirtualInterfacesOutputBuilder;

pub use crate::operation::describe_local_gateway_virtual_interfaces::_describe_local_gateway_virtual_interfaces_input::DescribeLocalGatewayVirtualInterfacesInputBuilder;

/// Fluent builder constructing a request to `DescribeLocalGatewayVirtualInterfaces`.
///
/// <p>Describes the specified local gateway virtual interfaces.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeLocalGatewayVirtualInterfacesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_local_gateway_virtual_interfaces::builders::DescribeLocalGatewayVirtualInterfacesInputBuilder,
}
impl DescribeLocalGatewayVirtualInterfacesFluentBuilder {
    /// Creates a new `DescribeLocalGatewayVirtualInterfaces`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::describe_local_gateway_virtual_interfaces::DescribeLocalGatewayVirtualInterfaces, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::describe_local_gateway_virtual_interfaces::DescribeLocalGatewayVirtualInterfacesError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::describe_local_gateway_virtual_interfaces::DescribeLocalGatewayVirtualInterfacesOutput, ::aws_smithy_http::result::SdkError<crate::operation::describe_local_gateway_virtual_interfaces::DescribeLocalGatewayVirtualInterfacesError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::describe_local_gateway_virtual_interfaces::DescribeLocalGatewayVirtualInterfacesOutput, ::aws_smithy_http::result::SdkError<crate::operation::describe_local_gateway_virtual_interfaces::DescribeLocalGatewayVirtualInterfacesError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::describe_local_gateway_virtual_interfaces::DescribeLocalGatewayVirtualInterfaces, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::describe_local_gateway_virtual_interfaces::DescribeLocalGatewayVirtualInterfacesError>
    >{
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_local_gateway_virtual_interfaces::paginator::DescribeLocalGatewayVirtualInterfacesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_local_gateway_virtual_interfaces::paginator::DescribeLocalGatewayVirtualInterfacesPaginator{
        crate::operation::describe_local_gateway_virtual_interfaces::paginator::DescribeLocalGatewayVirtualInterfacesPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `LocalGatewayVirtualInterfaceIds`.
    ///
    /// To override the contents of this collection use [`set_local_gateway_virtual_interface_ids`](Self::set_local_gateway_virtual_interface_ids).
    ///
    /// <p>The IDs of the virtual interfaces.</p>
    pub fn local_gateway_virtual_interface_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.local_gateway_virtual_interface_ids(input.into());
        self
    }
    /// <p>The IDs of the virtual interfaces.</p>
    pub fn set_local_gateway_virtual_interface_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_local_gateway_virtual_interface_ids(input);
        self
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>local-address</code> - The local address.</p> </li>
    /// <li> <p> <code>local-bgp-asn</code> - The Border Gateway Protocol (BGP) Autonomous System Number (ASN) of the local gateway.</p> </li>
    /// <li> <p> <code>local-gateway-id</code> - The ID of the local gateway.</p> </li>
    /// <li> <p> <code>local-gateway-virtual-interface-id</code> - The ID of the virtual interface.</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the local gateway virtual interface.</p> </li>
    /// <li> <p> <code>peer-address</code> - The peer address.</p> </li>
    /// <li> <p> <code>peer-bgp-asn</code> - The peer BGP ASN.</p> </li>
    /// <li> <p> <code>vlan</code> - The ID of the VLAN.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>local-address</code> - The local address.</p> </li>
    /// <li> <p> <code>local-bgp-asn</code> - The Border Gateway Protocol (BGP) Autonomous System Number (ASN) of the local gateway.</p> </li>
    /// <li> <p> <code>local-gateway-id</code> - The ID of the local gateway.</p> </li>
    /// <li> <p> <code>local-gateway-virtual-interface-id</code> - The ID of the virtual interface.</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the local gateway virtual interface.</p> </li>
    /// <li> <p> <code>peer-address</code> - The peer address.</p> </li>
    /// <li> <p> <code>peer-bgp-asn</code> - The peer BGP ASN.</p> </li>
    /// <li> <p> <code>vlan</code> - The ID of the VLAN.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
}
