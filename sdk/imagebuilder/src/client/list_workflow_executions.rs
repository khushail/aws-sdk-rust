// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListWorkflowExecutions`](crate::operation::list_workflow_executions::builders::ListWorkflowExecutionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_workflow_executions::builders::ListWorkflowExecutionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_workflow_executions::builders::ListWorkflowExecutionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_workflow_executions::builders::ListWorkflowExecutionsFluentBuilder::set_max_results): <p>The maximum items to return in a request.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_workflow_executions::builders::ListWorkflowExecutionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_workflow_executions::builders::ListWorkflowExecutionsFluentBuilder::set_next_token): <p>A token to specify where to start paginating. This is the NextToken from a previously truncated response.</p>
    ///   - [`image_build_version_arn(impl ::std::convert::Into<String>)`](crate::operation::list_workflow_executions::builders::ListWorkflowExecutionsFluentBuilder::image_build_version_arn) / [`set_image_build_version_arn(Option<String>)`](crate::operation::list_workflow_executions::builders::ListWorkflowExecutionsFluentBuilder::set_image_build_version_arn): <p>List all workflow runtime instances for the specified image build version resource ARN.</p>
    /// - On success, responds with [`ListWorkflowExecutionsOutput`](crate::operation::list_workflow_executions::ListWorkflowExecutionsOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::list_workflow_executions::ListWorkflowExecutionsOutput::request_id): <p>The request ID that uniquely identifies this request.</p>
    ///   - [`workflow_executions(Option<Vec<WorkflowExecutionMetadata>>)`](crate::operation::list_workflow_executions::ListWorkflowExecutionsOutput::workflow_executions): <p>Contains an array of runtime details that represents each time a workflow ran for the requested image build version.</p>
    ///   - [`image_build_version_arn(Option<String>)`](crate::operation::list_workflow_executions::ListWorkflowExecutionsOutput::image_build_version_arn): <p>The resource ARN of the image build version for which you requested a list of workflow runtime details.</p>
    ///   - [`message(Option<String>)`](crate::operation::list_workflow_executions::ListWorkflowExecutionsOutput::message): <p>The output message from the list action, if applicable.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_workflow_executions::ListWorkflowExecutionsOutput::next_token): <p>The next token used for paginated responses. When this field isn't empty, there are additional elements that the service has'ot included in this request. Use this token with the next request to retrieve additional objects.</p>
    /// - On failure, responds with [`SdkError<ListWorkflowExecutionsError>`](crate::operation::list_workflow_executions::ListWorkflowExecutionsError)
    pub fn list_workflow_executions(
        &self,
    ) -> crate::operation::list_workflow_executions::builders::ListWorkflowExecutionsFluentBuilder
    {
        crate::operation::list_workflow_executions::builders::ListWorkflowExecutionsFluentBuilder::new(self.handle.clone())
    }
}
