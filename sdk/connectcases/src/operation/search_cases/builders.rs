// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::search_cases::_search_cases_output::SearchCasesOutputBuilder;

pub use crate::operation::search_cases::_search_cases_input::SearchCasesInputBuilder;

/// Fluent builder constructing a request to `SearchCases`.
///
/// <p>Searches for cases within their associated Cases domain. Search results are returned as a paginated list of abridged case documents.</p> <note>
/// <p>For <code>customer_id</code> you must provide the full customer profile ARN in this format: <code> arn:aws:profile:your AWS Region:your AWS account ID:domains/profiles domain name/profiles/profile ID</code>. </p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SearchCasesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::search_cases::builders::SearchCasesInputBuilder,
}
impl SearchCasesFluentBuilder {
    /// Creates a new `SearchCases`.
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
            crate::operation::search_cases::SearchCases,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::search_cases::SearchCasesError>,
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
        crate::operation::search_cases::SearchCasesOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::search_cases::SearchCasesError>,
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
        crate::operation::search_cases::SearchCasesOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::search_cases::SearchCasesError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::search_cases::SearchCases,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::search_cases::SearchCasesError>,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::search_cases::paginator::SearchCasesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::search_cases::paginator::SearchCasesPaginator {
        crate::operation::search_cases::paginator::SearchCasesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The unique identifier of the Cases domain. </p>
    pub fn domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_id(input.into());
        self
    }
    /// <p>The unique identifier of the Cases domain. </p>
    pub fn set_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_id(input);
        self
    }
    /// <p>The maximum number of cases to return. The current maximum supported value is 25. This is also the default value when no other value is provided.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of cases to return. The current maximum supported value is 25. This is also the default value when no other value is provided.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A word or phrase used to perform a quick search.</p>
    pub fn search_term(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.search_term(input.into());
        self
    }
    /// <p>A word or phrase used to perform a quick search.</p>
    pub fn set_search_term(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_search_term(input);
        self
    }
    /// <p>A list of filter objects.</p>
    pub fn filter(mut self, input: crate::types::CaseFilter) -> Self {
        self.inner = self.inner.filter(input);
        self
    }
    /// <p>A list of filter objects.</p>
    pub fn set_filter(mut self, input: ::std::option::Option<crate::types::CaseFilter>) -> Self {
        self.inner = self.inner.set_filter(input);
        self
    }
    /// Appends an item to `sorts`.
    ///
    /// To override the contents of this collection use [`set_sorts`](Self::set_sorts).
    ///
    /// <p>A list of sorts where each sort specifies a field and their sort order to be applied to the results. </p>
    pub fn sorts(mut self, input: crate::types::Sort) -> Self {
        self.inner = self.inner.sorts(input);
        self
    }
    /// <p>A list of sorts where each sort specifies a field and their sort order to be applied to the results. </p>
    pub fn set_sorts(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Sort>>,
    ) -> Self {
        self.inner = self.inner.set_sorts(input);
        self
    }
    /// Appends an item to `fields`.
    ///
    /// To override the contents of this collection use [`set_fields`](Self::set_fields).
    ///
    /// <p>The list of field identifiers to be returned as part of the response.</p>
    pub fn fields(mut self, input: crate::types::FieldIdentifier) -> Self {
        self.inner = self.inner.fields(input);
        self
    }
    /// <p>The list of field identifiers to be returned as part of the response.</p>
    pub fn set_fields(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::FieldIdentifier>>,
    ) -> Self {
        self.inner = self.inner.set_fields(input);
        self
    }
}
