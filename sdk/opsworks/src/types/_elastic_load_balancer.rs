// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an Elastic Load Balancing instance.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ElasticLoadBalancer {
    /// <p>The Elastic Load Balancing instance's name.</p>
    #[doc(hidden)]
    pub elastic_load_balancer_name: ::std::option::Option<::std::string::String>,
    /// <p>The instance's AWS region.</p>
    #[doc(hidden)]
    pub region: ::std::option::Option<::std::string::String>,
    /// <p>The instance's public DNS name.</p>
    #[doc(hidden)]
    pub dns_name: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the stack that the instance is associated with.</p>
    #[doc(hidden)]
    pub stack_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the layer that the instance is attached to.</p>
    #[doc(hidden)]
    pub layer_id: ::std::option::Option<::std::string::String>,
    /// <p>The VPC ID.</p>
    #[doc(hidden)]
    pub vpc_id: ::std::option::Option<::std::string::String>,
    /// <p>A list of Availability Zones.</p>
    #[doc(hidden)]
    pub availability_zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A list of subnet IDs, if the stack is running in a VPC.</p>
    #[doc(hidden)]
    pub subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A list of the EC2 instances that the Elastic Load Balancing instance is managing traffic for.</p>
    #[doc(hidden)]
    pub ec2_instance_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ElasticLoadBalancer {
    /// <p>The Elastic Load Balancing instance's name.</p>
    pub fn elastic_load_balancer_name(&self) -> ::std::option::Option<&str> {
        self.elastic_load_balancer_name.as_deref()
    }
    /// <p>The instance's AWS region.</p>
    pub fn region(&self) -> ::std::option::Option<&str> {
        self.region.as_deref()
    }
    /// <p>The instance's public DNS name.</p>
    pub fn dns_name(&self) -> ::std::option::Option<&str> {
        self.dns_name.as_deref()
    }
    /// <p>The ID of the stack that the instance is associated with.</p>
    pub fn stack_id(&self) -> ::std::option::Option<&str> {
        self.stack_id.as_deref()
    }
    /// <p>The ID of the layer that the instance is attached to.</p>
    pub fn layer_id(&self) -> ::std::option::Option<&str> {
        self.layer_id.as_deref()
    }
    /// <p>The VPC ID.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>A list of Availability Zones.</p>
    pub fn availability_zones(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.availability_zones.as_deref()
    }
    /// <p>A list of subnet IDs, if the stack is running in a VPC.</p>
    pub fn subnet_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.subnet_ids.as_deref()
    }
    /// <p>A list of the EC2 instances that the Elastic Load Balancing instance is managing traffic for.</p>
    pub fn ec2_instance_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.ec2_instance_ids.as_deref()
    }
}
impl ElasticLoadBalancer {
    /// Creates a new builder-style object to manufacture [`ElasticLoadBalancer`](crate::types::ElasticLoadBalancer).
    pub fn builder() -> crate::types::builders::ElasticLoadBalancerBuilder {
        crate::types::builders::ElasticLoadBalancerBuilder::default()
    }
}

/// A builder for [`ElasticLoadBalancer`](crate::types::ElasticLoadBalancer).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ElasticLoadBalancerBuilder {
    pub(crate) elastic_load_balancer_name: ::std::option::Option<::std::string::String>,
    pub(crate) region: ::std::option::Option<::std::string::String>,
    pub(crate) dns_name: ::std::option::Option<::std::string::String>,
    pub(crate) stack_id: ::std::option::Option<::std::string::String>,
    pub(crate) layer_id: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
    pub(crate) availability_zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) ec2_instance_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ElasticLoadBalancerBuilder {
    /// <p>The Elastic Load Balancing instance's name.</p>
    pub fn elastic_load_balancer_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.elastic_load_balancer_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Elastic Load Balancing instance's name.</p>
    pub fn set_elastic_load_balancer_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.elastic_load_balancer_name = input;
        self
    }
    /// <p>The instance's AWS region.</p>
    pub fn region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The instance's AWS region.</p>
    pub fn set_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.region = input;
        self
    }
    /// <p>The instance's public DNS name.</p>
    pub fn dns_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.dns_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The instance's public DNS name.</p>
    pub fn set_dns_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.dns_name = input;
        self
    }
    /// <p>The ID of the stack that the instance is associated with.</p>
    pub fn stack_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stack_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the stack that the instance is associated with.</p>
    pub fn set_stack_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stack_id = input;
        self
    }
    /// <p>The ID of the layer that the instance is attached to.</p>
    pub fn layer_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.layer_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the layer that the instance is attached to.</p>
    pub fn set_layer_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.layer_id = input;
        self
    }
    /// <p>The VPC ID.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The VPC ID.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// Appends an item to `availability_zones`.
    ///
    /// To override the contents of this collection use [`set_availability_zones`](Self::set_availability_zones).
    ///
    /// <p>A list of Availability Zones.</p>
    pub fn availability_zones(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.availability_zones.unwrap_or_default();
        v.push(input.into());
        self.availability_zones = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of Availability Zones.</p>
    pub fn set_availability_zones(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.availability_zones = input;
        self
    }
    /// Appends an item to `subnet_ids`.
    ///
    /// To override the contents of this collection use [`set_subnet_ids`](Self::set_subnet_ids).
    ///
    /// <p>A list of subnet IDs, if the stack is running in a VPC.</p>
    pub fn subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.subnet_ids.unwrap_or_default();
        v.push(input.into());
        self.subnet_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of subnet IDs, if the stack is running in a VPC.</p>
    pub fn set_subnet_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.subnet_ids = input;
        self
    }
    /// Appends an item to `ec2_instance_ids`.
    ///
    /// To override the contents of this collection use [`set_ec2_instance_ids`](Self::set_ec2_instance_ids).
    ///
    /// <p>A list of the EC2 instances that the Elastic Load Balancing instance is managing traffic for.</p>
    pub fn ec2_instance_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.ec2_instance_ids.unwrap_or_default();
        v.push(input.into());
        self.ec2_instance_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the EC2 instances that the Elastic Load Balancing instance is managing traffic for.</p>
    pub fn set_ec2_instance_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.ec2_instance_ids = input;
        self
    }
    /// Consumes the builder and constructs a [`ElasticLoadBalancer`](crate::types::ElasticLoadBalancer).
    pub fn build(self) -> crate::types::ElasticLoadBalancer {
        crate::types::ElasticLoadBalancer {
            elastic_load_balancer_name: self.elastic_load_balancer_name,
            region: self.region,
            dns_name: self.dns_name,
            stack_id: self.stack_id,
            layer_id: self.layer_id,
            vpc_id: self.vpc_id,
            availability_zones: self.availability_zones,
            subnet_ids: self.subnet_ids,
            ec2_instance_ids: self.ec2_instance_ids,
        }
    }
}
