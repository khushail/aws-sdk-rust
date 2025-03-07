// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeregisterEcsCluster`](crate::operation::deregister_ecs_cluster::builders::DeregisterEcsClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`ecs_cluster_arn(impl ::std::convert::Into<String>)`](crate::operation::deregister_ecs_cluster::builders::DeregisterEcsClusterFluentBuilder::ecs_cluster_arn) / [`set_ecs_cluster_arn(Option<String>)`](crate::operation::deregister_ecs_cluster::builders::DeregisterEcsClusterFluentBuilder::set_ecs_cluster_arn): <p>The cluster's Amazon Resource Number (ARN).</p>
    /// - On success, responds with [`DeregisterEcsClusterOutput`](crate::operation::deregister_ecs_cluster::DeregisterEcsClusterOutput)
    /// - On failure, responds with [`SdkError<DeregisterEcsClusterError>`](crate::operation::deregister_ecs_cluster::DeregisterEcsClusterError)
    pub fn deregister_ecs_cluster(
        &self,
    ) -> crate::operation::deregister_ecs_cluster::builders::DeregisterEcsClusterFluentBuilder {
        crate::operation::deregister_ecs_cluster::builders::DeregisterEcsClusterFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
