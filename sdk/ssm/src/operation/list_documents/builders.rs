// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_documents::_list_documents_output::ListDocumentsOutputBuilder;

pub use crate::operation::list_documents::_list_documents_input::ListDocumentsInputBuilder;

/// Fluent builder constructing a request to `ListDocuments`.
///
/// <p>Returns all Systems Manager (SSM) documents in the current Amazon Web Services account and Amazon Web Services Region. You can limit the results of this request by using a filter.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListDocumentsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_documents::builders::ListDocumentsInputBuilder,
}
impl ListDocumentsFluentBuilder {
    /// Creates a new `ListDocuments`.
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
            crate::operation::list_documents::ListDocuments,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_documents::ListDocumentsError>,
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
        crate::operation::list_documents::ListDocumentsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_documents::ListDocumentsError>,
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
        crate::operation::list_documents::ListDocumentsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_documents::ListDocumentsError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_documents::ListDocuments,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_documents::ListDocumentsError>,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_documents::paginator::ListDocumentsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_documents::paginator::ListDocumentsPaginator {
        crate::operation::list_documents::paginator::ListDocumentsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// Appends an item to `DocumentFilterList`.
    ///
    /// To override the contents of this collection use [`set_document_filter_list`](Self::set_document_filter_list).
    ///
    /// <p>This data type is deprecated. Instead, use <code>Filters</code>.</p>
    pub fn document_filter_list(mut self, input: crate::types::DocumentFilter) -> Self {
        self.inner = self.inner.document_filter_list(input);
        self
    }
    /// <p>This data type is deprecated. Instead, use <code>Filters</code>.</p>
    pub fn set_document_filter_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DocumentFilter>>,
    ) -> Self {
        self.inner = self.inner.set_document_filter_list(input);
        self
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more <code>DocumentKeyValuesFilter</code> objects. Use a filter to return a more specific list of results. For keys, you can specify one or more key-value pair tags that have been applied to a document. Other valid keys include <code>Owner</code>, <code>Name</code>, <code>PlatformTypes</code>, <code>DocumentType</code>, and <code>TargetType</code>. For example, to return documents you own use <code>Key=Owner,Values=Self</code>. To specify a custom key-value pair, use the format <code>Key=tag:tagName,Values=valueName</code>.</p> <note>
    /// <p>This API operation only supports filtering documents by using a single tag key and one or more tag values. For example: <code>Key=tag:tagName,Values=valueName1,valueName2</code> </p>
    /// </note>
    pub fn filters(mut self, input: crate::types::DocumentKeyValuesFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>One or more <code>DocumentKeyValuesFilter</code> objects. Use a filter to return a more specific list of results. For keys, you can specify one or more key-value pair tags that have been applied to a document. Other valid keys include <code>Owner</code>, <code>Name</code>, <code>PlatformTypes</code>, <code>DocumentType</code>, and <code>TargetType</code>. For example, to return documents you own use <code>Key=Owner,Values=Self</code>. To specify a custom key-value pair, use the format <code>Key=tag:tagName,Values=valueName</code>.</p> <note>
    /// <p>This API operation only supports filtering documents by using a single tag key and one or more tag values. For example: <code>Key=tag:tagName,Values=valueName1,valueName2</code> </p>
    /// </note>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DocumentKeyValuesFilter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
