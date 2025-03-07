// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_record_history::_list_record_history_output::ListRecordHistoryOutputBuilder;

pub use crate::operation::list_record_history::_list_record_history_input::ListRecordHistoryInputBuilder;

/// Fluent builder constructing a request to `ListRecordHistory`.
///
/// <p>Lists the specified requests or all performed requests.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListRecordHistoryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_record_history::builders::ListRecordHistoryInputBuilder,
}
impl ListRecordHistoryFluentBuilder {
    /// Creates a new `ListRecordHistory`.
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
            crate::operation::list_record_history::ListRecordHistory,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_record_history::ListRecordHistoryError,
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
        crate::operation::list_record_history::ListRecordHistoryOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_record_history::ListRecordHistoryError,
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
        crate::operation::list_record_history::ListRecordHistoryOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_record_history::ListRecordHistoryError,
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
            crate::operation::list_record_history::ListRecordHistory,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_record_history::ListRecordHistoryError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li> <p> <code>jp</code> - Japanese</p> </li>
    /// <li> <p> <code>zh</code> - Chinese</p> </li>
    /// </ul>
    pub fn accept_language(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.accept_language(input.into());
        self
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li> <p> <code>jp</code> - Japanese</p> </li>
    /// <li> <p> <code>zh</code> - Chinese</p> </li>
    /// </ul>
    pub fn set_accept_language(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_accept_language(input);
        self
    }
    /// <p>The access level to use to obtain results. The default is <code>User</code>.</p>
    pub fn access_level_filter(mut self, input: crate::types::AccessLevelFilter) -> Self {
        self.inner = self.inner.access_level_filter(input);
        self
    }
    /// <p>The access level to use to obtain results. The default is <code>User</code>.</p>
    pub fn set_access_level_filter(
        mut self,
        input: ::std::option::Option<crate::types::AccessLevelFilter>,
    ) -> Self {
        self.inner = self.inner.set_access_level_filter(input);
        self
    }
    /// <p>The search filter to scope the results.</p>
    pub fn search_filter(mut self, input: crate::types::ListRecordHistorySearchFilter) -> Self {
        self.inner = self.inner.search_filter(input);
        self
    }
    /// <p>The search filter to scope the results.</p>
    pub fn set_search_filter(
        mut self,
        input: ::std::option::Option<crate::types::ListRecordHistorySearchFilter>,
    ) -> Self {
        self.inner = self.inner.set_search_filter(input);
        self
    }
    /// <p>The maximum number of items to return with this call.</p>
    pub fn page_size(mut self, input: i32) -> Self {
        self.inner = self.inner.page_size(input);
        self
    }
    /// <p>The maximum number of items to return with this call.</p>
    pub fn set_page_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_page_size(input);
        self
    }
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    pub fn page_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.page_token(input.into());
        self
    }
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    pub fn set_page_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_page_token(input);
        self
    }
}
