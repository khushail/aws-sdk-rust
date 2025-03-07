// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents a local file certificate. The certificate must meet specific requirements and you must have proxy authorization enabled. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/tls.html#virtual-node-tls-prerequisites">Transport Layer Security (TLS)</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VirtualGatewayListenerTlsFileCertificate {
    /// <p>The certificate chain for the certificate.</p>
    #[doc(hidden)]
    pub certificate_chain: ::std::option::Option<::std::string::String>,
    /// <p>The private key for a certificate stored on the file system of the mesh endpoint that the proxy is running on.</p>
    #[doc(hidden)]
    pub private_key: ::std::option::Option<::std::string::String>,
}
impl VirtualGatewayListenerTlsFileCertificate {
    /// <p>The certificate chain for the certificate.</p>
    pub fn certificate_chain(&self) -> ::std::option::Option<&str> {
        self.certificate_chain.as_deref()
    }
    /// <p>The private key for a certificate stored on the file system of the mesh endpoint that the proxy is running on.</p>
    pub fn private_key(&self) -> ::std::option::Option<&str> {
        self.private_key.as_deref()
    }
}
impl VirtualGatewayListenerTlsFileCertificate {
    /// Creates a new builder-style object to manufacture [`VirtualGatewayListenerTlsFileCertificate`](crate::types::VirtualGatewayListenerTlsFileCertificate).
    pub fn builder() -> crate::types::builders::VirtualGatewayListenerTlsFileCertificateBuilder {
        crate::types::builders::VirtualGatewayListenerTlsFileCertificateBuilder::default()
    }
}

/// A builder for [`VirtualGatewayListenerTlsFileCertificate`](crate::types::VirtualGatewayListenerTlsFileCertificate).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VirtualGatewayListenerTlsFileCertificateBuilder {
    pub(crate) certificate_chain: ::std::option::Option<::std::string::String>,
    pub(crate) private_key: ::std::option::Option<::std::string::String>,
}
impl VirtualGatewayListenerTlsFileCertificateBuilder {
    /// <p>The certificate chain for the certificate.</p>
    pub fn certificate_chain(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.certificate_chain = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The certificate chain for the certificate.</p>
    pub fn set_certificate_chain(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.certificate_chain = input;
        self
    }
    /// <p>The private key for a certificate stored on the file system of the mesh endpoint that the proxy is running on.</p>
    pub fn private_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.private_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The private key for a certificate stored on the file system of the mesh endpoint that the proxy is running on.</p>
    pub fn set_private_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.private_key = input;
        self
    }
    /// Consumes the builder and constructs a [`VirtualGatewayListenerTlsFileCertificate`](crate::types::VirtualGatewayListenerTlsFileCertificate).
    pub fn build(self) -> crate::types::VirtualGatewayListenerTlsFileCertificate {
        crate::types::VirtualGatewayListenerTlsFileCertificate {
            certificate_chain: self.certificate_chain,
            private_key: self.private_key,
        }
    }
}
