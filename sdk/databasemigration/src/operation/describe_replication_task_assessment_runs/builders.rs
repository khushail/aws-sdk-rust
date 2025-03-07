// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_replication_task_assessment_runs::_describe_replication_task_assessment_runs_output::DescribeReplicationTaskAssessmentRunsOutputBuilder;

pub use crate::operation::describe_replication_task_assessment_runs::_describe_replication_task_assessment_runs_input::DescribeReplicationTaskAssessmentRunsInputBuilder;

/// Fluent builder constructing a request to `DescribeReplicationTaskAssessmentRuns`.
///
/// <p>Returns a paginated list of premigration assessment runs based on filter settings.</p>
/// <p>These filter settings can specify a combination of premigration assessment runs, migration tasks, replication instances, and assessment run status values.</p> <note>
/// <p>This operation doesn't return information about individual assessments. For this information, see the <code>DescribeReplicationTaskIndividualAssessments</code> operation. </p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeReplicationTaskAssessmentRunsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_replication_task_assessment_runs::builders::DescribeReplicationTaskAssessmentRunsInputBuilder,
}
impl DescribeReplicationTaskAssessmentRunsFluentBuilder {
    /// Creates a new `DescribeReplicationTaskAssessmentRuns`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::describe_replication_task_assessment_runs::DescribeReplicationTaskAssessmentRuns, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::describe_replication_task_assessment_runs::DescribeReplicationTaskAssessmentRunsError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::describe_replication_task_assessment_runs::DescribeReplicationTaskAssessmentRunsOutput, ::aws_smithy_http::result::SdkError<crate::operation::describe_replication_task_assessment_runs::DescribeReplicationTaskAssessmentRunsError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::describe_replication_task_assessment_runs::DescribeReplicationTaskAssessmentRunsOutput, ::aws_smithy_http::result::SdkError<crate::operation::describe_replication_task_assessment_runs::DescribeReplicationTaskAssessmentRunsError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::describe_replication_task_assessment_runs::DescribeReplicationTaskAssessmentRuns, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::describe_replication_task_assessment_runs::DescribeReplicationTaskAssessmentRunsError>
    >{
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_replication_task_assessment_runs::paginator::DescribeReplicationTaskAssessmentRunsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_replication_task_assessment_runs::paginator::DescribeReplicationTaskAssessmentRunsPaginator{
        crate::operation::describe_replication_task_assessment_runs::paginator::DescribeReplicationTaskAssessmentRunsPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>Filters applied to the premigration assessment runs described in the form of key-value pairs.</p>
    /// <p>Valid filter names: <code>replication-task-assessment-run-arn</code>, <code>replication-task-arn</code>, <code>replication-instance-arn</code>, <code>status</code> </p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>Filters applied to the premigration assessment runs described in the form of key-value pairs.</p>
    /// <p>Valid filter names: <code>replication-task-assessment-run-arn</code>, <code>replication-task-arn</code>, <code>replication-instance-arn</code>, <code>status</code> </p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.inner = self.inner.max_records(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p>
    pub fn set_max_records(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_records(input);
        self
    }
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
}
