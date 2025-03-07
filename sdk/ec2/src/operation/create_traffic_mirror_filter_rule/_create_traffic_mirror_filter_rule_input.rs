// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateTrafficMirrorFilterRuleInput {
    /// <p>The ID of the filter that this rule is associated with.</p>
    #[doc(hidden)]
    pub traffic_mirror_filter_id: ::std::option::Option<::std::string::String>,
    /// <p>The type of traffic.</p>
    #[doc(hidden)]
    pub traffic_direction: ::std::option::Option<crate::types::TrafficDirection>,
    /// <p>The number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given direction. The rules are processed in ascending order by rule number.</p>
    #[doc(hidden)]
    pub rule_number: ::std::option::Option<i32>,
    /// <p>The action to take on the filtered traffic.</p>
    #[doc(hidden)]
    pub rule_action: ::std::option::Option<crate::types::TrafficMirrorRuleAction>,
    /// <p>The destination port range.</p>
    #[doc(hidden)]
    pub destination_port_range: ::std::option::Option<crate::types::TrafficMirrorPortRangeRequest>,
    /// <p>The source port range.</p>
    #[doc(hidden)]
    pub source_port_range: ::std::option::Option<crate::types::TrafficMirrorPortRangeRequest>,
    /// <p>The protocol, for example UDP, to assign to the Traffic Mirror rule.</p>
    /// <p>For information about the protocol value, see <a href="https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a> on the Internet Assigned Numbers Authority (IANA) website.</p>
    #[doc(hidden)]
    pub protocol: ::std::option::Option<i32>,
    /// <p>The destination CIDR block to assign to the Traffic Mirror rule.</p>
    #[doc(hidden)]
    pub destination_cidr_block: ::std::option::Option<::std::string::String>,
    /// <p>The source CIDR block to assign to the Traffic Mirror rule.</p>
    #[doc(hidden)]
    pub source_cidr_block: ::std::option::Option<::std::string::String>,
    /// <p>The description of the Traffic Mirror rule.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
}
impl CreateTrafficMirrorFilterRuleInput {
    /// <p>The ID of the filter that this rule is associated with.</p>
    pub fn traffic_mirror_filter_id(&self) -> ::std::option::Option<&str> {
        self.traffic_mirror_filter_id.as_deref()
    }
    /// <p>The type of traffic.</p>
    pub fn traffic_direction(&self) -> ::std::option::Option<&crate::types::TrafficDirection> {
        self.traffic_direction.as_ref()
    }
    /// <p>The number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given direction. The rules are processed in ascending order by rule number.</p>
    pub fn rule_number(&self) -> ::std::option::Option<i32> {
        self.rule_number
    }
    /// <p>The action to take on the filtered traffic.</p>
    pub fn rule_action(&self) -> ::std::option::Option<&crate::types::TrafficMirrorRuleAction> {
        self.rule_action.as_ref()
    }
    /// <p>The destination port range.</p>
    pub fn destination_port_range(
        &self,
    ) -> ::std::option::Option<&crate::types::TrafficMirrorPortRangeRequest> {
        self.destination_port_range.as_ref()
    }
    /// <p>The source port range.</p>
    pub fn source_port_range(
        &self,
    ) -> ::std::option::Option<&crate::types::TrafficMirrorPortRangeRequest> {
        self.source_port_range.as_ref()
    }
    /// <p>The protocol, for example UDP, to assign to the Traffic Mirror rule.</p>
    /// <p>For information about the protocol value, see <a href="https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a> on the Internet Assigned Numbers Authority (IANA) website.</p>
    pub fn protocol(&self) -> ::std::option::Option<i32> {
        self.protocol
    }
    /// <p>The destination CIDR block to assign to the Traffic Mirror rule.</p>
    pub fn destination_cidr_block(&self) -> ::std::option::Option<&str> {
        self.destination_cidr_block.as_deref()
    }
    /// <p>The source CIDR block to assign to the Traffic Mirror rule.</p>
    pub fn source_cidr_block(&self) -> ::std::option::Option<&str> {
        self.source_cidr_block.as_deref()
    }
    /// <p>The description of the Traffic Mirror rule.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
}
impl CreateTrafficMirrorFilterRuleInput {
    /// Creates a new builder-style object to manufacture [`CreateTrafficMirrorFilterRuleInput`](crate::operation::create_traffic_mirror_filter_rule::CreateTrafficMirrorFilterRuleInput).
    pub fn builder() -> crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleInputBuilder{
        crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleInputBuilder::default()
    }
}

