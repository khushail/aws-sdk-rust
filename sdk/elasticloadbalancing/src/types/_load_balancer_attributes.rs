// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The attributes for a load balancer.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LoadBalancerAttributes {
    /// <p>If enabled, the load balancer routes the request traffic evenly across all instances regardless of the Availability Zones.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-crosszone-lb.html">Configure Cross-Zone Load Balancing</a> in the <i>Classic Load Balancers Guide</i>.</p>
    #[doc(hidden)]
    pub cross_zone_load_balancing: ::std::option::Option<crate::types::CrossZoneLoadBalancing>,
    /// <p>If enabled, the load balancer captures detailed information of all requests and delivers the information to the Amazon S3 bucket that you specify.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-access-logs.html">Enable Access Logs</a> in the <i>Classic Load Balancers Guide</i>.</p>
    #[doc(hidden)]
    pub access_log: ::std::option::Option<crate::types::AccessLog>,
    /// <p>If enabled, the load balancer allows existing requests to complete before the load balancer shifts traffic away from a deregistered or unhealthy instance.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-conn-drain.html">Configure Connection Draining</a> in the <i>Classic Load Balancers Guide</i>.</p>
    #[doc(hidden)]
    pub connection_draining: ::std::option::Option<crate::types::ConnectionDraining>,
    /// <p>If enabled, the load balancer allows the connections to remain idle (no data is sent over the connection) for the specified duration.</p>
    /// <p>By default, Elastic Load Balancing maintains a 60-second idle connection timeout for both front-end and back-end connections of your load balancer. For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-idle-timeout.html">Configure Idle Connection Timeout</a> in the <i>Classic Load Balancers Guide</i>.</p>
    #[doc(hidden)]
    pub connection_settings: ::std::option::Option<crate::types::ConnectionSettings>,
    /// <p>Any additional attributes.</p>
    #[doc(hidden)]
    pub additional_attributes:
        ::std::option::Option<::std::vec::Vec<crate::types::AdditionalAttribute>>,
}
impl LoadBalancerAttributes {
    /// <p>If enabled, the load balancer routes the request traffic evenly across all instances regardless of the Availability Zones.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-crosszone-lb.html">Configure Cross-Zone Load Balancing</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn cross_zone_load_balancing(
        &self,
    ) -> ::std::option::Option<&crate::types::CrossZoneLoadBalancing> {
        self.cross_zone_load_balancing.as_ref()
    }
    /// <p>If enabled, the load balancer captures detailed information of all requests and delivers the information to the Amazon S3 bucket that you specify.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-access-logs.html">Enable Access Logs</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn access_log(&self) -> ::std::option::Option<&crate::types::AccessLog> {
        self.access_log.as_ref()
    }
    /// <p>If enabled, the load balancer allows existing requests to complete before the load balancer shifts traffic away from a deregistered or unhealthy instance.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-conn-drain.html">Configure Connection Draining</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn connection_draining(&self) -> ::std::option::Option<&crate::types::ConnectionDraining> {
        self.connection_draining.as_ref()
    }
    /// <p>If enabled, the load balancer allows the connections to remain idle (no data is sent over the connection) for the specified duration.</p>
    /// <p>By default, Elastic Load Balancing maintains a 60-second idle connection timeout for both front-end and back-end connections of your load balancer. For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-idle-timeout.html">Configure Idle Connection Timeout</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn connection_settings(&self) -> ::std::option::Option<&crate::types::ConnectionSettings> {
        self.connection_settings.as_ref()
    }
    /// <p>Any additional attributes.</p>
    pub fn additional_attributes(
        &self,
    ) -> ::std::option::Option<&[crate::types::AdditionalAttribute]> {
        self.additional_attributes.as_deref()
    }
}
impl LoadBalancerAttributes {
    /// Creates a new builder-style object to manufacture [`LoadBalancerAttributes`](crate::types::LoadBalancerAttributes).
    pub fn builder() -> crate::types::builders::LoadBalancerAttributesBuilder {
        crate::types::builders::LoadBalancerAttributesBuilder::default()
    }
}

