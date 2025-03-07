// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_tags_for_domain::_list_tags_for_domain_output::ListTagsForDomainOutputBuilder;

pub use crate::operation::list_tags_for_domain::_list_tags_for_domain_input::ListTagsForDomainInputBuilder;

/// Fluent builder constructing a request to `ListTagsForDomain`.
///
/// <p>This operation returns all of the tags that are associated with the specified domain.</p>
/// <p>All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListTagsForDomainFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_tags_for_domain::builders::ListTagsForDomainInputBuilder,
}
impl ListTagsForDomainFluentBuilder {
    /// Creates a new `ListTagsForDomain`.
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
            crate::operation::list_tags_for_domain::ListTagsForDomain,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_tags_for_domain::ListTagsForDomainError,
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
        crate::operation::list_tags_for_domain::ListTagsForDomainOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_tags_for_domain::ListTagsForDomainError,
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
        crate::operation::list_tags_for_domain::ListTagsForDomainOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_tags_for_domain::ListTagsForDomainError,
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
            crate::operation::list_tags_for_domain::ListTagsForDomain,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_tags_for_domain::ListTagsForDomainError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The domain for which you want to get a list of tags.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The domain for which you want to get a list of tags.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
}
