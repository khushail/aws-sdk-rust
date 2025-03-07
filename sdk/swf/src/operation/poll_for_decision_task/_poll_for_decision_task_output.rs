// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A structure that represents a decision task. Decision tasks are sent to deciders in order for them to make decisions.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PollForDecisionTaskOutput {
    /// <p>The opaque string used as a handle on the task. This token is used by workers to communicate progress and response information back to the system about the task.</p>
    #[doc(hidden)]
    pub task_token: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the <code>DecisionTaskStarted</code> event recorded in the history.</p>
    #[doc(hidden)]
    pub started_event_id: i64,
    /// <p>The workflow execution for which this decision task was created.</p>
    #[doc(hidden)]
    pub workflow_execution: ::std::option::Option<crate::types::WorkflowExecution>,
    /// <p>The type of the workflow execution for which this decision task was created.</p>
    #[doc(hidden)]
    pub workflow_type: ::std::option::Option<crate::types::WorkflowType>,
    /// <p>A paginated list of history events of the workflow execution. The decider uses this during the processing of the decision task.</p>
    #[doc(hidden)]
    pub events: ::std::option::Option<::std::vec::Vec<crate::types::HistoryEvent>>,
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p>
    /// <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    #[doc(hidden)]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the DecisionTaskStarted event of the previous decision task of this workflow execution that was processed by the decider. This can be used to determine the events in the history new since the last decision task received by the decider.</p>
    #[doc(hidden)]
    pub previous_started_event_id: i64,
    _request_id: Option<String>,
}
impl PollForDecisionTaskOutput {
    /// <p>The opaque string used as a handle on the task. This token is used by workers to communicate progress and response information back to the system about the task.</p>
    pub fn task_token(&self) -> ::std::option::Option<&str> {
        self.task_token.as_deref()
    }
    /// <p>The ID of the <code>DecisionTaskStarted</code> event recorded in the history.</p>
    pub fn started_event_id(&self) -> i64 {
        self.started_event_id
    }
    /// <p>The workflow execution for which this decision task was created.</p>
    pub fn workflow_execution(&self) -> ::std::option::Option<&crate::types::WorkflowExecution> {
        self.workflow_execution.as_ref()
    }
    /// <p>The type of the workflow execution for which this decision task was created.</p>
    pub fn workflow_type(&self) -> ::std::option::Option<&crate::types::WorkflowType> {
        self.workflow_type.as_ref()
    }
    /// <p>A paginated list of history events of the workflow execution. The decider uses this during the processing of the decision task.</p>
    pub fn events(&self) -> ::std::option::Option<&[crate::types::HistoryEvent]> {
        self.events.as_deref()
    }
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p>
    /// <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    pub fn next_page_token(&self) -> ::std::option::Option<&str> {
        self.next_page_token.as_deref()
    }
    /// <p>The ID of the DecisionTaskStarted event of the previous decision task of this workflow execution that was processed by the decider. This can be used to determine the events in the history new since the last decision task received by the decider.</p>
    pub fn previous_started_event_id(&self) -> i64 {
        self.previous_started_event_id
    }
}
impl ::aws_http::request_id::RequestId for PollForDecisionTaskOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PollForDecisionTaskOutput {
    /// Creates a new builder-style object to manufacture [`PollForDecisionTaskOutput`](crate::operation::poll_for_decision_task::PollForDecisionTaskOutput).
    pub fn builder(
    ) -> crate::operation::poll_for_decision_task::builders::PollForDecisionTaskOutputBuilder {
        crate::operation::poll_for_decision_task::builders::PollForDecisionTaskOutputBuilder::default()
    }
}