/// A builder for [`LoadBalancerAttributes`](crate::types::LoadBalancerAttributes).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LoadBalancerAttributesBuilder {
    pub(crate) cross_zone_load_balancing:
        ::std::option::Option<crate::types::CrossZoneLoadBalancing>,
    pub(crate) access_log: ::std::option::Option<crate::types::AccessLog>,
    pub(crate) connection_draining: ::std::option::Option<crate::types::ConnectionDraining>,
    pub(crate) connection_settings: ::std::option::Option<crate::types::ConnectionSettings>,
    pub(crate) additional_attributes:
        ::std::option::Option<::std::vec::Vec<crate::types::AdditionalAttribute>>,
}
impl LoadBalancerAttributesBuilder {
    /// <p>If enabled, the load balancer routes the request traffic evenly across all instances regardless of the Availability Zones.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-crosszone-lb.html">Configure Cross-Zone Load Balancing</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn cross_zone_load_balancing(
        mut self,
        input: crate::types::CrossZoneLoadBalancing,
    ) -> Self {
        self.cross_zone_load_balancing = ::std::option::Option::Some(input);
        self
    }
    /// <p>If enabled, the load balancer routes the request traffic evenly across all instances regardless of the Availability Zones.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-crosszone-lb.html">Configure Cross-Zone Load Balancing</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn set_cross_zone_load_balancing(
        mut self,
        input: ::std::option::Option<crate::types::CrossZoneLoadBalancing>,
    ) -> Self {
        self.cross_zone_load_balancing = input;
        self
    }
    /// <p>If enabled, the load balancer captures detailed information of all requests and delivers the information to the Amazon S3 bucket that you specify.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-access-logs.html">Enable Access Logs</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn access_log(mut self, input: crate::types::AccessLog) -> Self {
        self.access_log = ::std::option::Option::Some(input);
        self
    }
    /// <p>If enabled, the load balancer captures detailed information of all requests and delivers the information to the Amazon S3 bucket that you specify.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-access-logs.html">Enable Access Logs</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn set_access_log(mut self, input: ::std::option::Option<crate::types::AccessLog>) -> Self {
        self.access_log = input;
        self
    }
    /// <p>If enabled, the load balancer allows existing requests to complete before the load balancer shifts traffic away from a deregistered or unhealthy instance.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-conn-drain.html">Configure Connection Draining</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn connection_draining(mut self, input: crate::types::ConnectionDraining) -> Self {
        self.connection_draining = ::std::option::Option::Some(input);
        self
    }
    /// <p>If enabled, the load balancer allows existing requests to complete before the load balancer shifts traffic away from a deregistered or unhealthy instance.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-conn-drain.html">Configure Connection Draining</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn set_connection_draining(
        mut self,
        input: ::std::option::Option<crate::types::ConnectionDraining>,
    ) -> Self {
        self.connection_draining = input;
        self
    }
    /// <p>If enabled, the load balancer allows the connections to remain idle (no data is sent over the connection) for the specified duration.</p>
    /// <p>By default, Elastic Load Balancing maintains a 60-second idle connection timeout for both front-end and back-end connections of your load balancer. For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-idle-timeout.html">Configure Idle Connection Timeout</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn connection_settings(mut self, input: crate::types::ConnectionSettings) -> Self {
        self.connection_settings = ::std::option::Option::Some(input);
        self
    }
    /// <p>If enabled, the load balancer allows the connections to remain idle (no data is sent over the connection) for the specified duration.</p>
    /// <p>By default, Elastic Load Balancing maintains a 60-second idle connection timeout for both front-end and back-end connections of your load balancer. For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-idle-timeout.html">Configure Idle Connection Timeout</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub fn set_connection_settings(
        mut self,
        input: ::std::option::Option<crate::types::ConnectionSettings>,
    ) -> Self {
        self.connection_settings = input;
        self
    }
    /// Appends an item to `additional_attributes`.
    ///
    /// To override the contents of this collection use [`set_additional_attributes`](Self::set_additional_attributes).
    ///
    /// <p>Any additional attributes.</p>
    pub fn additional_attributes(mut self, input: crate::types::AdditionalAttribute) -> Self {
        let mut v = self.additional_attributes.unwrap_or_default();
        v.push(input);
        self.additional_attributes = ::std::option::Option::Some(v);
        self
    }
    /// <p>Any additional attributes.</p>
    pub fn set_additional_attributes(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AdditionalAttribute>>,
    ) -> Self {
        self.additional_attributes = input;
        self
    }
    /// Consumes the builder and constructs a [`LoadBalancerAttributes`](crate::types::LoadBalancerAttributes).
    pub fn build(self) -> crate::types::LoadBalancerAttributes {
        crate::types::LoadBalancerAttributes {
            cross_zone_load_balancing: self.cross_zone_load_balancing,
            access_log: self.access_log,
            connection_draining: self.connection_draining,
            connection_settings: self.connection_settings,
            additional_attributes: self.additional_attributes,
        }
    }
}
