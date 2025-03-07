// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AuthorizeSecurityGroupEgressInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the security group.</p>
    #[doc(hidden)]
    pub group_id: ::std::option::Option<::std::string::String>,
    /// <p>The sets of IP permissions. You can't specify a destination security group and a CIDR IP address range in the same set of permissions.</p>
    #[doc(hidden)]
    pub ip_permissions: ::std::option::Option<::std::vec::Vec<crate::types::IpPermission>>,
    /// <p>The tags applied to the security group rule.</p>
    #[doc(hidden)]
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>Not supported. Use a set of IP permissions to specify the CIDR.</p>
    #[doc(hidden)]
    pub cidr_ip: ::std::option::Option<::std::string::String>,
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    #[doc(hidden)]
    pub from_port: ::std::option::Option<i32>,
    /// <p>Not supported. Use a set of IP permissions to specify the protocol name or number.</p>
    #[doc(hidden)]
    pub ip_protocol: ::std::option::Option<::std::string::String>,
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    #[doc(hidden)]
    pub to_port: ::std::option::Option<i32>,
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    #[doc(hidden)]
    pub source_security_group_name: ::std::option::Option<::std::string::String>,
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    #[doc(hidden)]
    pub source_security_group_owner_id: ::std::option::Option<::std::string::String>,
}
impl AuthorizeSecurityGroupEgressInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the security group.</p>
    pub fn group_id(&self) -> ::std::option::Option<&str> {
        self.group_id.as_deref()
    }
    /// <p>The sets of IP permissions. You can't specify a destination security group and a CIDR IP address range in the same set of permissions.</p>
    pub fn ip_permissions(&self) -> ::std::option::Option<&[crate::types::IpPermission]> {
        self.ip_permissions.as_deref()
    }
    /// <p>The tags applied to the security group rule.</p>
    pub fn tag_specifications(&self) -> ::std::option::Option<&[crate::types::TagSpecification]> {
        self.tag_specifications.as_deref()
    }
    /// <p>Not supported. Use a set of IP permissions to specify the CIDR.</p>
    pub fn cidr_ip(&self) -> ::std::option::Option<&str> {
        self.cidr_ip.as_deref()
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn from_port(&self) -> ::std::option::Option<i32> {
        self.from_port
    }
    /// <p>Not supported. Use a set of IP permissions to specify the protocol name or number.</p>
    pub fn ip_protocol(&self) -> ::std::option::Option<&str> {
        self.ip_protocol.as_deref()
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn to_port(&self) -> ::std::option::Option<i32> {
        self.to_port
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn source_security_group_name(&self) -> ::std::option::Option<&str> {
        self.source_security_group_name.as_deref()
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn source_security_group_owner_id(&self) -> ::std::option::Option<&str> {
        self.source_security_group_owner_id.as_deref()
    }
}
impl AuthorizeSecurityGroupEgressInput {
    /// Creates a new builder-style object to manufacture [`AuthorizeSecurityGroupEgressInput`](crate::operation::authorize_security_group_egress::AuthorizeSecurityGroupEgressInput).
    pub fn builder() -> crate::operation::authorize_security_group_egress::builders::AuthorizeSecurityGroupEgressInputBuilder{
        crate::operation::authorize_security_group_egress::builders::AuthorizeSecurityGroupEgressInputBuilder::default()
    }
}

/// A builder for [`AuthorizeSecurityGroupEgressInput`](crate::operation::authorize_security_group_egress::AuthorizeSecurityGroupEgressInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AuthorizeSecurityGroupEgressInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) group_id: ::std::option::Option<::std::string::String>,
    pub(crate) ip_permissions: ::std::option::Option<::std::vec::Vec<crate::types::IpPermission>>,
    pub(crate) tag_specifications:
        ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) cidr_ip: ::std::option::Option<::std::string::String>,
    pub(crate) from_port: ::std::option::Option<i32>,
    pub(crate) ip_protocol: ::std::option::Option<::std::string::String>,
    pub(crate) to_port: ::std::option::Option<i32>,
    pub(crate) source_security_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) source_security_group_owner_id: ::std::option::Option<::std::string::String>,
}
impl AuthorizeSecurityGroupEgressInputBuilder {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>The ID of the security group.</p>
    pub fn group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the security group.</p>
    pub fn set_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_id = input;
        self
    }
    /// Appends an item to `ip_permissions`.
    ///
    /// To override the contents of this collection use [`set_ip_permissions`](Self::set_ip_permissions).
    ///
    /// <p>The sets of IP permissions. You can't specify a destination security group and a CIDR IP address range in the same set of permissions.</p>
    pub fn ip_permissions(mut self, input: crate::types::IpPermission) -> Self {
        let mut v = self.ip_permissions.unwrap_or_default();
        v.push(input);
        self.ip_permissions = ::std::option::Option::Some(v);
        self
    }
    /// <p>The sets of IP permissions. You can't specify a destination security group and a CIDR IP address range in the same set of permissions.</p>
    pub fn set_ip_permissions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::IpPermission>>,
    ) -> Self {
        self.ip_permissions = input;
        self
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags applied to the security group rule.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags applied to the security group rule.</p>
    pub fn set_tag_specifications(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the CIDR.</p>
    pub fn cidr_ip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cidr_ip = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the CIDR.</p>
    pub fn set_cidr_ip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cidr_ip = input;
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn from_port(mut self, input: i32) -> Self {
        self.from_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn set_from_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.from_port = input;
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the protocol name or number.</p>
    pub fn ip_protocol(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ip_protocol = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the protocol name or number.</p>
    pub fn set_ip_protocol(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ip_protocol = input;
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn to_port(mut self, input: i32) -> Self {
        self.to_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn set_to_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.to_port = input;
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn source_security_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.source_security_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn set_source_security_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.source_security_group_name = input;
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn source_security_group_owner_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.source_security_group_owner_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn set_source_security_group_owner_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.source_security_group_owner_id = input;
        self
    }
    /// Consumes the builder and constructs a [`AuthorizeSecurityGroupEgressInput`](crate::operation::authorize_security_group_egress::AuthorizeSecurityGroupEgressInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::authorize_security_group_egress::AuthorizeSecurityGroupEgressInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::authorize_security_group_egress::AuthorizeSecurityGroupEgressInput {
                dry_run: self.dry_run,
                group_id: self.group_id,
                ip_permissions: self.ip_permissions,
                tag_specifications: self.tag_specifications,
                cidr_ip: self.cidr_ip,
                from_port: self.from_port,
                ip_protocol: self.ip_protocol,
                to_port: self.to_port,
                source_security_group_name: self.source_security_group_name,
                source_security_group_owner_id: self.source_security_group_owner_id,
            },
        )
    }
}
