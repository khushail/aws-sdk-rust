// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteIpamResourceDiscoveryInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The IPAM resource discovery ID.</p>
    #[doc(hidden)]
    pub ipam_resource_discovery_id: ::std::option::Option<::std::string::String>,
}
impl DeleteIpamResourceDiscoveryInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The IPAM resource discovery ID.</p>
    pub fn ipam_resource_discovery_id(&self) -> ::std::option::Option<&str> {
        self.ipam_resource_discovery_id.as_deref()
    }
}
impl DeleteIpamResourceDiscoveryInput {
    /// Creates a new builder-style object to manufacture [`DeleteIpamResourceDiscoveryInput`](crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryInput).
    pub fn builder() -> crate::operation::delete_ipam_resource_discovery::builders::DeleteIpamResourceDiscoveryInputBuilder{
        crate::operation::delete_ipam_resource_discovery::builders::DeleteIpamResourceDiscoveryInputBuilder::default()
    }
}

/// A builder for [`DeleteIpamResourceDiscoveryInput`](crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteIpamResourceDiscoveryInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) ipam_resource_discovery_id: ::std::option::Option<::std::string::String>,
}
impl DeleteIpamResourceDiscoveryInputBuilder {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>The IPAM resource discovery ID.</p>
    pub fn ipam_resource_discovery_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.ipam_resource_discovery_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPAM resource discovery ID.</p>
    pub fn set_ipam_resource_discovery_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.ipam_resource_discovery_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteIpamResourceDiscoveryInput`](crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryInput {
                dry_run: self.dry_run,
                ipam_resource_discovery_id: self.ipam_resource_discovery_id,
            },
        )
    }
}
