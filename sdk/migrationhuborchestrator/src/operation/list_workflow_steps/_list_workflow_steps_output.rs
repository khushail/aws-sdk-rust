// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListWorkflowStepsOutput {
    /// <p>The pagination token.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The summary of steps in a migration workflow.</p>
    #[doc(hidden)]
    pub workflow_steps_summary:
        ::std::option::Option<::std::vec::Vec<crate::types::WorkflowStepSummary>>,
    _request_id: Option<String>,
}
impl ListWorkflowStepsOutput {
    /// <p>The pagination token.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The summary of steps in a migration workflow.</p>
    pub fn workflow_steps_summary(
        &self,
    ) -> ::std::option::Option<&[crate::types::WorkflowStepSummary]> {
        self.workflow_steps_summary.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListWorkflowStepsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListWorkflowStepsOutput {
    /// Creates a new builder-style object to manufacture [`ListWorkflowStepsOutput`](crate::operation::list_workflow_steps::ListWorkflowStepsOutput).
    pub fn builder(
    ) -> crate::operation::list_workflow_steps::builders::ListWorkflowStepsOutputBuilder {
        crate::operation::list_workflow_steps::builders::ListWorkflowStepsOutputBuilder::default()
    }
}

/// A builder for [`ListWorkflowStepsOutput`](crate::operation::list_workflow_steps::ListWorkflowStepsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListWorkflowStepsOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) workflow_steps_summary:
        ::std::option::Option<::std::vec::Vec<crate::types::WorkflowStepSummary>>,
    _request_id: Option<String>,
}
impl ListWorkflowStepsOutputBuilder {
    /// <p>The pagination token.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pagination token.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `workflow_steps_summary`.
    ///
    /// To override the contents of this collection use [`set_workflow_steps_summary`](Self::set_workflow_steps_summary).
    ///
    /// <p>The summary of steps in a migration workflow.</p>
    pub fn workflow_steps_summary(mut self, input: crate::types::WorkflowStepSummary) -> Self {
        let mut v = self.workflow_steps_summary.unwrap_or_default();
        v.push(input);
        self.workflow_steps_summary = ::std::option::Option::Some(v);
        self
    }
    /// <p>The summary of steps in a migration workflow.</p>
    pub fn set_workflow_steps_summary(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::WorkflowStepSummary>>,
    ) -> Self {
        self.workflow_steps_summary = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListWorkflowStepsOutput`](crate::operation::list_workflow_steps::ListWorkflowStepsOutput).
    pub fn build(self) -> crate::operation::list_workflow_steps::ListWorkflowStepsOutput {
        crate::operation::list_workflow_steps::ListWorkflowStepsOutput {
            next_token: self.next_token,
            workflow_steps_summary: self.workflow_steps_summary,
            _request_id: self._request_id,
        }
    }
}
