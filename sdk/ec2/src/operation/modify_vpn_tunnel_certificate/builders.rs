// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_vpn_tunnel_certificate::_modify_vpn_tunnel_certificate_output::ModifyVpnTunnelCertificateOutputBuilder;

pub use crate::operation::modify_vpn_tunnel_certificate::_modify_vpn_tunnel_certificate_input::ModifyVpnTunnelCertificateInputBuilder;

/// Fluent builder constructing a request to `ModifyVpnTunnelCertificate`.
///
/// <p>Modifies the VPN tunnel endpoint certificate.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyVpnTunnelCertificateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::modify_vpn_tunnel_certificate::builders::ModifyVpnTunnelCertificateInputBuilder,
}
impl ModifyVpnTunnelCertificateFluentBuilder {
    /// Creates a new `ModifyVpnTunnelCertificate`.
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
            crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificate,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateError,
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
        crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateError,
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
        crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateError,
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
            crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificate,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the Amazon Web Services Site-to-Site VPN connection.</p>
    pub fn vpn_connection_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.vpn_connection_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services Site-to-Site VPN connection.</p>
    pub fn set_vpn_connection_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_vpn_connection_id(input);
        self
    }
    /// <p>The external IP address of the VPN tunnel.</p>
    pub fn vpn_tunnel_outside_ip_address(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.vpn_tunnel_outside_ip_address(input.into());
        self
    }
    /// <p>The external IP address of the VPN tunnel.</p>
    pub fn set_vpn_tunnel_outside_ip_address(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_vpn_tunnel_outside_ip_address(input);
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
