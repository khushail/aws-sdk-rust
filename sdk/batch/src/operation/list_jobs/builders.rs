// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_jobs::_list_jobs_output::ListJobsOutputBuilder;

pub use crate::operation::list_jobs::_list_jobs_input::ListJobsInputBuilder;

/// Fluent builder constructing a request to `ListJobs`.
///
/// <p>Returns a list of Batch jobs.</p>
/// <p>You must specify only one of the following items:</p>
/// <ul>
/// <li> <p>A job queue ID to return a list of jobs in that job queue</p> </li>
/// <li> <p>A multi-node parallel job ID to return a list of nodes for that job</p> </li>
/// <li> <p>An array job ID to return a list of the children for that job</p> </li>
/// </ul>
/// <p>You can filter the results by job status with the <code>jobStatus</code> parameter. If you don't specify a status, only <code>RUNNING</code> jobs are returned.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListJobsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_jobs::builders::ListJobsInputBuilder,
}
impl ListJobsFluentBuilder {
    /// Creates a new `ListJobs`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_jobs::ListJobs,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_jobs::ListJobsError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_jobs::ListJobsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_jobs::ListJobsError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_jobs::ListJobsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_jobs::ListJobsError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_jobs::ListJobs,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_jobs::ListJobsError>,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_jobs::paginator::ListJobsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_jobs::paginator::ListJobsPaginator {
        crate::operation::list_jobs::paginator::ListJobsPaginator::new(self.handle, self.inner)
    }
    /// <p>The name or full Amazon Resource Name (ARN) of the job queue used to list jobs.</p>
    pub fn job_queue(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.job_queue(input.into());
        self
    }
    /// <p>The name or full Amazon Resource Name (ARN) of the job queue used to list jobs.</p>
    pub fn set_job_queue(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_job_queue(input);
        self
    }
    /// <p>The job ID for an array job. Specifying an array job ID with this parameter lists all child jobs from within the specified array.</p>
    pub fn array_job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.array_job_id(input.into());
        self
    }
    /// <p>The job ID for an array job. Specifying an array job ID with this parameter lists all child jobs from within the specified array.</p>
    pub fn set_array_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_array_job_id(input);
        self
    }
    /// <p>The job ID for a multi-node parallel job. Specifying a multi-node parallel job ID with this parameter lists all nodes that are associated with the specified job.</p>
    pub fn multi_node_job_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.multi_node_job_id(input.into());
        self
    }
    /// <p>The job ID for a multi-node parallel job. Specifying a multi-node parallel job ID with this parameter lists all nodes that are associated with the specified job.</p>
    pub fn set_multi_node_job_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_multi_node_job_id(input);
        self
    }
    /// <p>The job status used to filter jobs in the specified queue. If the <code>filters</code> parameter is specified, the <code>jobStatus</code> parameter is ignored and jobs with any status are returned. If you don't specify a status, only <code>RUNNING</code> jobs are returned.</p>
    pub fn job_status(mut self, input: crate::types::JobStatus) -> Self {
        self.inner = self.inner.job_status(input);
        self
    }
    /// <p>The job status used to filter jobs in the specified queue. If the <code>filters</code> parameter is specified, the <code>jobStatus</code> parameter is ignored and jobs with any status are returned. If you don't specify a status, only <code>RUNNING</code> jobs are returned.</p>
    pub fn set_job_status(mut self, input: ::std::option::Option<crate::types::JobStatus>) -> Self {
        self.inner = self.inner.set_job_status(input);
        self
    }
    /// <p>The maximum number of results returned by <code>ListJobs</code> in paginated output. When this parameter is used, <code>ListJobs</code> only returns <code>maxResults</code> results in a single page and a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListJobs</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results returned by <code>ListJobs</code> in paginated output. When this parameter is used, <code>ListJobs</code> only returns <code>maxResults</code> results in a single page and a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListJobs</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListJobs</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note>
    /// <p>Treat this token as an opaque identifier that's only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListJobs</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note>
    /// <p>Treat this token as an opaque identifier that's only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filter to apply to the query. Only one filter can be used at a time. When the filter is used, <code>jobStatus</code> is ignored. The filter doesn't apply to child jobs in an array or multi-node parallel (MNP) jobs. The results are sorted by the <code>createdAt</code> field, with the most recent jobs being first.</p>
    /// <dl>
    /// <dt>
    /// JOB_NAME
    /// </dt>
    /// <dd>
    /// <p>The value of the filter is a case-insensitive match for the job name. If the value ends with an asterisk (*), the filter matches any job name that begins with the string before the '*'. This corresponds to the <code>jobName</code> value. For example, <code>test1</code> matches both <code>Test1</code> and <code>test1</code>, and <code>test1*</code> matches both <code>test1</code> and <code>Test10</code>. When the <code>JOB_NAME</code> filter is used, the results are grouped by the job name and version.</p>
    /// </dd>
    /// <dt>
    /// JOB_DEFINITION
    /// </dt>
    /// <dd>
    /// <p>The value for the filter is the name or Amazon Resource Name (ARN) of the job definition. This corresponds to the <code>jobDefinition</code> value. The value is case sensitive. When the value for the filter is the job definition name, the results include all the jobs that used any revision of that job definition name. If the value ends with an asterisk (*), the filter matches any job definition name that begins with the string before the '*'. For example, <code>jd1</code> matches only <code>jd1</code>, and <code>jd1*</code> matches both <code>jd1</code> and <code>jd1A</code>. The version of the job definition that's used doesn't affect the sort order. When the <code>JOB_DEFINITION</code> filter is used and the ARN is used (which is in the form <code>arn:${Partition}:batch:${Region}:${Account}:job-definition/${JobDefinitionName}:${Revision}</code>), the results include jobs that used the specified revision of the job definition. Asterisk (*) isn't supported when the ARN is used.</p>
    /// </dd>
    /// <dt>
    /// BEFORE_CREATED_AT
    /// </dt>
    /// <dd>
    /// <p>The value for the filter is the time that's before the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p>
    /// </dd>
    /// <dt>
    /// AFTER_CREATED_AT
    /// </dt>
    /// <dd>
    /// <p>The value for the filter is the time that's after the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p>
    /// </dd>
    /// </dl>
    pub fn filters(mut self, input: crate::types::KeyValuesPair) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>The filter to apply to the query. Only one filter can be used at a time. When the filter is used, <code>jobStatus</code> is ignored. The filter doesn't apply to child jobs in an array or multi-node parallel (MNP) jobs. The results are sorted by the <code>createdAt</code> field, with the most recent jobs being first.</p>
    /// <dl>
    /// <dt>
    /// JOB_NAME
    /// </dt>
    /// <dd>
    /// <p>The value of the filter is a case-insensitive match for the job name. If the value ends with an asterisk (*), the filter matches any job name that begins with the string before the '*'. This corresponds to the <code>jobName</code> value. For example, <code>test1</code> matches both <code>Test1</code> and <code>test1</code>, and <code>test1*</code> matches both <code>test1</code> and <code>Test10</code>. When the <code>JOB_NAME</code> filter is used, the results are grouped by the job name and version.</p>
    /// </dd>
    /// <dt>
    /// JOB_DEFINITION
    /// </dt>
    /// <dd>
    /// <p>The value for the filter is the name or Amazon Resource Name (ARN) of the job definition. This corresponds to the <code>jobDefinition</code> value. The value is case sensitive. When the value for the filter is the job definition name, the results include all the jobs that used any revision of that job definition name. If the value ends with an asterisk (*), the filter matches any job definition name that begins with the string before the '*'. For example, <code>jd1</code> matches only <code>jd1</code>, and <code>jd1*</code> matches both <code>jd1</code> and <code>jd1A</code>. The version of the job definition that's used doesn't affect the sort order. When the <code>JOB_DEFINITION</code> filter is used and the ARN is used (which is in the form <code>arn:${Partition}:batch:${Region}:${Account}:job-definition/${JobDefinitionName}:${Revision}</code>), the results include jobs that used the specified revision of the job definition. Asterisk (*) isn't supported when the ARN is used.</p>
    /// </dd>
    /// <dt>
    /// BEFORE_CREATED_AT
    /// </dt>
    /// <dd>
    /// <p>The value for the filter is the time that's before the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p>
    /// </dd>
    /// <dt>
    /// AFTER_CREATED_AT
    /// </dt>
    /// <dd>
    /// <p>The value for the filter is the time that's after the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p>
    /// </dd>
    /// </dl>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::KeyValuesPair>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
}
