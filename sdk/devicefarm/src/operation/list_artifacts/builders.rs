// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_artifacts::_list_artifacts_output::ListArtifactsOutputBuilder;

pub use crate::operation::list_artifacts::_list_artifacts_input::ListArtifactsInputBuilder;

/// Fluent builder constructing a request to `ListArtifacts`.
///
/// <p>Gets information about artifacts.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListArtifactsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_artifacts::builders::ListArtifactsInputBuilder,
}
impl ListArtifactsFluentBuilder {
    /// Creates a new `ListArtifacts`.
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
            crate::operation::list_artifacts::ListArtifacts,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_artifacts::ListArtifactsError>,
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
        crate::operation::list_artifacts::ListArtifactsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_artifacts::ListArtifactsError>,
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
        crate::operation::list_artifacts::ListArtifactsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_artifacts::ListArtifactsError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_artifacts::ListArtifacts,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_artifacts::ListArtifactsError>,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_artifacts::paginator::ListArtifactsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_artifacts::paginator::ListArtifactsPaginator {
        crate::operation::list_artifacts::paginator::ListArtifactsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The run, job, suite, or test ARN.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// <p>The run, job, suite, or test ARN.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
    /// <p>The artifacts' type.</p>
    /// <p>Allowed values include:</p>
    /// <ul>
    /// <li> <p>FILE</p> </li>
    /// <li> <p>LOG</p> </li>
    /// <li> <p>SCREENSHOT</p> </li>
    /// </ul>
    pub fn r#type(mut self, input: crate::types::ArtifactCategory) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The artifacts' type.</p>
    /// <p>Allowed values include:</p>
    /// <ul>
    /// <li> <p>FILE</p> </li>
    /// <li> <p>LOG</p> </li>
    /// <li> <p>SCREENSHOT</p> </li>
    /// </ul>
    pub fn set_type(
        mut self,
        input: ::std::option::Option<crate::types::ArtifactCategory>,
    ) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
