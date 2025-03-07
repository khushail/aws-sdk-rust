// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a deployment configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeploymentConfigInfo {
    /// <p>The deployment configuration ID.</p>
    #[doc(hidden)]
    pub deployment_config_id: ::std::option::Option<::std::string::String>,
    /// <p>The deployment configuration name.</p>
    #[doc(hidden)]
    pub deployment_config_name: ::std::option::Option<::std::string::String>,
    /// <p>Information about the number or percentage of minimum healthy instance.</p>
    #[doc(hidden)]
    pub minimum_healthy_hosts: ::std::option::Option<crate::types::MinimumHealthyHosts>,
    /// <p>The time at which the deployment configuration was created.</p>
    #[doc(hidden)]
    pub create_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The destination platform type for the deployment (<code>Lambda</code>, <code>Server</code>, or <code>ECS</code>).</p>
    #[doc(hidden)]
    pub compute_platform: ::std::option::Option<crate::types::ComputePlatform>,
    /// <p>The configuration that specifies how the deployment traffic is routed. Used for deployments with a Lambda or Amazon ECS compute platform only.</p>
    #[doc(hidden)]
    pub traffic_routing_config: ::std::option::Option<crate::types::TrafficRoutingConfig>,
}
impl DeploymentConfigInfo {
    /// <p>The deployment configuration ID.</p>
    pub fn deployment_config_id(&self) -> ::std::option::Option<&str> {
        self.deployment_config_id.as_deref()
    }
    /// <p>The deployment configuration name.</p>
    pub fn deployment_config_name(&self) -> ::std::option::Option<&str> {
        self.deployment_config_name.as_deref()
    }
    /// <p>Information about the number or percentage of minimum healthy instance.</p>
    pub fn minimum_healthy_hosts(
        &self,
    ) -> ::std::option::Option<&crate::types::MinimumHealthyHosts> {
        self.minimum_healthy_hosts.as_ref()
    }
    /// <p>The time at which the deployment configuration was created.</p>
    pub fn create_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.create_time.as_ref()
    }
    /// <p>The destination platform type for the deployment (<code>Lambda</code>, <code>Server</code>, or <code>ECS</code>).</p>
    pub fn compute_platform(&self) -> ::std::option::Option<&crate::types::ComputePlatform> {
        self.compute_platform.as_ref()
    }
    /// <p>The configuration that specifies how the deployment traffic is routed. Used for deployments with a Lambda or Amazon ECS compute platform only.</p>
    pub fn traffic_routing_config(
        &self,
    ) -> ::std::option::Option<&crate::types::TrafficRoutingConfig> {
        self.traffic_routing_config.as_ref()
    }
}
impl DeploymentConfigInfo {
    /// Creates a new builder-style object to manufacture [`DeploymentConfigInfo`](crate::types::DeploymentConfigInfo).
    pub fn builder() -> crate::types::builders::DeploymentConfigInfoBuilder {
        crate::types::builders::DeploymentConfigInfoBuilder::default()
    }
}

/// A builder for [`DeploymentConfigInfo`](crate::types::DeploymentConfigInfo).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeploymentConfigInfoBuilder {
    pub(crate) deployment_config_id: ::std::option::Option<::std::string::String>,
    pub(crate) deployment_config_name: ::std::option::Option<::std::string::String>,
    pub(crate) minimum_healthy_hosts: ::std::option::Option<crate::types::MinimumHealthyHosts>,
    pub(crate) create_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) compute_platform: ::std::option::Option<crate::types::ComputePlatform>,
    pub(crate) traffic_routing_config: ::std::option::Option<crate::types::TrafficRoutingConfig>,
}
impl DeploymentConfigInfoBuilder {
    /// <p>The deployment configuration ID.</p>
    pub fn deployment_config_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.deployment_config_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The deployment configuration ID.</p>
    pub fn set_deployment_config_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.deployment_config_id = input;
        self
    }
    /// <p>The deployment configuration name.</p>
    pub fn deployment_config_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.deployment_config_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The deployment configuration name.</p>
    pub fn set_deployment_config_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.deployment_config_name = input;
        self
    }
    /// <p>Information about the number or percentage of minimum healthy instance.</p>
    pub fn minimum_healthy_hosts(mut self, input: crate::types::MinimumHealthyHosts) -> Self {
        self.minimum_healthy_hosts = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the number or percentage of minimum healthy instance.</p>
    pub fn set_minimum_healthy_hosts(
        mut self,
        input: ::std::option::Option<crate::types::MinimumHealthyHosts>,
    ) -> Self {
        self.minimum_healthy_hosts = input;
        self
    }
    /// <p>The time at which the deployment configuration was created.</p>
    pub fn create_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.create_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which the deployment configuration was created.</p>
    pub fn set_create_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.create_time = input;
        self
    }
    /// <p>The destination platform type for the deployment (<code>Lambda</code>, <code>Server</code>, or <code>ECS</code>).</p>
    pub fn compute_platform(mut self, input: crate::types::ComputePlatform) -> Self {
        self.compute_platform = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination platform type for the deployment (<code>Lambda</code>, <code>Server</code>, or <code>ECS</code>).</p>
    pub fn set_compute_platform(
        mut self,
        input: ::std::option::Option<crate::types::ComputePlatform>,
    ) -> Self {
        self.compute_platform = input;
        self
    }
    /// <p>The configuration that specifies how the deployment traffic is routed. Used for deployments with a Lambda or Amazon ECS compute platform only.</p>
    pub fn traffic_routing_config(mut self, input: crate::types::TrafficRoutingConfig) -> Self {
        self.traffic_routing_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration that specifies how the deployment traffic is routed. Used for deployments with a Lambda or Amazon ECS compute platform only.</p>
    pub fn set_traffic_routing_config(
        mut self,
        input: ::std::option::Option<crate::types::TrafficRoutingConfig>,
    ) -> Self {
        self.traffic_routing_config = input;
        self
    }
    /// Consumes the builder and constructs a [`DeploymentConfigInfo`](crate::types::DeploymentConfigInfo).
    pub fn build(self) -> crate::types::DeploymentConfigInfo {
        crate::types::DeploymentConfigInfo {
            deployment_config_id: self.deployment_config_id,
            deployment_config_name: self.deployment_config_name,
            minimum_healthy_hosts: self.minimum_healthy_hosts,
            create_time: self.create_time,
            compute_platform: self.compute_platform,
            traffic_routing_config: self.traffic_routing_config,
        }
    }
}
