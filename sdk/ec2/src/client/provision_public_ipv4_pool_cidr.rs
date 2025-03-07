// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ProvisionPublicIpv4PoolCidr`](crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrFluentBuilder::set_dry_run): <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`ipam_pool_id(impl ::std::convert::Into<String>)`](crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrFluentBuilder::ipam_pool_id) / [`set_ipam_pool_id(Option<String>)`](crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrFluentBuilder::set_ipam_pool_id): <p>The ID of the IPAM pool you would like to use to allocate this CIDR.</p>
    ///   - [`pool_id(impl ::std::convert::Into<String>)`](crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrFluentBuilder::pool_id) / [`set_pool_id(Option<String>)`](crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrFluentBuilder::set_pool_id): <p>The ID of the public IPv4 pool you would like to use for this CIDR.</p>
    ///   - [`netmask_length(i32)`](crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrFluentBuilder::netmask_length) / [`set_netmask_length(Option<i32>)`](crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrFluentBuilder::set_netmask_length): <p>The netmask length of the CIDR you would like to allocate to the public IPv4 pool.</p>
    /// - On success, responds with [`ProvisionPublicIpv4PoolCidrOutput`](crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrOutput) with field(s):
    ///   - [`pool_id(Option<String>)`](crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrOutput::pool_id): <p>The ID of the pool that you want to provision the CIDR to.</p>
    ///   - [`pool_address_range(Option<PublicIpv4PoolRange>)`](crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrOutput::pool_address_range): <p>Information about the address range of the public IPv4 pool.</p>
    /// - On failure, responds with [`SdkError<ProvisionPublicIpv4PoolCidrError>`](crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrError)
    pub fn provision_public_ipv4_pool_cidr(&self) -> crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrFluentBuilder{
        crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrFluentBuilder::new(self.handle.clone())
    }
}
