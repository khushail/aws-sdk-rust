// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetWorkflow`](crate::operation::get_workflow::builders::GetWorkflowFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::get_workflow::builders::GetWorkflowFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::get_workflow::builders::GetWorkflowFluentBuilder::set_id): <p>The ID of the migration workflow.</p>
    /// - On success, responds with [`GetWorkflowOutput`](crate::operation::get_workflow::GetWorkflowOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::operation::get_workflow::GetWorkflowOutput::id): <p>The ID of the migration workflow.</p>
    ///   - [`arn(Option<String>)`](crate::operation::get_workflow::GetWorkflowOutput::arn): <p>The Amazon Resource Name (ARN) of the migration workflow.</p>
    ///   - [`name(Option<String>)`](crate::operation::get_workflow::GetWorkflowOutput::name): <p>The name of the migration workflow.</p>
    ///   - [`description(Option<String>)`](crate::operation::get_workflow::GetWorkflowOutput::description): <p>The description of the migration workflow.</p>
    ///   - [`template_id(Option<String>)`](crate::operation::get_workflow::GetWorkflowOutput::template_id): <p>The ID of the template.</p>
    ///   - [`ads_application_configuration_id(Option<String>)`](crate::operation::get_workflow::GetWorkflowOutput::ads_application_configuration_id): <p>The configuration ID of the application configured in Application Discovery Service.</p>
    ///   - [`ads_application_name(Option<String>)`](crate::operation::get_workflow::GetWorkflowOutput::ads_application_name): <p>The name of the application configured in Application Discovery Service.</p>
    ///   - [`status(Option<MigrationWorkflowStatusEnum>)`](crate::operation::get_workflow::GetWorkflowOutput::status): <p>The status of the migration workflow.</p>
    ///   - [`status_message(Option<String>)`](crate::operation::get_workflow::GetWorkflowOutput::status_message): <p>The status message of the migration workflow.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::get_workflow::GetWorkflowOutput::creation_time): <p>The time at which the migration workflow was created.</p>
    ///   - [`last_start_time(Option<DateTime>)`](crate::operation::get_workflow::GetWorkflowOutput::last_start_time): <p>The time at which the migration workflow was last started.</p>
    ///   - [`last_stop_time(Option<DateTime>)`](crate::operation::get_workflow::GetWorkflowOutput::last_stop_time): <p>The time at which the migration workflow was last stopped.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::operation::get_workflow::GetWorkflowOutput::last_modified_time): <p>The time at which the migration workflow was last modified.</p>
    ///   - [`end_time(Option<DateTime>)`](crate::operation::get_workflow::GetWorkflowOutput::end_time): <p>The time at which the migration workflow ended.</p>
    ///   - [`tools(Option<Vec<Tool>>)`](crate::operation::get_workflow::GetWorkflowOutput::tools): <p>List of AWS services utilized in a migration workflow.</p>
    ///   - [`total_steps(Option<i32>)`](crate::operation::get_workflow::GetWorkflowOutput::total_steps): <p>The total number of steps in the migration workflow.</p>
    ///   - [`completed_steps(Option<i32>)`](crate::operation::get_workflow::GetWorkflowOutput::completed_steps): <p>Get a list of completed steps in the migration workflow.</p>
    ///   - [`workflow_inputs(Option<HashMap<String, StepInput>>)`](crate::operation::get_workflow::GetWorkflowOutput::workflow_inputs): <p>The inputs required for creating the migration workflow.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::get_workflow::GetWorkflowOutput::tags): <p>The tags added to the migration workflow.</p>
    ///   - [`workflow_bucket(Option<String>)`](crate::operation::get_workflow::GetWorkflowOutput::workflow_bucket): <p>The Amazon S3 bucket where the migration logs are stored.</p>
    /// - On failure, responds with [`SdkError<GetWorkflowError>`](crate::operation::get_workflow::GetWorkflowError)
    pub fn get_workflow(
        &self,
    ) -> crate::operation::get_workflow::builders::GetWorkflowFluentBuilder {
        crate::operation::get_workflow::builders::GetWorkflowFluentBuilder::new(self.handle.clone())
    }
}
