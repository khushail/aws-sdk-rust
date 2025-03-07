// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeRoutingProfile`](crate::operation::describe_routing_profile::builders::DescribeRoutingProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::describe_routing_profile::builders::DescribeRoutingProfileFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::describe_routing_profile::builders::DescribeRoutingProfileFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    ///   - [`routing_profile_id(impl ::std::convert::Into<String>)`](crate::operation::describe_routing_profile::builders::DescribeRoutingProfileFluentBuilder::routing_profile_id) / [`set_routing_profile_id(Option<String>)`](crate::operation::describe_routing_profile::builders::DescribeRoutingProfileFluentBuilder::set_routing_profile_id): <p>The identifier of the routing profile.</p>
    /// - On success, responds with [`DescribeRoutingProfileOutput`](crate::operation::describe_routing_profile::DescribeRoutingProfileOutput) with field(s):
    ///   - [`routing_profile(Option<RoutingProfile>)`](crate::operation::describe_routing_profile::DescribeRoutingProfileOutput::routing_profile): <p>The routing profile.</p>
    /// - On failure, responds with [`SdkError<DescribeRoutingProfileError>`](crate::operation::describe_routing_profile::DescribeRoutingProfileError)
    pub fn describe_routing_profile(
        &self,
    ) -> crate::operation::describe_routing_profile::builders::DescribeRoutingProfileFluentBuilder
    {
        crate::operation::describe_routing_profile::builders::DescribeRoutingProfileFluentBuilder::new(self.handle.clone())
    }
}
