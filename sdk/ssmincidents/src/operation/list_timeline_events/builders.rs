// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_timeline_events::_list_timeline_events_output::ListTimelineEventsOutputBuilder;

pub use crate::operation::list_timeline_events::_list_timeline_events_input::ListTimelineEventsInputBuilder;

/// Fluent builder constructing a request to `ListTimelineEvents`.
///
/// <p>Lists timeline events for the specified incident record.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListTimelineEventsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_timeline_events::builders::ListTimelineEventsInputBuilder,
}
impl ListTimelineEventsFluentBuilder {
    /// Creates a new `ListTimelineEvents`.
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
            crate::operation::list_timeline_events::ListTimelineEvents,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_timeline_events::ListTimelineEventsError,
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
        crate::operation::list_timeline_events::ListTimelineEventsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_timeline_events::ListTimelineEventsError,
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
        crate::operation::list_timeline_events::ListTimelineEventsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_timeline_events::ListTimelineEventsError,
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
            crate::operation::list_timeline_events::ListTimelineEvents,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_timeline_events::ListTimelineEventsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_timeline_events::paginator::ListTimelineEventsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_timeline_events::paginator::ListTimelineEventsPaginator {
        crate::operation::list_timeline_events::paginator::ListTimelineEventsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The Amazon Resource Name (ARN) of the incident that includes the timeline event.</p>
    pub fn incident_record_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.incident_record_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the incident that includes the timeline event.</p>
    pub fn set_incident_record_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_incident_record_arn(input);
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>Filters the timeline events based on the provided conditional values. You can filter timeline events with the following keys:</p>
    /// <ul>
    /// <li> <p> <code>eventTime</code> </p> </li>
    /// <li> <p> <code>eventType</code> </p> </li>
    /// </ul>
    /// <p>Note the following when deciding how to use Filters:</p>
    /// <ul>
    /// <li> <p>If you don't specify a Filter, the response includes all timeline events.</p> </li>
    /// <li> <p>If you specify more than one filter in a single request, the response returns timeline events that match all filters.</p> </li>
    /// <li> <p>If you specify a filter with more than one value, the response returns timeline events that match any of the values provided.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>Filters the timeline events based on the provided conditional values. You can filter timeline events with the following keys:</p>
    /// <ul>
    /// <li> <p> <code>eventTime</code> </p> </li>
    /// <li> <p> <code>eventType</code> </p> </li>
    /// </ul>
    /// <p>Note the following when deciding how to use Filters:</p>
    /// <ul>
    /// <li> <p>If you don't specify a Filter, the response includes all timeline events.</p> </li>
    /// <li> <p>If you specify more than one filter in a single request, the response returns timeline events that match all filters.</p> </li>
    /// <li> <p>If you specify a filter with more than one value, the response returns timeline events that match any of the values provided.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>Sort timeline events by the specified key value pair.</p>
    pub fn sort_by(mut self, input: crate::types::TimelineEventSort) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>Sort timeline events by the specified key value pair.</p>
    pub fn set_sort_by(
        mut self,
        input: ::std::option::Option<crate::types::TimelineEventSort>,
    ) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>Sorts the order of timeline events by the value specified in the <code>sortBy</code> field.</p>
    pub fn sort_order(mut self, input: crate::types::SortOrder) -> Self {
        self.inner = self.inner.sort_order(input);
        self
    }
    /// <p>Sorts the order of timeline events by the value specified in the <code>sortBy</code> field.</p>
    pub fn set_sort_order(mut self, input: ::std::option::Option<crate::types::SortOrder>) -> Self {
        self.inner = self.inner.set_sort_order(input);
        self
    }
    /// <p>The maximum number of results per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results per page.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The pagination token to continue to the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token to continue to the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
