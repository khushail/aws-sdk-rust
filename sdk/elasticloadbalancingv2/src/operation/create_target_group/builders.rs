// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_target_group::_create_target_group_output::CreateTargetGroupOutputBuilder;

pub use crate::operation::create_target_group::_create_target_group_input::CreateTargetGroupInputBuilder;

/// Fluent builder constructing a request to `CreateTargetGroup`.
///
/// <p>Creates a target group.</p>
/// <p>For more information, see the following:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-target-groups.html">Target groups for your Application Load Balancers</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-target-groups.html">Target groups for your Network Load Balancers</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/gateway/target-groups.html">Target groups for your Gateway Load Balancers</a> </p> </li>
/// </ul>
/// <p>This operation is idempotent, which means that it completes at most one time. If you attempt to create multiple target groups with the same settings, each call succeeds.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateTargetGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_target_group::builders::CreateTargetGroupInputBuilder,
}
impl CreateTargetGroupFluentBuilder {
    /// Creates a new `CreateTargetGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_target_group::CreateTargetGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_target_group::CreateTargetGroupError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_target_group::CreateTargetGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_target_group::CreateTargetGroupError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_target_group::CreateTargetGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_target_group::CreateTargetGroupError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_target_group::CreateTargetGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_target_group::CreateTargetGroupError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the target group.</p>
    /// <p>This name must be unique per region per account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the target group.</p>
    /// <p>This name must be unique per region per account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The protocol to use for routing traffic to the targets. For Application Load Balancers, the supported protocols are HTTP and HTTPS. For Network Load Balancers, the supported protocols are TCP, TLS, UDP, or TCP_UDP. For Gateway Load Balancers, the supported protocol is GENEVE. A TCP_UDP listener must be associated with a TCP_UDP target group. If the target is a Lambda function, this parameter does not apply.</p>
    pub fn protocol(mut self, input: crate::types::ProtocolEnum) -> Self {
        self.inner = self.inner.protocol(input);
        self
    }
    /// <p>The protocol to use for routing traffic to the targets. For Application Load Balancers, the supported protocols are HTTP and HTTPS. For Network Load Balancers, the supported protocols are TCP, TLS, UDP, or TCP_UDP. For Gateway Load Balancers, the supported protocol is GENEVE. A TCP_UDP listener must be associated with a TCP_UDP target group. If the target is a Lambda function, this parameter does not apply.</p>
    pub fn set_protocol(
        mut self,
        input: ::std::option::Option<crate::types::ProtocolEnum>,
    ) -> Self {
        self.inner = self.inner.set_protocol(input);
        self
    }
    /// <p>[HTTP/HTTPS protocol] The protocol version. Specify <code>GRPC</code> to send requests to targets using gRPC. Specify <code>HTTP2</code> to send requests to targets using HTTP/2. The default is <code>HTTP1</code>, which sends requests to targets using HTTP/1.1.</p>
    pub fn protocol_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.protocol_version(input.into());
        self
    }
    /// <p>[HTTP/HTTPS protocol] The protocol version. Specify <code>GRPC</code> to send requests to targets using gRPC. Specify <code>HTTP2</code> to send requests to targets using HTTP/2. The default is <code>HTTP1</code>, which sends requests to targets using HTTP/1.1.</p>
    pub fn set_protocol_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_protocol_version(input);
        self
    }
    /// <p>The port on which the targets receive traffic. This port is used unless you specify a port override when registering the target. If the target is a Lambda function, this parameter does not apply. If the protocol is GENEVE, the supported port is 6081.</p>
    pub fn port(mut self, input: i32) -> Self {
        self.inner = self.inner.port(input);
        self
    }
    /// <p>The port on which the targets receive traffic. This port is used unless you specify a port override when registering the target. If the target is a Lambda function, this parameter does not apply. If the protocol is GENEVE, the supported port is 6081.</p>
    pub fn set_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_port(input);
        self
    }
    /// <p>The identifier of the virtual private cloud (VPC). If the target is a Lambda function, this parameter does not apply. Otherwise, this parameter is required.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpc_id(input.into());
        self
    }
    /// <p>The identifier of the virtual private cloud (VPC). If the target is a Lambda function, this parameter does not apply. Otherwise, this parameter is required.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_id(input);
        self
    }
    /// <p>The protocol the load balancer uses when performing health checks on targets. For Application Load Balancers, the default is HTTP. For Network Load Balancers and Gateway Load Balancers, the default is TCP. The TCP protocol is not supported for health checks if the protocol of the target group is HTTP or HTTPS. The GENEVE, TLS, UDP, and TCP_UDP protocols are not supported for health checks.</p>
    pub fn health_check_protocol(mut self, input: crate::types::ProtocolEnum) -> Self {
        self.inner = self.inner.health_check_protocol(input);
        self
    }
    /// <p>The protocol the load balancer uses when performing health checks on targets. For Application Load Balancers, the default is HTTP. For Network Load Balancers and Gateway Load Balancers, the default is TCP. The TCP protocol is not supported for health checks if the protocol of the target group is HTTP or HTTPS. The GENEVE, TLS, UDP, and TCP_UDP protocols are not supported for health checks.</p>
    pub fn set_health_check_protocol(
        mut self,
        input: ::std::option::Option<crate::types::ProtocolEnum>,
    ) -> Self {
        self.inner = self.inner.set_health_check_protocol(input);
        self
    }
    /// <p>The port the load balancer uses when performing health checks on targets. If the protocol is HTTP, HTTPS, TCP, TLS, UDP, or TCP_UDP, the default is <code>traffic-port</code>, which is the port on which each target receives traffic from the load balancer. If the protocol is GENEVE, the default is port 80.</p>
    pub fn health_check_port(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.health_check_port(input.into());
        self
    }
    /// <p>The port the load balancer uses when performing health checks on targets. If the protocol is HTTP, HTTPS, TCP, TLS, UDP, or TCP_UDP, the default is <code>traffic-port</code>, which is the port on which each target receives traffic from the load balancer. If the protocol is GENEVE, the default is port 80.</p>
    pub fn set_health_check_port(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_health_check_port(input);
        self
    }
    /// <p>Indicates whether health checks are enabled. If the target type is <code>lambda</code>, health checks are disabled by default but can be enabled. If the target type is <code>instance</code>, <code>ip</code>, or <code>alb</code>, health checks are always enabled and cannot be disabled.</p>
    pub fn health_check_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.health_check_enabled(input);
        self
    }
    /// <p>Indicates whether health checks are enabled. If the target type is <code>lambda</code>, health checks are disabled by default but can be enabled. If the target type is <code>instance</code>, <code>ip</code>, or <code>alb</code>, health checks are always enabled and cannot be disabled.</p>
    pub fn set_health_check_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_health_check_enabled(input);
        self
    }
    /// <p>[HTTP/HTTPS health checks] The destination for health checks on the targets.</p>
    /// <p>[HTTP1 or HTTP2 protocol version] The ping path. The default is /.</p>
    /// <p>[GRPC protocol version] The path of a custom health check method with the format /package.service/method. The default is /Amazon Web Services.ALB/healthcheck.</p>
    pub fn health_check_path(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.health_check_path(input.into());
        self
    }
    /// <p>[HTTP/HTTPS health checks] The destination for health checks on the targets.</p>
    /// <p>[HTTP1 or HTTP2 protocol version] The ping path. The default is /.</p>
    /// <p>[GRPC protocol version] The path of a custom health check method with the format /package.service/method. The default is /Amazon Web Services.ALB/healthcheck.</p>
    pub fn set_health_check_path(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_health_check_path(input);
        self
    }
    /// <p>The approximate amount of time, in seconds, between health checks of an individual target. The range is 5-300. If the target group protocol is TCP, TLS, UDP, TCP_UDP, HTTP or HTTPS, the default is 30 seconds. If the target group protocol is GENEVE, the default is 10 seconds. If the target type is <code>lambda</code>, the default is 35 seconds.</p>
    pub fn health_check_interval_seconds(mut self, input: i32) -> Self {
        self.inner = self.inner.health_check_interval_seconds(input);
        self
    }
    /// <p>The approximate amount of time, in seconds, between health checks of an individual target. The range is 5-300. If the target group protocol is TCP, TLS, UDP, TCP_UDP, HTTP or HTTPS, the default is 30 seconds. If the target group protocol is GENEVE, the default is 10 seconds. If the target type is <code>lambda</code>, the default is 35 seconds.</p>
    pub fn set_health_check_interval_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_health_check_interval_seconds(input);
        self
    }
    /// <p>The amount of time, in seconds, during which no response from a target means a failed health check. The range is 2–120 seconds. For target groups with a protocol of HTTP, the default is 6 seconds. For target groups with a protocol of TCP, TLS or HTTPS, the default is 10 seconds. For target groups with a protocol of GENEVE, the default is 5 seconds. If the target type is <code>lambda</code>, the default is 30 seconds.</p>
    pub fn health_check_timeout_seconds(mut self, input: i32) -> Self {
        self.inner = self.inner.health_check_timeout_seconds(input);
        self
    }
    /// <p>The amount of time, in seconds, during which no response from a target means a failed health check. The range is 2–120 seconds. For target groups with a protocol of HTTP, the default is 6 seconds. For target groups with a protocol of TCP, TLS or HTTPS, the default is 10 seconds. For target groups with a protocol of GENEVE, the default is 5 seconds. If the target type is <code>lambda</code>, the default is 30 seconds.</p>
    pub fn set_health_check_timeout_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_health_check_timeout_seconds(input);
        self
    }
    /// <p>The number of consecutive health check successes required before considering a target healthy. The range is 2-10. If the target group protocol is TCP, TCP_UDP, UDP, TLS, HTTP or HTTPS, the default is 5. For target groups with a protocol of GENEVE, the default is 5. If the target type is <code>lambda</code>, the default is 5.</p>
    pub fn healthy_threshold_count(mut self, input: i32) -> Self {
        self.inner = self.inner.healthy_threshold_count(input);
        self
    }
    /// <p>The number of consecutive health check successes required before considering a target healthy. The range is 2-10. If the target group protocol is TCP, TCP_UDP, UDP, TLS, HTTP or HTTPS, the default is 5. For target groups with a protocol of GENEVE, the default is 5. If the target type is <code>lambda</code>, the default is 5.</p>
    pub fn set_healthy_threshold_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_healthy_threshold_count(input);
        self
    }
    /// <p>The number of consecutive health check failures required before considering a target unhealthy. The range is 2-10. If the target group protocol is TCP, TCP_UDP, UDP, TLS, HTTP or HTTPS, the default is 2. For target groups with a protocol of GENEVE, the default is 2. If the target type is <code>lambda</code>, the default is 5.</p>
    pub fn unhealthy_threshold_count(mut self, input: i32) -> Self {
        self.inner = self.inner.unhealthy_threshold_count(input);
        self
    }
    /// <p>The number of consecutive health check failures required before considering a target unhealthy. The range is 2-10. If the target group protocol is TCP, TCP_UDP, UDP, TLS, HTTP or HTTPS, the default is 2. For target groups with a protocol of GENEVE, the default is 2. If the target type is <code>lambda</code>, the default is 5.</p>
    pub fn set_unhealthy_threshold_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_unhealthy_threshold_count(input);
        self
    }
    /// <p>[HTTP/HTTPS health checks] The HTTP or gRPC codes to use when checking for a successful response from a target. For target groups with a protocol of TCP, TCP_UDP, UDP or TLS the range is 200-599. For target groups with a protocol of HTTP or HTTPS, the range is 200-499. For target groups with a protocol of GENEVE, the range is 200-399.</p>
    pub fn matcher(mut self, input: crate::types::Matcher) -> Self {
        self.inner = self.inner.matcher(input);
        self
    }
    /// <p>[HTTP/HTTPS health checks] The HTTP or gRPC codes to use when checking for a successful response from a target. For target groups with a protocol of TCP, TCP_UDP, UDP or TLS the range is 200-599. For target groups with a protocol of HTTP or HTTPS, the range is 200-499. For target groups with a protocol of GENEVE, the range is 200-399.</p>
    pub fn set_matcher(mut self, input: ::std::option::Option<crate::types::Matcher>) -> Self {
        self.inner = self.inner.set_matcher(input);
        self
    }
    /// <p>The type of target that you must specify when registering targets with this target group. You can't specify targets for a target group using more than one target type.</p>
    /// <ul>
    /// <li> <p> <code>instance</code> - Register targets by instance ID. This is the default value.</p> </li>
    /// <li> <p> <code>ip</code> - Register targets by IP address. You can specify IP addresses from the subnets of the virtual private cloud (VPC) for the target group, the RFC 1918 range (10.0.0.0/8, 172.16.0.0/12, and 192.168.0.0/16), and the RFC 6598 range (100.64.0.0/10). You can't specify publicly routable IP addresses.</p> </li>
    /// <li> <p> <code>lambda</code> - Register a single Lambda function as a target.</p> </li>
    /// <li> <p> <code>alb</code> - Register a single Application Load Balancer as a target.</p> </li>
    /// </ul>
    pub fn target_type(mut self, input: crate::types::TargetTypeEnum) -> Self {
        self.inner = self.inner.target_type(input);
        self
    }
    /// <p>The type of target that you must specify when registering targets with this target group. You can't specify targets for a target group using more than one target type.</p>
    /// <ul>
    /// <li> <p> <code>instance</code> - Register targets by instance ID. This is the default value.</p> </li>
    /// <li> <p> <code>ip</code> - Register targets by IP address. You can specify IP addresses from the subnets of the virtual private cloud (VPC) for the target group, the RFC 1918 range (10.0.0.0/8, 172.16.0.0/12, and 192.168.0.0/16), and the RFC 6598 range (100.64.0.0/10). You can't specify publicly routable IP addresses.</p> </li>
    /// <li> <p> <code>lambda</code> - Register a single Lambda function as a target.</p> </li>
    /// <li> <p> <code>alb</code> - Register a single Application Load Balancer as a target.</p> </li>
    /// </ul>
    pub fn set_target_type(
        mut self,
        input: ::std::option::Option<crate::types::TargetTypeEnum>,
    ) -> Self {
        self.inner = self.inner.set_target_type(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to assign to the target group.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags to assign to the target group.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The type of IP address used for this target group. The possible values are <code>ipv4</code> and <code>ipv6</code>. This is an optional parameter. If not specified, the IP address type defaults to <code>ipv4</code>.</p>
    pub fn ip_address_type(mut self, input: crate::types::TargetGroupIpAddressTypeEnum) -> Self {
        self.inner = self.inner.ip_address_type(input);
        self
    }
    /// <p>The type of IP address used for this target group. The possible values are <code>ipv4</code> and <code>ipv6</code>. This is an optional parameter. If not specified, the IP address type defaults to <code>ipv4</code>.</p>
    pub fn set_ip_address_type(
        mut self,
        input: ::std::option::Option<crate::types::TargetGroupIpAddressTypeEnum>,
    ) -> Self {
        self.inner = self.inner.set_ip_address_type(input);
        self
    }
}
