// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The Kubernetes network configuration for the cluster. The response contains a value for <b>serviceIpv6Cidr</b> or <b>serviceIpv4Cidr</b>, but not both. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct KubernetesNetworkConfigResponse {
    /// <p>The CIDR block that Kubernetes pod and service IP addresses are assigned from. Kubernetes assigns addresses from an IPv4 CIDR block assigned to a subnet that the node is in. If you didn't specify a CIDR block when you created the cluster, then Kubernetes assigns addresses from either the <code>10.100.0.0/16</code> or <code>172.20.0.0/16</code> CIDR blocks. If this was specified, then it was specified when the cluster was created and it can't be changed.</p>
    #[doc(hidden)]
    pub service_ipv4_cidr: ::std::option::Option<::std::string::String>,
    /// <p>The CIDR block that Kubernetes pod and service IP addresses are assigned from if you created a 1.21 or later cluster with version 1.10.1 or later of the Amazon VPC CNI add-on and specified <code>ipv6</code> for <b>ipFamily</b> when you created the cluster. Kubernetes assigns service addresses from the unique local address range (<code>fc00::/7</code>) because you can't specify a custom IPv6 CIDR block when you create the cluster.</p>
    #[doc(hidden)]
    pub service_ipv6_cidr: ::std::option::Option<::std::string::String>,
    /// <p>The IP family used to assign Kubernetes pod and service IP addresses. The IP family is always <code>ipv4</code>, unless you have a <code>1.21</code> or later cluster running version 1.10.1 or later of the Amazon VPC CNI add-on and specified <code>ipv6</code> when you created the cluster. </p>
    #[doc(hidden)]
    pub ip_family: ::std::option::Option<crate::types::IpFamily>,
}
impl KubernetesNetworkConfigResponse {
    /// <p>The CIDR block that Kubernetes pod and service IP addresses are assigned from. Kubernetes assigns addresses from an IPv4 CIDR block assigned to a subnet that the node is in. If you didn't specify a CIDR block when you created the cluster, then Kubernetes assigns addresses from either the <code>10.100.0.0/16</code> or <code>172.20.0.0/16</code> CIDR blocks. If this was specified, then it was specified when the cluster was created and it can't be changed.</p>
    pub fn service_ipv4_cidr(&self) -> ::std::option::Option<&str> {
        self.service_ipv4_cidr.as_deref()
    }
    /// <p>The CIDR block that Kubernetes pod and service IP addresses are assigned from if you created a 1.21 or later cluster with version 1.10.1 or later of the Amazon VPC CNI add-on and specified <code>ipv6</code> for <b>ipFamily</b> when you created the cluster. Kubernetes assigns service addresses from the unique local address range (<code>fc00::/7</code>) because you can't specify a custom IPv6 CIDR block when you create the cluster.</p>
    pub fn service_ipv6_cidr(&self) -> ::std::option::Option<&str> {
        self.service_ipv6_cidr.as_deref()
    }
    /// <p>The IP family used to assign Kubernetes pod and service IP addresses. The IP family is always <code>ipv4</code>, unless you have a <code>1.21</code> or later cluster running version 1.10.1 or later of the Amazon VPC CNI add-on and specified <code>ipv6</code> when you created the cluster. </p>
    pub fn ip_family(&self) -> ::std::option::Option<&crate::types::IpFamily> {
        self.ip_family.as_ref()
    }
}
impl KubernetesNetworkConfigResponse {
    /// Creates a new builder-style object to manufacture [`KubernetesNetworkConfigResponse`](crate::types::KubernetesNetworkConfigResponse).
    pub fn builder() -> crate::types::builders::KubernetesNetworkConfigResponseBuilder {
        crate::types::builders::KubernetesNetworkConfigResponseBuilder::default()
    }
}

/// A builder for [`KubernetesNetworkConfigResponse`](crate::types::KubernetesNetworkConfigResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct KubernetesNetworkConfigResponseBuilder {
    pub(crate) service_ipv4_cidr: ::std::option::Option<::std::string::String>,
    pub(crate) service_ipv6_cidr: ::std::option::Option<::std::string::String>,
    pub(crate) ip_family: ::std::option::Option<crate::types::IpFamily>,
}
impl KubernetesNetworkConfigResponseBuilder {
    /// <p>The CIDR block that Kubernetes pod and service IP addresses are assigned from. Kubernetes assigns addresses from an IPv4 CIDR block assigned to a subnet that the node is in. If you didn't specify a CIDR block when you created the cluster, then Kubernetes assigns addresses from either the <code>10.100.0.0/16</code> or <code>172.20.0.0/16</code> CIDR blocks. If this was specified, then it was specified when the cluster was created and it can't be changed.</p>
    pub fn service_ipv4_cidr(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.service_ipv4_cidr = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The CIDR block that Kubernetes pod and service IP addresses are assigned from. Kubernetes assigns addresses from an IPv4 CIDR block assigned to a subnet that the node is in. If you didn't specify a CIDR block when you created the cluster, then Kubernetes assigns addresses from either the <code>10.100.0.0/16</code> or <code>172.20.0.0/16</code> CIDR blocks. If this was specified, then it was specified when the cluster was created and it can't be changed.</p>
    pub fn set_service_ipv4_cidr(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.service_ipv4_cidr = input;
        self
    }
    /// <p>The CIDR block that Kubernetes pod and service IP addresses are assigned from if you created a 1.21 or later cluster with version 1.10.1 or later of the Amazon VPC CNI add-on and specified <code>ipv6</code> for <b>ipFamily</b> when you created the cluster. Kubernetes assigns service addresses from the unique local address range (<code>fc00::/7</code>) because you can't specify a custom IPv6 CIDR block when you create the cluster.</p>
    pub fn service_ipv6_cidr(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.service_ipv6_cidr = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The CIDR block that Kubernetes pod and service IP addresses are assigned from if you created a 1.21 or later cluster with version 1.10.1 or later of the Amazon VPC CNI add-on and specified <code>ipv6</code> for <b>ipFamily</b> when you created the cluster. Kubernetes assigns service addresses from the unique local address range (<code>fc00::/7</code>) because you can't specify a custom IPv6 CIDR block when you create the cluster.</p>
    pub fn set_service_ipv6_cidr(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.service_ipv6_cidr = input;
        self
    }
    /// <p>The IP family used to assign Kubernetes pod and service IP addresses. The IP family is always <code>ipv4</code>, unless you have a <code>1.21</code> or later cluster running version 1.10.1 or later of the Amazon VPC CNI add-on and specified <code>ipv6</code> when you created the cluster. </p>
    pub fn ip_family(mut self, input: crate::types::IpFamily) -> Self {
        self.ip_family = ::std::option::Option::Some(input);
        self
    }
    /// <p>The IP family used to assign Kubernetes pod and service IP addresses. The IP family is always <code>ipv4</code>, unless you have a <code>1.21</code> or later cluster running version 1.10.1 or later of the Amazon VPC CNI add-on and specified <code>ipv6</code> when you created the cluster. </p>
    pub fn set_ip_family(mut self, input: ::std::option::Option<crate::types::IpFamily>) -> Self {
        self.ip_family = input;
        self
    }
    /// Consumes the builder and constructs a [`KubernetesNetworkConfigResponse`](crate::types::KubernetesNetworkConfigResponse).
    pub fn build(self) -> crate::types::KubernetesNetworkConfigResponse {
        crate::types::KubernetesNetworkConfigResponse {
            service_ipv4_cidr: self.service_ipv4_cidr,
            service_ipv6_cidr: self.service_ipv6_cidr,
            ip_family: self.ip_family,
        }
    }
}
