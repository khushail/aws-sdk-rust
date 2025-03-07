// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_contexts::_list_contexts_output::ListContextsOutputBuilder;

pub use crate::operation::list_contexts::_list_contexts_input::ListContextsInputBuilder;

/// Fluent builder constructing a request to `ListContexts`.
///
/// <p>Lists the contexts in your account and their properties.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListContextsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_contexts::builders::ListContextsInputBuilder,
}
impl ListContextsFluentBuilder {
    /// Creates a new `ListContexts`.
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
            crate::operation::list_contexts::ListContexts,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_contexts::ListContextsError>,
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
        crate::operation::list_contexts::ListContextsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_contexts::ListContextsError>,
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
        crate::operation::list_contexts::ListContextsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_contexts::ListContextsError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_contexts::ListContexts,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_contexts::ListContextsError>,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_contexts::paginator::ListContextsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_contexts::paginator::ListContextsPaginator {
        crate::operation::list_contexts::paginator::ListContextsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>A filter that returns only contexts with the specified source URI.</p>
    pub fn source_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_uri(input.into());
        self
    }
    /// <p>A filter that returns only contexts with the specified source URI.</p>
    pub fn set_source_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_uri(input);
        self
    }
    /// <p>A filter that returns only contexts of the specified type.</p>
    pub fn context_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.context_type(input.into());
        self
    }
    /// <p>A filter that returns only contexts of the specified type.</p>
    pub fn set_context_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_context_type(input);
        self
    }
    /// <p>A filter that returns only contexts created on or after the specified time.</p>
    pub fn created_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.created_after(input);
        self
    }
    /// <p>A filter that returns only contexts created on or after the specified time.</p>
    pub fn set_created_after(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_created_after(input);
        self
    }
    /// <p>A filter that returns only contexts created on or before the specified time.</p>
    pub fn created_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.created_before(input);
        self
    }
    /// <p>A filter that returns only contexts created on or before the specified time.</p>
    pub fn set_created_before(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_created_before(input);
        self
    }
    /// <p>The property used to sort results. The default value is <code>CreationTime</code>.</p>
    pub fn sort_by(mut self, input: crate::types::SortContextsBy) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>The property used to sort results. The default value is <code>CreationTime</code>.</p>
    pub fn set_sort_by(
        mut self,
        input: ::std::option::Option<crate::types::SortContextsBy>,
    ) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>The sort order. The default value is <code>Descending</code>.</p>
    pub fn sort_order(mut self, input: crate::types::SortOrder) -> Self {
        self.inner = self.inner.sort_order(input);
        self
    }
    /// <p>The sort order. The default value is <code>Descending</code>.</p>
    pub fn set_sort_order(mut self, input: ::std::option::Option<crate::types::SortOrder>) -> Self {
        self.inner = self.inner.set_sort_order(input);
        self
    }
    /// <p>If the previous call to <code>ListContexts</code> didn't return the full set of contexts, the call returns a token for getting the next set of contexts.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the previous call to <code>ListContexts</code> didn't return the full set of contexts, the call returns a token for getting the next set of contexts.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of contexts to return in the response. The default value is 10.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of contexts to return in the response. The default value is 10.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
