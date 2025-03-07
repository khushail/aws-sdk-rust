// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetPendingJobExecutionsOutput {
    /// <p>A list of JobExecutionSummary objects with status IN_PROGRESS.</p>
    #[doc(hidden)]
    pub in_progress_jobs: ::std::option::Option<::std::vec::Vec<crate::types::JobExecutionSummary>>,
    /// <p>A list of JobExecutionSummary objects with status QUEUED.</p>
    #[doc(hidden)]
    pub queued_jobs: ::std::option::Option<::std::vec::Vec<crate::types::JobExecutionSummary>>,
    _request_id: Option<String>,
}
impl GetPendingJobExecutionsOutput {
    /// <p>A list of JobExecutionSummary objects with status IN_PROGRESS.</p>
    pub fn in_progress_jobs(&self) -> ::std::option::Option<&[crate::types::JobExecutionSummary]> {
        self.in_progress_jobs.as_deref()
    }
    /// <p>A list of JobExecutionSummary objects with status QUEUED.</p>
    pub fn queued_jobs(&self) -> ::std::option::Option<&[crate::types::JobExecutionSummary]> {
        self.queued_jobs.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetPendingJobExecutionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetPendingJobExecutionsOutput {
    /// Creates a new builder-style object to manufacture [`GetPendingJobExecutionsOutput`](crate::operation::get_pending_job_executions::GetPendingJobExecutionsOutput).
    pub fn builder(
    ) -> crate::operation::get_pending_job_executions::builders::GetPendingJobExecutionsOutputBuilder
    {
        crate::operation::get_pending_job_executions::builders::GetPendingJobExecutionsOutputBuilder::default()
    }
}

/// A builder for [`GetPendingJobExecutionsOutput`](crate::operation::get_pending_job_executions::GetPendingJobExecutionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetPendingJobExecutionsOutputBuilder {
    pub(crate) in_progress_jobs:
        ::std::option::Option<::std::vec::Vec<crate::types::JobExecutionSummary>>,
    pub(crate) queued_jobs:
        ::std::option::Option<::std::vec::Vec<crate::types::JobExecutionSummary>>,
    _request_id: Option<String>,
}
impl GetPendingJobExecutionsOutputBuilder {
    /// Appends an item to `in_progress_jobs`.
    ///
    /// To override the contents of this collection use [`set_in_progress_jobs`](Self::set_in_progress_jobs).
    ///
    /// <p>A list of JobExecutionSummary objects with status IN_PROGRESS.</p>
    pub fn in_progress_jobs(mut self, input: crate::types::JobExecutionSummary) -> Self {
        let mut v = self.in_progress_jobs.unwrap_or_default();
        v.push(input);
        self.in_progress_jobs = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of JobExecutionSummary objects with status IN_PROGRESS.</p>
    pub fn set_in_progress_jobs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::JobExecutionSummary>>,
    ) -> Self {
        self.in_progress_jobs = input;
        self
    }
    /// Appends an item to `queued_jobs`.
    ///
    /// To override the contents of this collection use [`set_queued_jobs`](Self::set_queued_jobs).
    ///
    /// <p>A list of JobExecutionSummary objects with status QUEUED.</p>
    pub fn queued_jobs(mut self, input: crate::types::JobExecutionSummary) -> Self {
        let mut v = self.queued_jobs.unwrap_or_default();
        v.push(input);
        self.queued_jobs = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of JobExecutionSummary objects with status QUEUED.</p>
    pub fn set_queued_jobs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::JobExecutionSummary>>,
    ) -> Self {
        self.queued_jobs = input;
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
    /// Consumes the builder and constructs a [`GetPendingJobExecutionsOutput`](crate::operation::get_pending_job_executions::GetPendingJobExecutionsOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_pending_job_executions::GetPendingJobExecutionsOutput {
        crate::operation::get_pending_job_executions::GetPendingJobExecutionsOutput {
            in_progress_jobs: self.in_progress_jobs,
            queued_jobs: self.queued_jobs,
            _request_id: self._request_id,
        }
    }
}
