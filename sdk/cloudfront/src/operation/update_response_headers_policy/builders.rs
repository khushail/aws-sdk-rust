// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_response_headers_policy::_update_response_headers_policy_output::UpdateResponseHeadersPolicyOutputBuilder;

pub use crate::operation::update_response_headers_policy::_update_response_headers_policy_input::UpdateResponseHeadersPolicyInputBuilder;

/// Fluent builder constructing a request to `UpdateResponseHeadersPolicy`.
///
/// <p>Updates a response headers policy.</p>
/// <p>When you update a response headers policy, the entire policy is replaced. You cannot update some policy fields independent of others. To update a response headers policy configuration:</p>
/// <ol>
/// <li> <p>Use <code>GetResponseHeadersPolicyConfig</code> to get the current policy's configuration.</p> </li>
/// <li> <p>Modify the fields in the response headers policy configuration that you want to update.</p> </li>
/// <li> <p>Call <code>UpdateResponseHeadersPolicy</code>, providing the entire response headers policy configuration, including the fields that you modified and those that you didn't.</p> </li>
/// </ol>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateResponseHeadersPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_response_headers_policy::builders::UpdateResponseHeadersPolicyInputBuilder,
}
impl UpdateResponseHeadersPolicyFluentBuilder {
    /// Creates a new `UpdateResponseHeadersPolicy`.
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
            crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyError,
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
        crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyError,
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
        crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyError,
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
            crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>A response headers policy configuration.</p>
    pub fn response_headers_policy_config(
        mut self,
        input: crate::types::ResponseHeadersPolicyConfig,
    ) -> Self {
        self.inner = self.inner.response_headers_policy_config(input);
        self
    }
    /// <p>A response headers policy configuration.</p>
    pub fn set_response_headers_policy_config(
        mut self,
        input: ::std::option::Option<crate::types::ResponseHeadersPolicyConfig>,
    ) -> Self {
        self.inner = self.inner.set_response_headers_policy_config(input);
        self
    }
    /// <p>The identifier for the response headers policy that you are updating.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The identifier for the response headers policy that you are updating.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The version of the response headers policy that you are updating.</p>
    /// <p>The version is returned in the cache policy's <code>ETag</code> field in the response to <code>GetResponseHeadersPolicyConfig</code>.</p>
    pub fn if_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.if_match(input.into());
        self
    }
    /// <p>The version of the response headers policy that you are updating.</p>
    /// <p>The version is returned in the cache policy's <code>ETag</code> field in the response to <code>GetResponseHeadersPolicyConfig</code>.</p>
    pub fn set_if_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_if_match(input);
        self
    }
}
