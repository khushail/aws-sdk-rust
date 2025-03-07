// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetDeploymentConfig`](crate::operation::get_deployment_config::builders::GetDeploymentConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`deployment_config_name(impl ::std::convert::Into<String>)`](crate::operation::get_deployment_config::builders::GetDeploymentConfigFluentBuilder::deployment_config_name) / [`set_deployment_config_name(Option<String>)`](crate::operation::get_deployment_config::builders::GetDeploymentConfigFluentBuilder::set_deployment_config_name): <p>The name of a deployment configuration associated with the IAM user or Amazon Web Services account.</p>
    /// - On success, responds with [`GetDeploymentConfigOutput`](crate::operation::get_deployment_config::GetDeploymentConfigOutput) with field(s):
    ///   - [`deployment_config_info(Option<DeploymentConfigInfo>)`](crate::operation::get_deployment_config::GetDeploymentConfigOutput::deployment_config_info): <p>Information about the deployment configuration.</p>
    /// - On failure, responds with [`SdkError<GetDeploymentConfigError>`](crate::operation::get_deployment_config::GetDeploymentConfigError)
    pub fn get_deployment_config(
        &self,
    ) -> crate::operation::get_deployment_config::builders::GetDeploymentConfigFluentBuilder {
        crate::operation::get_deployment_config::builders::GetDeploymentConfigFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
