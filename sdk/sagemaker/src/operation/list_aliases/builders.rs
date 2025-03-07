// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_aliases::_list_aliases_output::ListAliasesOutputBuilder;

pub use crate::operation::list_aliases::_list_aliases_input::ListAliasesInputBuilder;

/// Fluent builder constructing a request to `ListAliases`.
///
/// <p>Lists the aliases of a specified image or image version.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListAliasesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_aliases::builders::ListAliasesInputBuilder,
}
impl ListAliasesFluentBuilder {
    /// Creates a new `ListAliases`.
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
            crate::operation::list_aliases::ListAliases,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_aliases::ListAliasesError>,
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
        crate::operation::list_aliases::ListAliasesOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_aliases::ListAliasesError>,
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
        crate::operation::list_aliases::ListAliasesOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_aliases::ListAliasesError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_aliases::ListAliases,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_aliases::ListAliasesError>,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_aliases::paginator::ListAliasesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_aliases::paginator::ListAliasesPaginator {
        crate::operation::list_aliases::paginator::ListAliasesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The name of the image.</p>
    pub fn image_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.image_name(input.into());
        self
    }
    /// <p>The name of the image.</p>
    pub fn set_image_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_image_name(input);
        self
    }
    /// <p>The alias of the image version.</p>
    pub fn alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.alias(input.into());
        self
    }
    /// <p>The alias of the image version.</p>
    pub fn set_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_alias(input);
        self
    }
    /// <p>The version of the image. If image version is not specified, the aliases of all versions of the image are listed.</p>
    pub fn version(mut self, input: i32) -> Self {
        self.inner = self.inner.version(input);
        self
    }
    /// <p>The version of the image. If image version is not specified, the aliases of all versions of the image are listed.</p>
    pub fn set_version(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_version(input);
        self
    }
    /// <p>The maximum number of aliases to return.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of aliases to return.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>If the previous call to <code>ListAliases</code> didn't return the full set of aliases, the call returns a token for retrieving the next set of aliases.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the previous call to <code>ListAliases</code> didn't return the full set of aliases, the call returns a token for retrieving the next set of aliases.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
