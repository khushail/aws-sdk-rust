// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for CreateLoadBalancer.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateLoadBalancerInput {
    /// <p>The name of the load balancer.</p>
    /// <p>This name must be unique within your set of load balancers for the region, must have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and cannot begin or end with a hyphen.</p>
    #[doc(hidden)]
    pub load_balancer_name: ::std::option::Option<::std::string::String>,
    /// <p>The listeners.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-listener-config.html">Listeners for Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    #[doc(hidden)]
    pub listeners: ::std::option::Option<::std::vec::Vec<crate::types::Listener>>,
    /// <p>One or more Availability Zones from the same region as the load balancer.</p>
    /// <p>You must specify at least one Availability Zone.</p>
    /// <p>You can add more Availability Zones after you create the load balancer using <code>EnableAvailabilityZonesForLoadBalancer</code>.</p>
    #[doc(hidden)]
    pub availability_zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The IDs of the subnets in your VPC to attach to the load balancer. Specify one subnet per Availability Zone specified in <code>AvailabilityZones</code>.</p>
    #[doc(hidden)]
    pub subnets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The IDs of the security groups to assign to the load balancer.</p>
    #[doc(hidden)]
    pub security_groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The type of a load balancer. Valid only for load balancers in a VPC.</p>
    /// <p>By default, Elastic Load Balancing creates an Internet-facing load balancer with a DNS name that resolves to public IP addresses. For more information about Internet-facing and Internal load balancers, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/userguide/how-elastic-load-balancing-works.html#load-balancer-scheme">Load Balancer Scheme</a> in the <i>Elastic Load Balancing User Guide</i>.</p>
    /// <p>Specify <code>internal</code> to create a load balancer with a DNS name that resolves to private IP addresses.</p>
    #[doc(hidden)]
    pub scheme: ::std::option::Option<::std::string::String>,
    /// <p>A list of tags to assign to the load balancer.</p>
    /// <p>For more information about tagging your load balancer, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/add-remove-tags.html">Tag Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateLoadBalancerInput {
    /// <p>The name of the load balancer.</p>
    /// <p>This name must be unique within your set of load balancers for the region, must have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and cannot begin or end with a hyphen.</p>
    pub fn load_balancer_name(&self) -> ::std::option::Option<&str> {
        self.load_balancer_name.as_deref()
    }
    /// <p>The listeners.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-listener-config.html">Listeners for Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn listeners(&self) -> ::std::option::Option<&[crate::types::Listener]> {
        self.listeners.as_deref()
    }
    /// <p>One or more Availability Zones from the same region as the load balancer.</p>
    /// <p>You must specify at least one Availability Zone.</p>
    /// <p>You can add more Availability Zones after you create the load balancer using <code>EnableAvailabilityZonesForLoadBalancer</code>.</p>
    pub fn availability_zones(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.availability_zones.as_deref()
    }
    /// <p>The IDs of the subnets in your VPC to attach to the load balancer. Specify one subnet per Availability Zone specified in <code>AvailabilityZones</code>.</p>
    pub fn subnets(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.subnets.as_deref()
    }
    /// <p>The IDs of the security groups to assign to the load balancer.</p>
    pub fn security_groups(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.security_groups.as_deref()
    }
    /// <p>The type of a load balancer. Valid only for load balancers in a VPC.</p>
    /// <p>By default, Elastic Load Balancing creates an Internet-facing load balancer with a DNS name that resolves to public IP addresses. For more information about Internet-facing and Internal load balancers, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/userguide/how-elastic-load-balancing-works.html#load-balancer-scheme">Load Balancer Scheme</a> in the <i>Elastic Load Balancing User Guide</i>.</p>
    /// <p>Specify <code>internal</code> to create a load balancer with a DNS name that resolves to private IP addresses.</p>
    pub fn scheme(&self) -> ::std::option::Option<&str> {
        self.scheme.as_deref()
    }
    /// <p>A list of tags to assign to the load balancer.</p>
    /// <p>For more information about tagging your load balancer, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/add-remove-tags.html">Tag Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl CreateLoadBalancerInput {
    /// Creates a new builder-style object to manufacture [`CreateLoadBalancerInput`](crate::operation::create_load_balancer::CreateLoadBalancerInput).
    pub fn builder(
    ) -> crate::operation::create_load_balancer::builders::CreateLoadBalancerInputBuilder {
        crate::operation::create_load_balancer::builders::CreateLoadBalancerInputBuilder::default()
    }
}

