// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListOpenWorkflowExecutionsInput {
    /// <p>The name of the domain that contains the workflow executions to list.</p>
    #[doc(hidden)]
    pub domain: ::std::option::Option<::std::string::String>,
    /// <p>Workflow executions are included in the returned results based on whether their start times are within the range specified by this filter.</p>
    #[doc(hidden)]
    pub start_time_filter: ::std::option::Option<crate::types::ExecutionTimeFilter>,
    /// <p>If specified, only executions of the type specified in the filter are returned.</p> <note>
    /// <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p>
    /// </note>
    #[doc(hidden)]
    pub type_filter: ::std::option::Option<crate::types::WorkflowTypeFilter>,
    /// <p>If specified, only executions that have the matching tag are listed.</p> <note>
    /// <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p>
    /// </note>
    #[doc(hidden)]
    pub tag_filter: ::std::option::Option<crate::types::TagFilter>,
    /// <p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return a <code>400</code> error: "<code>Specified token has exceeded its maximum lifetime</code>". </p>
    /// <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call. </p>
    #[doc(hidden)]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results. </p>
    #[doc(hidden)]
    pub maximum_page_size: ::std::option::Option<i32>,
    /// <p>When set to <code>true</code>, returns the results in reverse order. By default the results are returned in descending order of the start time of the executions.</p>
    #[doc(hidden)]
    pub reverse_order: ::std::option::Option<bool>,
    /// <p>If specified, only workflow executions matching the workflow ID specified in the filter are returned.</p> <note>
    /// <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p>
    /// </note>
    #[doc(hidden)]
    pub execution_filter: ::std::option::Option<crate::types::WorkflowExecutionFilter>,
}
impl ListOpenWorkflowExecutionsInput {
    /// <p>The name of the domain that contains the workflow executions to list.</p>
    pub fn domain(&self) -> ::std::option::Option<&str> {
        self.domain.as_deref()
    }
    /// <p>Workflow executions are included in the returned results based on whether their start times are within the range specified by this filter.</p>
    pub fn start_time_filter(&self) -> ::std::option::Option<&crate::types::ExecutionTimeFilter> {
        self.start_time_filter.as_ref()
    }
    /// <p>If specified, only executions of the type specified in the filter are returned.</p> <note>
    /// <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p>
    /// </note>
    pub fn type_filter(&self) -> ::std::option::Option<&crate::types::WorkflowTypeFilter> {
        self.type_filter.as_ref()
    }
    /// <p>If specified, only executions that have the matching tag are listed.</p> <note>
    /// <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p>
    /// </note>
    pub fn tag_filter(&self) -> ::std::option::Option<&crate::types::TagFilter> {
        self.tag_filter.as_ref()
    }
    /// <p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return a <code>400</code> error: "<code>Specified token has exceeded its maximum lifetime</code>". </p>
    /// <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call. </p>
    pub fn next_page_token(&self) -> ::std::option::Option<&str> {
        self.next_page_token.as_deref()
    }
    /// <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results. </p>
    pub fn maximum_page_size(&self) -> ::std::option::Option<i32> {
        self.maximum_page_size
    }
    /// <p>When set to <code>true</code>, returns the results in reverse order. By default the results are returned in descending order of the start time of the executions.</p>
    pub fn reverse_order(&self) -> ::std::option::Option<bool> {
        self.reverse_order
    }
    /// <p>If specified, only workflow executions matching the workflow ID specified in the filter are returned.</p> <note>
    /// <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p>
    /// </note>
    pub fn execution_filter(
        &self,
    ) -> ::std::option::Option<&crate::types::WorkflowExecutionFilter> {
        self.execution_filter.as_ref()
    }
}
impl ListOpenWorkflowExecutionsInput {
    /// Creates a new builder-style object to manufacture [`ListOpenWorkflowExecutionsInput`](crate::operation::list_open_workflow_executions::ListOpenWorkflowExecutionsInput).
    pub fn builder() -> crate::operation::list_open_workflow_executions::builders::ListOpenWorkflowExecutionsInputBuilder{
        crate::operation::list_open_workflow_executions::builders::ListOpenWorkflowExecutionsInputBuilder::default()
    }
}