/// A builder for [`PollForDecisionTaskOutput`](crate::operation::poll_for_decision_task::PollForDecisionTaskOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PollForDecisionTaskOutputBuilder {
    pub(crate) task_token: ::std::option::Option<::std::string::String>,
    pub(crate) started_event_id: ::std::option::Option<i64>,
    pub(crate) workflow_execution: ::std::option::Option<crate::types::WorkflowExecution>,
    pub(crate) workflow_type: ::std::option::Option<crate::types::WorkflowType>,
    pub(crate) events: ::std::option::Option<::std::vec::Vec<crate::types::HistoryEvent>>,
    pub(crate) next_page_token: ::std::option::Option<::std::string::String>,
    pub(crate) previous_started_event_id: ::std::option::Option<i64>,
    _request_id: Option<String>,
}
impl PollForDecisionTaskOutputBuilder {
    /// <p>The opaque string used as a handle on the task. This token is used by workers to communicate progress and response information back to the system about the task.</p>
    pub fn task_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.task_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The opaque string used as a handle on the task. This token is used by workers to communicate progress and response information back to the system about the task.</p>
    pub fn set_task_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.task_token = input;
        self
    }
    /// <p>The ID of the <code>DecisionTaskStarted</code> event recorded in the history.</p>
    pub fn started_event_id(mut self, input: i64) -> Self {
        self.started_event_id = ::std::option::Option::Some(input);
        self
    }
    /// <p>The ID of the <code>DecisionTaskStarted</code> event recorded in the history.</p>
    pub fn set_started_event_id(mut self, input: ::std::option::Option<i64>) -> Self {
        self.started_event_id = input;
        self
    }
    /// <p>The workflow execution for which this decision task was created.</p>
    pub fn workflow_execution(mut self, input: crate::types::WorkflowExecution) -> Self {
        self.workflow_execution = ::std::option::Option::Some(input);
        self
    }
    /// <p>The workflow execution for which this decision task was created.</p>
    pub fn set_workflow_execution(
        mut self,
        input: ::std::option::Option<crate::types::WorkflowExecution>,
    ) -> Self {
        self.workflow_execution = input;
        self
    }
    /// <p>The type of the workflow execution for which this decision task was created.</p>
    pub fn workflow_type(mut self, input: crate::types::WorkflowType) -> Self {
        self.workflow_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of the workflow execution for which this decision task was created.</p>
    pub fn set_workflow_type(
        mut self,
        input: ::std::option::Option<crate::types::WorkflowType>,
    ) -> Self {
        self.workflow_type = input;
        self
    }
    /// Appends an item to `events`.
    ///
    /// To override the contents of this collection use [`set_events`](Self::set_events).
    ///
    /// <p>A paginated list of history events of the workflow execution. The decider uses this during the processing of the decision task.</p>
    pub fn events(mut self, input: crate::types::HistoryEvent) -> Self {
        let mut v = self.events.unwrap_or_default();
        v.push(input);
        self.events = ::std::option::Option::Some(v);
        self
    }
    /// <p>A paginated list of history events of the workflow execution. The decider uses this during the processing of the decision task.</p>
    pub fn set_events(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::HistoryEvent>>,
    ) -> Self {
        self.events = input;
        self
    }
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p>
    /// <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    pub fn next_page_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.next_page_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p>
    /// <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    pub fn set_next_page_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.next_page_token = input;
        self
    }
    /// <p>The ID of the DecisionTaskStarted event of the previous decision task of this workflow execution that was processed by the decider. This can be used to determine the events in the history new since the last decision task received by the decider.</p>
    pub fn previous_started_event_id(mut self, input: i64) -> Self {
        self.previous_started_event_id = ::std::option::Option::Some(input);
        self
    }
    /// <p>The ID of the DecisionTaskStarted event of the previous decision task of this workflow execution that was processed by the decider. This can be used to determine the events in the history new since the last decision task received by the decider.</p>
    pub fn set_previous_started_event_id(mut self, input: ::std::option::Option<i64>) -> Self {
        self.previous_started_event_id = input;
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
    /// Consumes the builder and constructs a [`PollForDecisionTaskOutput`](crate::operation::poll_for_decision_task::PollForDecisionTaskOutput).
    pub fn build(self) -> crate::operation::poll_for_decision_task::PollForDecisionTaskOutput {
        crate::operation::poll_for_decision_task::PollForDecisionTaskOutput {
            task_token: self.task_token,
            started_event_id: self.started_event_id.unwrap_or_default(),
            workflow_execution: self.workflow_execution,
            workflow_type: self.workflow_type,
            events: self.events,
            next_page_token: self.next_page_token,
            previous_started_event_id: self.previous_started_event_id.unwrap_or_default(),
            _request_id: self._request_id,
        }
    }
}
