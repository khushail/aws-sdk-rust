// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetDeployment`](crate::operation::get_deployment::builders::GetDeploymentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`deployment_id(impl ::std::convert::Into<String>)`](crate::operation::get_deployment::builders::GetDeploymentFluentBuilder::deployment_id) / [`set_deployment_id(Option<String>)`](crate::operation::get_deployment::builders::GetDeploymentFluentBuilder::set_deployment_id): <p>The ID of the deployment.</p>
    /// - On success, responds with [`GetDeploymentOutput`](crate::operation::get_deployment::GetDeploymentOutput) with field(s):
    ///   - [`target_arn(Option<String>)`](crate::operation::get_deployment::GetDeploymentOutput::target_arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the target IoT thing or thing group.</p>
    ///   - [`revision_id(Option<String>)`](crate::operation::get_deployment::GetDeploymentOutput::revision_id): <p>The revision number of the deployment.</p>
    ///   - [`deployment_id(Option<String>)`](crate::operation::get_deployment::GetDeploymentOutput::deployment_id): <p>The ID of the deployment.</p>
    ///   - [`deployment_name(Option<String>)`](crate::operation::get_deployment::GetDeploymentOutput::deployment_name): <p>The name of the deployment.</p>
    ///   - [`deployment_status(Option<DeploymentStatus>)`](crate::operation::get_deployment::GetDeploymentOutput::deployment_status): <p>The status of the deployment.</p>
    ///   - [`iot_job_id(Option<String>)`](crate::operation::get_deployment::GetDeploymentOutput::iot_job_id): <p>The ID of the IoT job that applies the deployment to target devices.</p>
    ///   - [`iot_job_arn(Option<String>)`](crate::operation::get_deployment::GetDeploymentOutput::iot_job_arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the IoT job that applies the deployment to target devices.</p>
    ///   - [`components(Option<HashMap<String, ComponentDeploymentSpecification>>)`](crate::operation::get_deployment::GetDeploymentOutput::components): <p>The components to deploy. This is a dictionary, where each key is the name of a component, and each key's value is the version and configuration to deploy for that component.</p>
    ///   - [`deployment_policies(Option<DeploymentPolicies>)`](crate::operation::get_deployment::GetDeploymentOutput::deployment_policies): <p>The deployment policies for the deployment. These policies define how the deployment updates components and handles failure.</p>
    ///   - [`iot_job_configuration(Option<DeploymentIoTJobConfiguration>)`](crate::operation::get_deployment::GetDeploymentOutput::iot_job_configuration): <p>The job configuration for the deployment configuration. The job configuration specifies the rollout, timeout, and stop configurations for the deployment configuration.</p>
    ///   - [`creation_timestamp(Option<DateTime>)`](crate::operation::get_deployment::GetDeploymentOutput::creation_timestamp): <p>The time at which the deployment was created, expressed in ISO 8601 format.</p>
    ///   - [`is_latest_for_target(bool)`](crate::operation::get_deployment::GetDeploymentOutput::is_latest_for_target): <p>Whether or not the deployment is the latest revision for its target.</p>
    ///   - [`parent_target_arn(Option<String>)`](crate::operation::get_deployment::GetDeploymentOutput::parent_target_arn): <p>The parent deployment's target <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> within a subdeployment.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::get_deployment::GetDeploymentOutput::tags): <p>A list of key-value pairs that contain metadata for the resource. For more information, see <a href="https://docs.aws.amazon.com/greengrass/v2/developerguide/tag-resources.html">Tag your resources</a> in the <i>IoT Greengrass V2 Developer Guide</i>.</p>
    /// - On failure, responds with [`SdkError<GetDeploymentError>`](crate::operation::get_deployment::GetDeploymentError)
    pub fn get_deployment(
        &self,
    ) -> crate::operation::get_deployment::builders::GetDeploymentFluentBuilder {
        crate::operation::get_deployment::builders::GetDeploymentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
