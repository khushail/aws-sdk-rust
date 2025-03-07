// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeInboundDmarcSettingsInput {
    /// <p>Lists the ID of the given organization.</p>
    #[doc(hidden)]
    pub organization_id: ::std::option::Option<::std::string::String>,
}
impl DescribeInboundDmarcSettingsInput {
    /// <p>Lists the ID of the given organization.</p>
    pub fn organization_id(&self) -> ::std::option::Option<&str> {
        self.organization_id.as_deref()
    }
}
impl DescribeInboundDmarcSettingsInput {
    /// Creates a new builder-style object to manufacture [`DescribeInboundDmarcSettingsInput`](crate::operation::describe_inbound_dmarc_settings::DescribeInboundDmarcSettingsInput).
    pub fn builder() -> crate::operation::describe_inbound_dmarc_settings::builders::DescribeInboundDmarcSettingsInputBuilder{
        crate::operation::describe_inbound_dmarc_settings::builders::DescribeInboundDmarcSettingsInputBuilder::default()
    }
}

/// A builder for [`DescribeInboundDmarcSettingsInput`](crate::operation::describe_inbound_dmarc_settings::DescribeInboundDmarcSettingsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeInboundDmarcSettingsInputBuilder {
    pub(crate) organization_id: ::std::option::Option<::std::string::String>,
}
impl DescribeInboundDmarcSettingsInputBuilder {
    /// <p>Lists the ID of the given organization.</p>
    pub fn organization_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.organization_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Lists the ID of the given organization.</p>
    pub fn set_organization_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.organization_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeInboundDmarcSettingsInput`](crate::operation::describe_inbound_dmarc_settings::DescribeInboundDmarcSettingsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_inbound_dmarc_settings::DescribeInboundDmarcSettingsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_inbound_dmarc_settings::DescribeInboundDmarcSettingsInput {
                organization_id: self.organization_id,
            },
        )
    }
}