/// A builder for [`ListOpenWorkflowExecutionsInput`](crate::operation::list_open_workflow_executions::ListOpenWorkflowExecutionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListOpenWorkflowExecutionsInputBuilder {
    pub(crate) domain: ::std::option::Option<::std::string::String>,
    pub(crate) start_time_filter: ::std::option::Option<crate::types::ExecutionTimeFilter>,
    pub(crate) type_filter: ::std::option::Option<crate::types::WorkflowTypeFilter>,
    pub(crate) tag_filter: ::std::option::Option<crate::types::TagFilter>,
    pub(crate) next_page_token: ::std::option::Option<::std::string::String>,
    pub(crate) maximum_page_size: ::std::option::Option<i32>,
    pub(crate) reverse_order: ::std::option::Option<bool>,
    pub(crate) execution_filter: ::std::option::Option<crate::types::WorkflowExecutionFilter>,
}
impl ListOpenWorkflowExecutionsInputBuilder {
    /// <p>The name of the domain that contains the workflow executions to list.</p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the domain that contains the workflow executions to list.</p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain = input;
        self
    }
    /// <p>Workflow executions are included in the returned results based on whether their start times are within the range specified by this filter.</p>
    pub fn start_time_filter(mut self, input: crate::types::ExecutionTimeFilter) -> Self {
        self.start_time_filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>Workflow executions are included in the returned results based on whether their start times are within the range specified by this filter.</p>
    pub fn set_start_time_filter(
        mut self,
        input: ::std::option::Option<crate::types::ExecutionTimeFilter>,
    ) -> Self {
        self.start_time_filter = input;
        self
    }
    /// <p>If specified, only executions of the type specified in the filter are returned.</p> <note>
    /// <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p>
    /// </note>
    pub fn type_filter(mut self, input: crate::types::WorkflowTypeFilter) -> Self {
        self.type_filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>If specified, only executions of the type specified in the filter are returned.</p> <note>
    /// <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p>
    /// </note>
    pub fn set_type_filter(
        mut self,
        input: ::std::option::Option<crate::types::WorkflowTypeFilter>,
    ) -> Self {
        self.type_filter = input;
        self
    }
    /// <p>If specified, only executions that have the matching tag are listed.</p> <note>
    /// <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p>
    /// </note>
    pub fn tag_filter(mut self, input: crate::types::TagFilter) -> Self {
        self.tag_filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>If specified, only executions that have the matching tag are listed.</p> <note>
    /// <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p>
    /// </note>
    pub fn set_tag_filter(mut self, input: ::std::option::Option<crate::types::TagFilter>) -> Self {
        self.tag_filter = input;
        self
    }
    /// <p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return a <code>400</code> error: "<code>Specified token has exceeded its maximum lifetime</code>". </p>
    /// <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call. </p>
    pub fn next_page_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.next_page_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return a <code>400</code> error: "<code>Specified token has exceeded its maximum lifetime</code>". </p>
    /// <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call. </p>
    pub fn set_next_page_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.next_page_token = input;
        self
    }
    /// <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results. </p>
    pub fn maximum_page_size(mut self, input: i32) -> Self {
        self.maximum_page_size = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results. </p>
    pub fn set_maximum_page_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.maximum_page_size = input;
        self
    }
    /// <p>When set to <code>true</code>, returns the results in reverse order. By default the results are returned in descending order of the start time of the executions.</p>
    pub fn reverse_order(mut self, input: bool) -> Self {
        self.reverse_order = ::std::option::Option::Some(input);
        self
    }
    /// <p>When set to <code>true</code>, returns the results in reverse order. By default the results are returned in descending order of the start time of the executions.</p>
    pub fn set_reverse_order(mut self, input: ::std::option::Option<bool>) -> Self {
        self.reverse_order = input;
        self
    }
    /// <p>If specified, only workflow executions matching the workflow ID specified in the filter are returned.</p> <note>
    /// <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p>
    /// </note>
    pub fn execution_filter(mut self, input: crate::types::WorkflowExecutionFilter) -> Self {
        self.execution_filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>If specified, only workflow executions matching the workflow ID specified in the filter are returned.</p> <note>
    /// <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p>
    /// </note>
    pub fn set_execution_filter(
        mut self,
        input: ::std::option::Option<crate::types::WorkflowExecutionFilter>,
    ) -> Self {
        self.execution_filter = input;
        self
    }
    /// Consumes the builder and constructs a [`ListOpenWorkflowExecutionsInput`](crate::operation::list_open_workflow_executions::ListOpenWorkflowExecutionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_open_workflow_executions::ListOpenWorkflowExecutionsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_open_workflow_executions::ListOpenWorkflowExecutionsInput {
                domain: self.domain,
                start_time_filter: self.start_time_filter,
                type_filter: self.type_filter,
                tag_filter: self.tag_filter,
                next_page_token: self.next_page_token,
                maximum_page_size: self.maximum_page_size,
                reverse_order: self.reverse_order,
                execution_filter: self.execution_filter,
            },
        )
    }
}
