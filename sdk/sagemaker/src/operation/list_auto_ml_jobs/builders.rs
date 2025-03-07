// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_auto_ml_jobs::_list_auto_ml_jobs_output::ListAutoMlJobsOutputBuilder;

pub use crate::operation::list_auto_ml_jobs::_list_auto_ml_jobs_input::ListAutoMlJobsInputBuilder;

/// Fluent builder constructing a request to `ListAutoMLJobs`.
///
/// <p>Request a list of jobs.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListAutoMLJobsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_auto_ml_jobs::builders::ListAutoMlJobsInputBuilder,
}
impl ListAutoMLJobsFluentBuilder {
    /// Creates a new `ListAutoMLJobs`.
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
            crate::operation::list_auto_ml_jobs::ListAutoMLJobs,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_auto_ml_jobs::ListAutoMLJobsError,
        >,
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
        crate::operation::list_auto_ml_jobs::ListAutoMlJobsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_auto_ml_jobs::ListAutoMLJobsError,
        >,
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
        crate::operation::list_auto_ml_jobs::ListAutoMlJobsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_auto_ml_jobs::ListAutoMLJobsError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_auto_ml_jobs::ListAutoMLJobs,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_auto_ml_jobs::ListAutoMLJobsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_auto_ml_jobs::paginator::ListAutoMlJobsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_auto_ml_jobs::paginator::ListAutoMlJobsPaginator {
        crate::operation::list_auto_ml_jobs::paginator::ListAutoMlJobsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn creation_time_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_after(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn set_creation_time_after(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_creation_time_after(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn creation_time_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_before(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn set_creation_time_before(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_creation_time_before(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn last_modified_time_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.last_modified_time_after(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn set_last_modified_time_after(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_last_modified_time_after(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn last_modified_time_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.last_modified_time_before(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn set_last_modified_time_before(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_last_modified_time_before(input);
        self
    }
    /// <p>Request a list of jobs, using a search filter for name.</p>
    pub fn name_contains(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.name_contains(input.into());
        self
    }
    /// <p>Request a list of jobs, using a search filter for name.</p>
    pub fn set_name_contains(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_name_contains(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for status.</p>
    pub fn status_equals(mut self, input: crate::types::AutoMlJobStatus) -> Self {
        self.inner = self.inner.status_equals(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for status.</p>
    pub fn set_status_equals(
        mut self,
        input: ::std::option::Option<crate::types::AutoMlJobStatus>,
    ) -> Self {
        self.inner = self.inner.set_status_equals(input);
        self
    }
    /// <p>The sort order for the results. The default is <code>Descending</code>.</p>
    pub fn sort_order(mut self, input: crate::types::AutoMlSortOrder) -> Self {
        self.inner = self.inner.sort_order(input);
        self
    }
    /// <p>The sort order for the results. The default is <code>Descending</code>.</p>
    pub fn set_sort_order(
        mut self,
        input: ::std::option::Option<crate::types::AutoMlSortOrder>,
    ) -> Self {
        self.inner = self.inner.set_sort_order(input);
        self
    }
    /// <p>The parameter by which to sort the results. The default is <code>Name</code>.</p>
    pub fn sort_by(mut self, input: crate::types::AutoMlSortBy) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>The parameter by which to sort the results. The default is <code>Name</code>.</p>
    pub fn set_sort_by(mut self, input: ::std::option::Option<crate::types::AutoMlSortBy>) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>Request a list of jobs up to a specified limit.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Request a list of jobs up to a specified limit.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>If the previous response was truncated, you receive this token. Use it in your next request to receive the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the previous response was truncated, you receive this token. Use it in your next request to receive the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