/// A builder for [`CreateTrafficMirrorFilterRuleInput`](crate::operation::create_traffic_mirror_filter_rule::CreateTrafficMirrorFilterRuleInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateTrafficMirrorFilterRuleInputBuilder {
    pub(crate) traffic_mirror_filter_id: ::std::option::Option<::std::string::String>,
    pub(crate) traffic_direction: ::std::option::Option<crate::types::TrafficDirection>,
    pub(crate) rule_number: ::std::option::Option<i32>,
    pub(crate) rule_action: ::std::option::Option<crate::types::TrafficMirrorRuleAction>,
    pub(crate) destination_port_range:
        ::std::option::Option<crate::types::TrafficMirrorPortRangeRequest>,
    pub(crate) source_port_range:
        ::std::option::Option<crate::types::TrafficMirrorPortRangeRequest>,
    pub(crate) protocol: ::std::option::Option<i32>,
    pub(crate) destination_cidr_block: ::std::option::Option<::std::string::String>,
    pub(crate) source_cidr_block: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
}
impl CreateTrafficMirrorFilterRuleInputBuilder {
    /// <p>The ID of the filter that this rule is associated with.</p>
    pub fn traffic_mirror_filter_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.traffic_mirror_filter_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the filter that this rule is associated with.</p>
    pub fn set_traffic_mirror_filter_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.traffic_mirror_filter_id = input;
        self
    }
    /// <p>The type of traffic.</p>
    pub fn traffic_direction(mut self, input: crate::types::TrafficDirection) -> Self {
        self.traffic_direction = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of traffic.</p>
    pub fn set_traffic_direction(
        mut self,
        input: ::std::option::Option<crate::types::TrafficDirection>,
    ) -> Self {
        self.traffic_direction = input;
        self
    }
    /// <p>The number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given direction. The rules are processed in ascending order by rule number.</p>
    pub fn rule_number(mut self, input: i32) -> Self {
        self.rule_number = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given direction. The rules are processed in ascending order by rule number.</p>
    pub fn set_rule_number(mut self, input: ::std::option::Option<i32>) -> Self {
        self.rule_number = input;
        self
    }
    /// <p>The action to take on the filtered traffic.</p>
    pub fn rule_action(mut self, input: crate::types::TrafficMirrorRuleAction) -> Self {
        self.rule_action = ::std::option::Option::Some(input);
        self
    }
    /// <p>The action to take on the filtered traffic.</p>
    pub fn set_rule_action(
        mut self,
        input: ::std::option::Option<crate::types::TrafficMirrorRuleAction>,
    ) -> Self {
        self.rule_action = input;
        self
    }
    /// <p>The destination port range.</p>
    pub fn destination_port_range(
        mut self,
        input: crate::types::TrafficMirrorPortRangeRequest,
    ) -> Self {
        self.destination_port_range = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination port range.</p>
    pub fn set_destination_port_range(
        mut self,
        input: ::std::option::Option<crate::types::TrafficMirrorPortRangeRequest>,
    ) -> Self {
        self.destination_port_range = input;
        self
    }
    /// <p>The source port range.</p>
    pub fn source_port_range(mut self, input: crate::types::TrafficMirrorPortRangeRequest) -> Self {
        self.source_port_range = ::std::option::Option::Some(input);
        self
    }
    /// <p>The source port range.</p>
    pub fn set_source_port_range(
        mut self,
        input: ::std::option::Option<crate::types::TrafficMirrorPortRangeRequest>,
    ) -> Self {
        self.source_port_range = input;
        self
    }
    /// <p>The protocol, for example UDP, to assign to the Traffic Mirror rule.</p>
    /// <p>For information about the protocol value, see <a href="https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a> on the Internet Assigned Numbers Authority (IANA) website.</p>
    pub fn protocol(mut self, input: i32) -> Self {
        self.protocol = ::std::option::Option::Some(input);
        self
    }
    /// <p>The protocol, for example UDP, to assign to the Traffic Mirror rule.</p>
    /// <p>For information about the protocol value, see <a href="https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a> on the Internet Assigned Numbers Authority (IANA) website.</p>
    pub fn set_protocol(mut self, input: ::std::option::Option<i32>) -> Self {
        self.protocol = input;
        self
    }
    /// <p>The destination CIDR block to assign to the Traffic Mirror rule.</p>
    pub fn destination_cidr_block(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.destination_cidr_block = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The destination CIDR block to assign to the Traffic Mirror rule.</p>
    pub fn set_destination_cidr_block(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.destination_cidr_block = input;
        self
    }
    /// <p>The source CIDR block to assign to the Traffic Mirror rule.</p>
    pub fn source_cidr_block(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.source_cidr_block = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The source CIDR block to assign to the Traffic Mirror rule.</p>
    pub fn set_source_cidr_block(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.source_cidr_block = input;
        self
    }
    /// <p>The description of the Traffic Mirror rule.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the Traffic Mirror rule.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateTrafficMirrorFilterRuleInput`](crate::operation::create_traffic_mirror_filter_rule::CreateTrafficMirrorFilterRuleInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_traffic_mirror_filter_rule::CreateTrafficMirrorFilterRuleInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_traffic_mirror_filter_rule::CreateTrafficMirrorFilterRuleInput {
                traffic_mirror_filter_id: self.traffic_mirror_filter_id
                ,
                traffic_direction: self.traffic_direction
                ,
                rule_number: self.rule_number
                ,
                rule_action: self.rule_action
                ,
                destination_port_range: self.destination_port_range
                ,
                source_port_range: self.source_port_range
                ,
                protocol: self.protocol
                ,
                destination_cidr_block: self.destination_cidr_block
                ,
                source_cidr_block: self.source_cidr_block
                ,
                description: self.description
                ,
                dry_run: self.dry_run
                ,
                client_token: self.client_token
                ,
            }
        )
    }
}
