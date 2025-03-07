// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociateWirelessGatewayFromCertificateInput {
    /// <p>The ID of the resource to update.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
}
impl DisassociateWirelessGatewayFromCertificateInput {
    /// <p>The ID of the resource to update.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl DisassociateWirelessGatewayFromCertificateInput {
    /// Creates a new builder-style object to manufacture [`DisassociateWirelessGatewayFromCertificateInput`](crate::operation::disassociate_wireless_gateway_from_certificate::DisassociateWirelessGatewayFromCertificateInput).
    pub fn builder() -> crate::operation::disassociate_wireless_gateway_from_certificate::builders::DisassociateWirelessGatewayFromCertificateInputBuilder{
        crate::operation::disassociate_wireless_gateway_from_certificate::builders::DisassociateWirelessGatewayFromCertificateInputBuilder::default()
    }
}

/// A builder for [`DisassociateWirelessGatewayFromCertificateInput`](crate::operation::disassociate_wireless_gateway_from_certificate::DisassociateWirelessGatewayFromCertificateInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DisassociateWirelessGatewayFromCertificateInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl DisassociateWirelessGatewayFromCertificateInputBuilder {
    /// <p>The ID of the resource to update.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the resource to update.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Consumes the builder and constructs a [`DisassociateWirelessGatewayFromCertificateInput`](crate::operation::disassociate_wireless_gateway_from_certificate::DisassociateWirelessGatewayFromCertificateInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::disassociate_wireless_gateway_from_certificate::DisassociateWirelessGatewayFromCertificateInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::disassociate_wireless_gateway_from_certificate::DisassociateWirelessGatewayFromCertificateInput {
                id: self.id
                ,
            }
        )
    }
}
