// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListWorkflows`](crate::operation::list_workflows::builders::ListWorkflowsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_workflows::builders::ListWorkflowsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`r#type(WorkflowType)`](crate::operation::list_workflows::builders::ListWorkflowsFluentBuilder::type) / [`set_type(Option<WorkflowType>)`](crate::operation::list_workflows::builders::ListWorkflowsFluentBuilder::set_type): <p>The workflows' type.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::list_workflows::builders::ListWorkflowsFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::list_workflows::builders::ListWorkflowsFluentBuilder::set_name): <p>The workflows' name.</p>
    ///   - [`starting_token(impl ::std::convert::Into<String>)`](crate::operation::list_workflows::builders::ListWorkflowsFluentBuilder::starting_token) / [`set_starting_token(Option<String>)`](crate::operation::list_workflows::builders::ListWorkflowsFluentBuilder::set_starting_token): <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_workflows::builders::ListWorkflowsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_workflows::builders::ListWorkflowsFluentBuilder::set_max_results): <p>The maximum number of workflows to return in one page of results.</p>
    /// - On success, responds with [`ListWorkflowsOutput`](crate::operation::list_workflows::ListWorkflowsOutput) with field(s):
    ///   - [`items(Option<Vec<WorkflowListItem>>)`](crate::operation::list_workflows::ListWorkflowsOutput::items): <p>The workflows' items.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_workflows::ListWorkflowsOutput::next_token): <p>A pagination token that's included if more results are available.</p>
    /// - On failure, responds with [`SdkError<ListWorkflowsError>`](crate::operation::list_workflows::ListWorkflowsError)
    pub fn list_workflows(
        &self,
    ) -> crate::operation::list_workflows::builders::ListWorkflowsFluentBuilder {
        crate::operation::list_workflows::builders::ListWorkflowsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
