// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutHypervisorPropertyMappingsInput {
    /// <p>The Amazon Resource Name (ARN) of the hypervisor.</p>
    #[doc(hidden)]
    pub hypervisor_arn: ::std::option::Option<::std::string::String>,
    /// <p>This action requests the mappings of on-premises VMware tags to the Amazon Web Services tags.</p>
    #[doc(hidden)]
    pub vmware_to_aws_tag_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::VmwareToAwsTagMapping>>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role.</p>
    #[doc(hidden)]
    pub iam_role_arn: ::std::option::Option<::std::string::String>,
}
impl PutHypervisorPropertyMappingsInput {
    /// <p>The Amazon Resource Name (ARN) of the hypervisor.</p>
    pub fn hypervisor_arn(&self) -> ::std::option::Option<&str> {
        self.hypervisor_arn.as_deref()
    }
    /// <p>This action requests the mappings of on-premises VMware tags to the Amazon Web Services tags.</p>
    pub fn vmware_to_aws_tag_mappings(
        &self,
    ) -> ::std::option::Option<&[crate::types::VmwareToAwsTagMapping]> {
        self.vmware_to_aws_tag_mappings.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role.</p>
    pub fn iam_role_arn(&self) -> ::std::option::Option<&str> {
        self.iam_role_arn.as_deref()
    }
}
impl PutHypervisorPropertyMappingsInput {
    /// Creates a new builder-style object to manufacture [`PutHypervisorPropertyMappingsInput`](crate::operation::put_hypervisor_property_mappings::PutHypervisorPropertyMappingsInput).
    pub fn builder() -> crate::operation::put_hypervisor_property_mappings::builders::PutHypervisorPropertyMappingsInputBuilder{
        crate::operation::put_hypervisor_property_mappings::builders::PutHypervisorPropertyMappingsInputBuilder::default()
    }
}

/// A builder for [`PutHypervisorPropertyMappingsInput`](crate::operation::put_hypervisor_property_mappings::PutHypervisorPropertyMappingsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PutHypervisorPropertyMappingsInputBuilder {
    pub(crate) hypervisor_arn: ::std::option::Option<::std::string::String>,
    pub(crate) vmware_to_aws_tag_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::VmwareToAwsTagMapping>>,
    pub(crate) iam_role_arn: ::std::option::Option<::std::string::String>,
}
impl PutHypervisorPropertyMappingsInputBuilder {
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
    /// Appends an item to `vmware_to_aws_tag_mappings`.
    ///
    /// To override the contents of this collection use [`set_vmware_to_aws_tag_mappings`](Self::set_vmware_to_aws_tag_mappings).
    ///
    /// <p>This action requests the mappings of on-premises VMware tags to the Amazon Web Services tags.</p>
    pub fn vmware_to_aws_tag_mappings(
        mut self,
        input: crate::types::VmwareToAwsTagMapping,
    ) -> Self {
        let mut v = self.vmware_to_aws_tag_mappings.unwrap_or_default();
        v.push(input);
        self.vmware_to_aws_tag_mappings = ::std::option::Option::Some(v);
        self
    }
    /// <p>This action requests the mappings of on-premises VMware tags to the Amazon Web Services tags.</p>
    pub fn set_vmware_to_aws_tag_mappings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::VmwareToAwsTagMapping>>,
    ) -> Self {
        self.vmware_to_aws_tag_mappings = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role.</p>
    pub fn iam_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.iam_role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role.</p>
    pub fn set_iam_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.iam_role_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`PutHypervisorPropertyMappingsInput`](crate::operation::put_hypervisor_property_mappings::PutHypervisorPropertyMappingsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_hypervisor_property_mappings::PutHypervisorPropertyMappingsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::put_hypervisor_property_mappings::PutHypervisorPropertyMappingsInput {
                hypervisor_arn: self.hypervisor_arn
                ,
                vmware_to_aws_tag_mappings: self.vmware_to_aws_tag_mappings
                ,
                iam_role_arn: self.iam_role_arn
                ,
            }
        )
    }
}
