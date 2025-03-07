// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartVirtualMachinesMetadataSyncInput {
    /// <p>The Amazon Resource Name (ARN) of the hypervisor.</p>
    #[doc(hidden)]
    pub hypervisor_arn: ::std::option::Option<::std::string::String>,
}
impl StartVirtualMachinesMetadataSyncInput {
    /// <p>The Amazon Resource Name (ARN) of the hypervisor.</p>
    pub fn hypervisor_arn(&self) -> ::std::option::Option<&str> {
        self.hypervisor_arn.as_deref()
    }
}
impl StartVirtualMachinesMetadataSyncInput {
    /// Creates a new builder-style object to manufacture [`StartVirtualMachinesMetadataSyncInput`](crate::operation::start_virtual_machines_metadata_sync::StartVirtualMachinesMetadataSyncInput).
    pub fn builder() -> crate::operation::start_virtual_machines_metadata_sync::builders::StartVirtualMachinesMetadataSyncInputBuilder{
        crate::operation::start_virtual_machines_metadata_sync::builders::StartVirtualMachinesMetadataSyncInputBuilder::default()
    }
}

/// A builder for [`StartVirtualMachinesMetadataSyncInput`](crate::operation::start_virtual_machines_metadata_sync::StartVirtualMachinesMetadataSyncInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StartVirtualMachinesMetadataSyncInputBuilder {
    pub(crate) hypervisor_arn: ::std::option::Option<::std::string::String>,
}
impl StartVirtualMachinesMetadataSyncInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the hypervisor.</p>
    pub fn hypervisor_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.hypervisor_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the hypervisor.</p>
    pub fn set_hypervisor_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.hypervisor_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`StartVirtualMachinesMetadataSyncInput`](crate::operation::start_virtual_machines_metadata_sync::StartVirtualMachinesMetadataSyncInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::start_virtual_machines_metadata_sync::StartVirtualMachinesMetadataSyncInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::start_virtual_machines_metadata_sync::StartVirtualMachinesMetadataSyncInput {
                hypervisor_arn: self.hypervisor_arn
                ,
            }
        )
    }
}
