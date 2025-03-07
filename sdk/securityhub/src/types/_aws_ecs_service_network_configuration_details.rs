// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>For tasks that use the <code>awsvpc</code> networking mode, the VPC subnet and security group configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsEcsServiceNetworkConfigurationDetails {
    /// <p>The VPC subnet and security group configuration.</p>
    #[doc(hidden)]
    pub aws_vpc_configuration: ::std::option::Option<
        crate::types::AwsEcsServiceNetworkConfigurationAwsVpcConfigurationDetails,
    >,
}
impl AwsEcsServiceNetworkConfigurationDetails {
    /// <p>The VPC subnet and security group configuration.</p>
    pub fn aws_vpc_configuration(
        &self,
    ) -> ::std::option::Option<
        &crate::types::AwsEcsServiceNetworkConfigurationAwsVpcConfigurationDetails,
    > {
        self.aws_vpc_configuration.as_ref()
    }
}
impl AwsEcsServiceNetworkConfigurationDetails {
    /// Creates a new builder-style object to manufacture [`AwsEcsServiceNetworkConfigurationDetails`](crate::types::AwsEcsServiceNetworkConfigurationDetails).
    pub fn builder() -> crate::types::builders::AwsEcsServiceNetworkConfigurationDetailsBuilder {
        crate::types::builders::AwsEcsServiceNetworkConfigurationDetailsBuilder::default()
    }
}

/// A builder for [`AwsEcsServiceNetworkConfigurationDetails`](crate::types::AwsEcsServiceNetworkConfigurationDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsEcsServiceNetworkConfigurationDetailsBuilder {
    pub(crate) aws_vpc_configuration: ::std::option::Option<
        crate::types::AwsEcsServiceNetworkConfigurationAwsVpcConfigurationDetails,
    >,
}
impl AwsEcsServiceNetworkConfigurationDetailsBuilder {
    /// <p>The VPC subnet and security group configuration.</p>
    pub fn aws_vpc_configuration(
        mut self,
        input: crate::types::AwsEcsServiceNetworkConfigurationAwsVpcConfigurationDetails,
    ) -> Self {
        self.aws_vpc_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The VPC subnet and security group configuration.</p>
    pub fn set_aws_vpc_configuration(
        mut self,
        input: ::std::option::Option<
            crate::types::AwsEcsServiceNetworkConfigurationAwsVpcConfigurationDetails,
        >,
    ) -> Self {
        self.aws_vpc_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsEcsServiceNetworkConfigurationDetails`](crate::types::AwsEcsServiceNetworkConfigurationDetails).
    pub fn build(self) -> crate::types::AwsEcsServiceNetworkConfigurationDetails {
        crate::types::AwsEcsServiceNetworkConfigurationDetails {
            aws_vpc_configuration: self.aws_vpc_configuration,
        }
    }
}
