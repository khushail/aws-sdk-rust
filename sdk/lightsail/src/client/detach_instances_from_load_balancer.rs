// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DetachInstancesFromLoadBalancer`](crate::operation::detach_instances_from_load_balancer::builders::DetachInstancesFromLoadBalancerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`load_balancer_name(impl ::std::convert::Into<String>)`](crate::operation::detach_instances_from_load_balancer::builders::DetachInstancesFromLoadBalancerFluentBuilder::load_balancer_name) / [`set_load_balancer_name(Option<String>)`](crate::operation::detach_instances_from_load_balancer::builders::DetachInstancesFromLoadBalancerFluentBuilder::set_load_balancer_name): <p>The name of the Lightsail load balancer.</p>
    ///   - [`instance_names(Vec<String>)`](crate::operation::detach_instances_from_load_balancer::builders::DetachInstancesFromLoadBalancerFluentBuilder::instance_names) / [`set_instance_names(Option<Vec<String>>)`](crate::operation::detach_instances_from_load_balancer::builders::DetachInstancesFromLoadBalancerFluentBuilder::set_instance_names): <p>An array of strings containing the names of the instances you want to detach from the load balancer.</p>
    /// - On success, responds with [`DetachInstancesFromLoadBalancerOutput`](crate::operation::detach_instances_from_load_balancer::DetachInstancesFromLoadBalancerOutput) with field(s):
    ///   - [`operations(Option<Vec<Operation>>)`](crate::operation::detach_instances_from_load_balancer::DetachInstancesFromLoadBalancerOutput::operations): <p>An array of objects that describe the result of the action, such as the status of the request, the timestamp of the request, and the resources affected by the request.</p>
    /// - On failure, responds with [`SdkError<DetachInstancesFromLoadBalancerError>`](crate::operation::detach_instances_from_load_balancer::DetachInstancesFromLoadBalancerError)
    pub fn detach_instances_from_load_balancer(&self) -> crate::operation::detach_instances_from_load_balancer::builders::DetachInstancesFromLoadBalancerFluentBuilder{
        crate::operation::detach_instances_from_load_balancer::builders::DetachInstancesFromLoadBalancerFluentBuilder::new(self.handle.clone())
    }
}
