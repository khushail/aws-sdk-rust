// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_vpn_connection::_create_vpn_connection_output::CreateVpnConnectionOutputBuilder;

pub use crate::operation::create_vpn_connection::_create_vpn_connection_input::CreateVpnConnectionInputBuilder;

/// Fluent builder constructing a request to `CreateVpnConnection`.
///
/// <p>Creates a VPN connection between an existing virtual private gateway or transit gateway and a customer gateway. The supported connection type is <code>ipsec.1</code>.</p>
/// <p>The response includes information that you need to give to your network administrator to configure your customer gateway.</p> <important>
/// <p>We strongly recommend that you use HTTPS when calling this operation because the response contains sensitive cryptographic information for configuring your customer gateway device.</p>
/// </important>
/// <p>If you decide to shut down your VPN connection for any reason and later create a new VPN connection, you must reconfigure your customer gateway with the new information returned from this call.</p>
/// <p>This is an idempotent operation. If you perform the operation more than once, Amazon EC2 doesn't return an error.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/vpn/latest/s2svpn/VPC_VPN.html">Amazon Web Services Site-to-Site VPN</a> in the <i>Amazon Web Services Site-to-Site VPN User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateVpnConnectionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_vpn_connection::builders::CreateVpnConnectionInputBuilder,
}
impl CreateVpnConnectionFluentBuilder {
    /// Creates a new `CreateVpnConnection`.
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
            crate::operation::create_vpn_connection::CreateVpnConnection,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_vpn_connection::CreateVpnConnectionError,
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
        crate::operation::create_vpn_connection::CreateVpnConnectionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_vpn_connection::CreateVpnConnectionError,
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
        crate::operation::create_vpn_connection::CreateVpnConnectionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_vpn_connection::CreateVpnConnectionError,
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
            crate::operation::create_vpn_connection::CreateVpnConnection,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_vpn_connection::CreateVpnConnectionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the customer gateway.</p>
    pub fn customer_gateway_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.customer_gateway_id(input.into());
        self
    }
    /// <p>The ID of the customer gateway.</p>
    pub fn set_customer_gateway_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_customer_gateway_id(input);
        self
    }
    /// <p>The type of VPN connection (<code>ipsec.1</code>).</p>
    pub fn r#type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.r#type(input.into());
        self
    }
    /// <p>The type of VPN connection (<code>ipsec.1</code>).</p>
    pub fn set_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The ID of the virtual private gateway. If you specify a virtual private gateway, you cannot specify a transit gateway.</p>
    pub fn vpn_gateway_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.vpn_gateway_id(input.into());
        self
    }
    /// <p>The ID of the virtual private gateway. If you specify a virtual private gateway, you cannot specify a transit gateway.</p>
    pub fn set_vpn_gateway_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_vpn_gateway_id(input);
        self
    }
    /// <p>The ID of the transit gateway. If you specify a transit gateway, you cannot specify a virtual private gateway.</p>
    pub fn transit_gateway_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.transit_gateway_id(input.into());
        self
    }
    /// <p>The ID of the transit gateway. If you specify a transit gateway, you cannot specify a virtual private gateway.</p>
    pub fn set_transit_gateway_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_transit_gateway_id(input);
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
    /// <p>The options for the VPN connection.</p>
    pub fn options(mut self, input: crate::types::VpnConnectionOptionsSpecification) -> Self {
        self.inner = self.inner.options(input);
        self
    }
    /// <p>The options for the VPN connection.</p>
    pub fn set_options(
        mut self,
        input: ::std::option::Option<crate::types::VpnConnectionOptionsSpecification>,
    ) -> Self {
        self.inner = self.inner.set_options(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the VPN connection.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the VPN connection.</p>
    pub fn set_tag_specifications(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
}
