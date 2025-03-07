// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListWorkflowStepGroupsInput {
    /// <p>The pagination token.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results that can be returned.</p>
    #[doc(hidden)]
    pub max_results: i32,
    /// <p>The ID of the migration workflow.</p>
    #[doc(hidden)]
    pub workflow_id: ::std::option::Option<::std::string::String>,
}
impl ListWorkflowStepGroupsInput {
    /// <p>The pagination token.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results that can be returned.</p>
    pub fn max_results(&self) -> i32 {
        self.max_results
    }
    /// <p>The ID of the migration workflow.</p>
    pub fn workflow_id(&self) -> ::std::option::Option<&str> {
        self.workflow_id.as_deref()
    }
}
impl ListWorkflowStepGroupsInput {
    /// Creates a new builder-style object to manufacture [`ListWorkflowStepGroupsInput`](crate::operation::list_workflow_step_groups::ListWorkflowStepGroupsInput).
    pub fn builder(
    ) -> crate::operation::list_workflow_step_groups::builders::ListWorkflowStepGroupsInputBuilder
    {
        crate::operation::list_workflow_step_groups::builders::ListWorkflowStepGroupsInputBuilder::default()
    }
}

/// A builder for [`ListWorkflowStepGroupsInput`](crate::operation::list_workflow_step_groups::ListWorkflowStepGroupsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListWorkflowStepGroupsInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) workflow_id: ::std::option::Option<::std::string::String>,
}
impl ListWorkflowStepGroupsInputBuilder {
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
    /// <p>The maximum number of results that can be returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results that can be returned.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The ID of the migration workflow.</p>
    pub fn workflow_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workflow_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the migration workflow.</p>
    pub fn set_workflow_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workflow_id = input;
        self
    }
    /// Consumes the builder and constructs a [`ListWorkflowStepGroupsInput`](crate::operation::list_workflow_step_groups::ListWorkflowStepGroupsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_workflow_step_groups::ListWorkflowStepGroupsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_workflow_step_groups::ListWorkflowStepGroupsInput {
                next_token: self.next_token,
                max_results: self.max_results.unwrap_or_default(),
                workflow_id: self.workflow_id,
            },
        )
    }
}
