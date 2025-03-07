// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_hub_content_versions::_list_hub_content_versions_output::ListHubContentVersionsOutputBuilder;

pub use crate::operation::list_hub_content_versions::_list_hub_content_versions_input::ListHubContentVersionsInputBuilder;

/// Fluent builder constructing a request to `ListHubContentVersions`.
///
/// <p>List hub content versions.</p> <note>
/// <p>Hub APIs are only callable through SageMaker Studio.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListHubContentVersionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::list_hub_content_versions::builders::ListHubContentVersionsInputBuilder,
}
impl ListHubContentVersionsFluentBuilder {
    /// Creates a new `ListHubContentVersions`.
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
            crate::operation::list_hub_content_versions::ListHubContentVersions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_hub_content_versions::ListHubContentVersionsError,
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
        crate::operation::list_hub_content_versions::ListHubContentVersionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_hub_content_versions::ListHubContentVersionsError,
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
        crate::operation::list_hub_content_versions::ListHubContentVersionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_hub_content_versions::ListHubContentVersionsError,
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
            crate::operation::list_hub_content_versions::ListHubContentVersions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_hub_content_versions::ListHubContentVersionsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the hub to list the content versions of.</p>
    pub fn hub_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.hub_name(input.into());
        self
    }
    /// <p>The name of the hub to list the content versions of.</p>
    pub fn set_hub_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_hub_name(input);
        self
    }
    /// <p>The type of hub content to list versions of.</p>
    pub fn hub_content_type(mut self, input: crate::types::HubContentType) -> Self {
        self.inner = self.inner.hub_content_type(input);
        self
    }
    /// <p>The type of hub content to list versions of.</p>
    pub fn set_hub_content_type(
        mut self,
        input: ::std::option::Option<crate::types::HubContentType>,
    ) -> Self {
        self.inner = self.inner.set_hub_content_type(input);
        self
    }
    /// <p>The name of the hub content.</p>
    pub fn hub_content_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.hub_content_name(input.into());
        self
    }
    /// <p>The name of the hub content.</p>
    pub fn set_hub_content_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_hub_content_name(input);
        self
    }
    /// <p>The lower bound of the hub content versions to list.</p>
    pub fn min_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.min_version(input.into());
        self
    }
    /// <p>The lower bound of the hub content versions to list.</p>
    pub fn set_min_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_min_version(input);
        self
    }
    /// <p>The upper bound of the hub content schema version.</p>
    pub fn max_schema_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.max_schema_version(input.into());
        self
    }
    /// <p>The upper bound of the hub content schema version.</p>
    pub fn set_max_schema_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_max_schema_version(input);
        self
    }
    /// <p>Only list hub content versions that were created before the time specified.</p>
    pub fn creation_time_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_before(input);
        self
    }
    /// <p>Only list hub content versions that were created before the time specified.</p>
    pub fn set_creation_time_before(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_creation_time_before(input);
        self
    }
    /// <p>Only list hub content versions that were created after the time specified.</p>
    pub fn creation_time_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_after(input);
        self
    }
    /// <p>Only list hub content versions that were created after the time specified.</p>
    pub fn set_creation_time_after(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_creation_time_after(input);
        self
    }
    /// <p>Sort hub content versions by either name or creation time.</p>
    pub fn sort_by(mut self, input: crate::types::HubContentSortBy) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>Sort hub content versions by either name or creation time.</p>
    pub fn set_sort_by(
        mut self,
        input: ::std::option::Option<crate::types::HubContentSortBy>,
    ) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>Sort hub content versions by ascending or descending order.</p>
    pub fn sort_order(mut self, input: crate::types::SortOrder) -> Self {
        self.inner = self.inner.sort_order(input);
        self
    }
    /// <p>Sort hub content versions by ascending or descending order.</p>
    pub fn set_sort_order(mut self, input: ::std::option::Option<crate::types::SortOrder>) -> Self {
        self.inner = self.inner.set_sort_order(input);
        self
    }
    /// <p>The maximum number of hub content versions to list.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of hub content versions to list.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>If the response to a previous <code>ListHubContentVersions</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of hub content versions, use the token in the next request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the response to a previous <code>ListHubContentVersions</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of hub content versions, use the token in the next request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
