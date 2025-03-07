// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::search_databases_by_lf_tags::_search_databases_by_lf_tags_output::SearchDatabasesByLfTagsOutputBuilder;

pub use crate::operation::search_databases_by_lf_tags::_search_databases_by_lf_tags_input::SearchDatabasesByLfTagsInputBuilder;

/// Fluent builder constructing a request to `SearchDatabasesByLFTags`.
///
/// <p>This operation allows a search on <code>DATABASE</code> resources by <code>TagCondition</code>. This operation is used by admins who want to grant user permissions on certain <code>TagConditions</code>. Before making a grant, the admin can use <code>SearchDatabasesByTags</code> to find all resources where the given <code>TagConditions</code> are valid to verify whether the returned resources can be shared.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SearchDatabasesByLFTagsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::search_databases_by_lf_tags::builders::SearchDatabasesByLfTagsInputBuilder,
}
impl SearchDatabasesByLFTagsFluentBuilder {
    /// Creates a new `SearchDatabasesByLFTags`.
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
            crate::operation::search_databases_by_lf_tags::SearchDatabasesByLFTags,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::search_databases_by_lf_tags::SearchDatabasesByLFTagsError,
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
        crate::operation::search_databases_by_lf_tags::SearchDatabasesByLfTagsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::search_databases_by_lf_tags::SearchDatabasesByLFTagsError,
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
        crate::operation::search_databases_by_lf_tags::SearchDatabasesByLfTagsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::search_databases_by_lf_tags::SearchDatabasesByLFTagsError,
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
            crate::operation::search_databases_by_lf_tags::SearchDatabasesByLFTags,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::search_databases_by_lf_tags::SearchDatabasesByLFTagsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::search_databases_by_lf_tags::paginator::SearchDatabasesByLfTagsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::search_databases_by_lf_tags::paginator::SearchDatabasesByLfTagsPaginator
    {
        crate::operation::search_databases_by_lf_tags::paginator::SearchDatabasesByLfTagsPaginator::new(self.handle, self.inner)
    }
    /// <p>A continuation token, if this is not the first call to retrieve this list.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A continuation token, if this is not the first call to retrieve this list.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p>
    pub fn catalog_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.catalog_id(input.into());
        self
    }
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p>
    pub fn set_catalog_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_catalog_id(input);
        self
    }
    /// Appends an item to `Expression`.
    ///
    /// To override the contents of this collection use [`set_expression`](Self::set_expression).
    ///
    /// <p>A list of conditions (<code>LFTag</code> structures) to search for in database resources.</p>
    pub fn expression(mut self, input: crate::types::LfTag) -> Self {
        self.inner = self.inner.expression(input);
        self
    }
    /// <p>A list of conditions (<code>LFTag</code> structures) to search for in database resources.</p>
    pub fn set_expression(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::LfTag>>,
    ) -> Self {
        self.inner = self.inner.set_expression(input);
        self
    }
}
