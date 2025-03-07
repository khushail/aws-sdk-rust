// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ContinueDeployment`](crate::operation::continue_deployment::builders::ContinueDeploymentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`deployment_id(impl ::std::convert::Into<String>)`](crate::operation::continue_deployment::builders::ContinueDeploymentFluentBuilder::deployment_id) / [`set_deployment_id(Option<String>)`](crate::operation::continue_deployment::builders::ContinueDeploymentFluentBuilder::set_deployment_id): <p> The unique ID of a blue/green deployment for which you want to start rerouting traffic to the replacement environment. </p>
    ///   - [`deployment_wait_type(DeploymentWaitType)`](crate::operation::continue_deployment::builders::ContinueDeploymentFluentBuilder::deployment_wait_type) / [`set_deployment_wait_type(Option<DeploymentWaitType>)`](crate::operation::continue_deployment::builders::ContinueDeploymentFluentBuilder::set_deployment_wait_type): <p> The status of the deployment's waiting period. <code>READY_WAIT</code> indicates that the deployment is ready to start shifting traffic. <code>TERMINATION_WAIT</code> indicates that the traffic is shifted, but the original target is not terminated. </p>
    /// - On success, responds with [`ContinueDeploymentOutput`](crate::operation::continue_deployment::ContinueDeploymentOutput)
    /// - On failure, responds with [`SdkError<ContinueDeploymentError>`](crate::operation::continue_deployment::ContinueDeploymentError)
    pub fn continue_deployment(
        &self,
    ) -> crate::operation::continue_deployment::builders::ContinueDeploymentFluentBuilder {
        crate::operation::continue_deployment::builders::ContinueDeploymentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
