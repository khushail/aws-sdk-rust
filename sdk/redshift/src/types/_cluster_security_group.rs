// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a security group.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ClusterSecurityGroup {
    /// <p>The name of the cluster security group to which the operation was applied.</p>
    #[doc(hidden)]
    pub cluster_security_group_name: ::std::option::Option<::std::string::String>,
    /// <p>A description of the security group.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>A list of EC2 security groups that are permitted to access clusters associated with this cluster security group.</p>
    #[doc(hidden)]
    pub ec2_security_groups: ::std::option::Option<::std::vec::Vec<crate::types::Ec2SecurityGroup>>,
    /// <p>A list of IP ranges (CIDR blocks) that are permitted to access clusters associated with this cluster security group.</p>
    #[doc(hidden)]
    pub ip_ranges: ::std::option::Option<::std::vec::Vec<crate::types::IpRange>>,
    /// <p>The list of tags for the cluster security group.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl ClusterSecurityGroup {
    /// <p>The name of the cluster security group to which the operation was applied.</p>
    pub fn cluster_security_group_name(&self) -> ::std::option::Option<&str> {
        self.cluster_security_group_name.as_deref()
    }
    /// <p>A description of the security group.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>A list of EC2 security groups that are permitted to access clusters associated with this cluster security group.</p>
    pub fn ec2_security_groups(&self) -> ::std::option::Option<&[crate::types::Ec2SecurityGroup]> {
        self.ec2_security_groups.as_deref()
    }
    /// <p>A list of IP ranges (CIDR blocks) that are permitted to access clusters associated with this cluster security group.</p>
    pub fn ip_ranges(&self) -> ::std::option::Option<&[crate::types::IpRange]> {
        self.ip_ranges.as_deref()
    }
    /// <p>The list of tags for the cluster security group.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl ClusterSecurityGroup {
    /// Creates a new builder-style object to manufacture [`ClusterSecurityGroup`](crate::types::ClusterSecurityGroup).
    pub fn builder() -> crate::types::builders::ClusterSecurityGroupBuilder {
        crate::types::builders::ClusterSecurityGroupBuilder::default()
    }
}

/// A builder for [`ClusterSecurityGroup`](crate::types::ClusterSecurityGroup).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ClusterSecurityGroupBuilder {
    pub(crate) cluster_security_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) ec2_security_groups:
        ::std::option::Option<::std::vec::Vec<crate::types::Ec2SecurityGroup>>,
    pub(crate) ip_ranges: ::std::option::Option<::std::vec::Vec<crate::types::IpRange>>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl ClusterSecurityGroupBuilder {
    /// <p>The name of the cluster security group to which the operation was applied.</p>
    pub fn cluster_security_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.cluster_security_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the cluster security group to which the operation was applied.</p>
    pub fn set_cluster_security_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.cluster_security_group_name = input;
        self
    }
    /// <p>A description of the security group.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the security group.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Appends an item to `ec2_security_groups`.
    ///
    /// To override the contents of this collection use [`set_ec2_security_groups`](Self::set_ec2_security_groups).
    ///
    /// <p>A list of EC2 security groups that are permitted to access clusters associated with this cluster security group.</p>
    pub fn ec2_security_groups(mut self, input: crate::types::Ec2SecurityGroup) -> Self {
        let mut v = self.ec2_security_groups.unwrap_or_default();
        v.push(input);
        self.ec2_security_groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of EC2 security groups that are permitted to access clusters associated with this cluster security group.</p>
    pub fn set_ec2_security_groups(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Ec2SecurityGroup>>,
    ) -> Self {
        self.ec2_security_groups = input;
        self
    }
    /// Appends an item to `ip_ranges`.
    ///
    /// To override the contents of this collection use [`set_ip_ranges`](Self::set_ip_ranges).
    ///
    /// <p>A list of IP ranges (CIDR blocks) that are permitted to access clusters associated with this cluster security group.</p>
    pub fn ip_ranges(mut self, input: crate::types::IpRange) -> Self {
        let mut v = self.ip_ranges.unwrap_or_default();
        v.push(input);
        self.ip_ranges = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of IP ranges (CIDR blocks) that are permitted to access clusters associated with this cluster security group.</p>
    pub fn set_ip_ranges(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::IpRange>>,
    ) -> Self {
        self.ip_ranges = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The list of tags for the cluster security group.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of tags for the cluster security group.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`ClusterSecurityGroup`](crate::types::ClusterSecurityGroup).
    pub fn build(self) -> crate::types::ClusterSecurityGroup {
        crate::types::ClusterSecurityGroup {
            cluster_security_group_name: self.cluster_security_group_name,
            description: self.description,
            ec2_security_groups: self.ec2_security_groups,
            ip_ranges: self.ip_ranges,
            tags: self.tags,
        }
    }
}