/// A builder for [`CreateLoadBalancerInput`](crate::operation::create_load_balancer::CreateLoadBalancerInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateLoadBalancerInputBuilder {
    pub(crate) load_balancer_name: ::std::option::Option<::std::string::String>,
    pub(crate) listeners: ::std::option::Option<::std::vec::Vec<crate::types::Listener>>,
    pub(crate) availability_zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) subnets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) security_groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) scheme: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateLoadBalancerInputBuilder {
    /// <p>The name of the load balancer.</p>
    /// <p>This name must be unique within your set of load balancers for the region, must have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and cannot begin or end with a hyphen.</p>
    pub fn load_balancer_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.load_balancer_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the load balancer.</p>
    /// <p>This name must be unique within your set of load balancers for the region, must have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and cannot begin or end with a hyphen.</p>
    pub fn set_load_balancer_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.load_balancer_name = input;
        self
    }
    /// Appends an item to `listeners`.
    ///
    /// To override the contents of this collection use [`set_listeners`](Self::set_listeners).
    ///
    /// <p>The listeners.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-listener-config.html">Listeners for Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn listeners(mut self, input: crate::types::Listener) -> Self {
        let mut v = self.listeners.unwrap_or_default();
        v.push(input);
        self.listeners = ::std::option::Option::Some(v);
        self
    }
    /// <p>The listeners.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-listener-config.html">Listeners for Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn set_listeners(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Listener>>,
    ) -> Self {
        self.listeners = input;
        self
    }
    /// Appends an item to `availability_zones`.
    ///
    /// To override the contents of this collection use [`set_availability_zones`](Self::set_availability_zones).
    ///
    /// <p>One or more Availability Zones from the same region as the load balancer.</p>
    /// <p>You must specify at least one Availability Zone.</p>
    /// <p>You can add more Availability Zones after you create the load balancer using <code>EnableAvailabilityZonesForLoadBalancer</code>.</p>
    pub fn availability_zones(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.availability_zones.unwrap_or_default();
        v.push(input.into());
        self.availability_zones = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more Availability Zones from the same region as the load balancer.</p>
    /// <p>You must specify at least one Availability Zone.</p>
    /// <p>You can add more Availability Zones after you create the load balancer using <code>EnableAvailabilityZonesForLoadBalancer</code>.</p>
    pub fn set_availability_zones(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.availability_zones = input;
        self
    }
    /// Appends an item to `subnets`.
    ///
    /// To override the contents of this collection use [`set_subnets`](Self::set_subnets).
    ///
    /// <p>The IDs of the subnets in your VPC to attach to the load balancer. Specify one subnet per Availability Zone specified in <code>AvailabilityZones</code>.</p>
    pub fn subnets(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.subnets.unwrap_or_default();
        v.push(input.into());
        self.subnets = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the subnets in your VPC to attach to the load balancer. Specify one subnet per Availability Zone specified in <code>AvailabilityZones</code>.</p>
    pub fn set_subnets(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.subnets = input;
        self
    }
    /// Appends an item to `security_groups`.
    ///
    /// To override the contents of this collection use [`set_security_groups`](Self::set_security_groups).
    ///
    /// <p>The IDs of the security groups to assign to the load balancer.</p>
    pub fn security_groups(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.security_groups.unwrap_or_default();
        v.push(input.into());
        self.security_groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the security groups to assign to the load balancer.</p>
    pub fn set_security_groups(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.security_groups = input;
        self
    }
    /// <p>The type of a load balancer. Valid only for load balancers in a VPC.</p>
    /// <p>By default, Elastic Load Balancing creates an Internet-facing load balancer with a DNS name that resolves to public IP addresses. For more information about Internet-facing and Internal load balancers, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/userguide/how-elastic-load-balancing-works.html#load-balancer-scheme">Load Balancer Scheme</a> in the <i>Elastic Load Balancing User Guide</i>.</p>
    /// <p>Specify <code>internal</code> to create a load balancer with a DNS name that resolves to private IP addresses.</p>
    pub fn scheme(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.scheme = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of a load balancer. Valid only for load balancers in a VPC.</p>
    /// <p>By default, Elastic Load Balancing creates an Internet-facing load balancer with a DNS name that resolves to public IP addresses. For more information about Internet-facing and Internal load balancers, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/userguide/how-elastic-load-balancing-works.html#load-balancer-scheme">Load Balancer Scheme</a> in the <i>Elastic Load Balancing User Guide</i>.</p>
    /// <p>Specify <code>internal</code> to create a load balancer with a DNS name that resolves to private IP addresses.</p>
    pub fn set_scheme(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.scheme = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tags to assign to the load balancer.</p>
    /// <p>For more information about tagging your load balancer, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/add-remove-tags.html">Tag Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of tags to assign to the load balancer.</p>
    /// <p>For more information about tagging your load balancer, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/add-remove-tags.html">Tag Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateLoadBalancerInput`](crate::operation::create_load_balancer::CreateLoadBalancerInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_load_balancer::CreateLoadBalancerInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_load_balancer::CreateLoadBalancerInput {
                load_balancer_name: self.load_balancer_name,
                listeners: self.listeners,
                availability_zones: self.availability_zones,
                subnets: self.subnets,
                security_groups: self.security_groups,
                scheme: self.scheme,
                tags: self.tags,
            },
        )
    }
}
