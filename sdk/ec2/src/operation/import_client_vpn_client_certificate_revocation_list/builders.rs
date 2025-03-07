// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_client_vpn_client_certificate_revocation_list::_import_client_vpn_client_certificate_revocation_list_output::ImportClientVpnClientCertificateRevocationListOutputBuilder;

pub use crate::operation::import_client_vpn_client_certificate_revocation_list::_import_client_vpn_client_certificate_revocation_list_input::ImportClientVpnClientCertificateRevocationListInputBuilder;

/// Fluent builder constructing a request to `ImportClientVpnClientCertificateRevocationList`.
///
/// <p>Uploads a client certificate revocation list to the specified Client VPN endpoint. Uploading a client certificate revocation list overwrites the existing client certificate revocation list.</p>
/// <p>Uploading a client certificate revocation list resets existing client connections.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ImportClientVpnClientCertificateRevocationListFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::import_client_vpn_client_certificate_revocation_list::builders::ImportClientVpnClientCertificateRevocationListInputBuilder,
}
impl ImportClientVpnClientCertificateRevocationListFluentBuilder {
    /// Creates a new `ImportClientVpnClientCertificateRevocationList`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::import_client_vpn_client_certificate_revocation_list::ImportClientVpnClientCertificateRevocationList, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::import_client_vpn_client_certificate_revocation_list::ImportClientVpnClientCertificateRevocationListError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::import_client_vpn_client_certificate_revocation_list::ImportClientVpnClientCertificateRevocationListOutput, ::aws_smithy_http::result::SdkError<crate::operation::import_client_vpn_client_certificate_revocation_list::ImportClientVpnClientCertificateRevocationListError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::import_client_vpn_client_certificate_revocation_list::ImportClientVpnClientCertificateRevocationListOutput, ::aws_smithy_http::result::SdkError<crate::operation::import_client_vpn_client_certificate_revocation_list::ImportClientVpnClientCertificateRevocationListError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::import_client_vpn_client_certificate_revocation_list::ImportClientVpnClientCertificateRevocationList, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::import_client_vpn_client_certificate_revocation_list::ImportClientVpnClientCertificateRevocationListError>
    >{
        self.customize_middleware().await
    }
    /// <p>The ID of the Client VPN endpoint to which the client certificate revocation list applies.</p>
    pub fn client_vpn_endpoint_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.client_vpn_endpoint_id(input.into());
        self
    }
    /// <p>The ID of the Client VPN endpoint to which the client certificate revocation list applies.</p>
    pub fn set_client_vpn_endpoint_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_client_vpn_endpoint_id(input);
        self
    }
    /// <p>The client certificate revocation list file. For more information, see <a href="https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/cvpn-working-certificates.html#cvpn-working-certificates-generate">Generate a Client Certificate Revocation List</a> in the <i>Client VPN Administrator Guide</i>.</p>
    pub fn certificate_revocation_list(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.certificate_revocation_list(input.into());
        self
    }
    /// <p>The client certificate revocation list file. For more information, see <a href="https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/cvpn-working-certificates.html#cvpn-working-certificates-generate">Generate a Client Certificate Revocation List</a> in the <i>Client VPN Administrator Guide</i>.</p>
    pub fn set_certificate_revocation_list(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_certificate_revocation_list(input);
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
