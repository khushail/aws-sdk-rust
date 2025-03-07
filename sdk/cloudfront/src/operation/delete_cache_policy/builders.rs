// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_cache_policy::_delete_cache_policy_output::DeleteCachePolicyOutputBuilder;

pub use crate::operation::delete_cache_policy::_delete_cache_policy_input::DeleteCachePolicyInputBuilder;

/// Fluent builder constructing a request to `DeleteCachePolicy`.
///
/// <p>Deletes a cache policy.</p>
/// <p>You cannot delete a cache policy if it's attached to a cache behavior. First update your distributions to remove the cache policy from all cache behaviors, then delete the cache policy.</p>
/// <p>To delete a cache policy, you must provide the policy's identifier and version. To get these values, you can use <code>ListCachePolicies</code> or <code>GetCachePolicy</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteCachePolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_cache_policy::builders::DeleteCachePolicyInputBuilder,
}
impl DeleteCachePolicyFluentBuilder {
    /// Creates a new `DeleteCachePolicy`.
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
            crate::operation::delete_cache_policy::DeleteCachePolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_cache_policy::DeleteCachePolicyError,
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
        crate::operation::delete_cache_policy::DeleteCachePolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_cache_policy::DeleteCachePolicyError,
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
        crate::operation::delete_cache_policy::DeleteCachePolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_cache_policy::DeleteCachePolicyError,
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
            crate::operation::delete_cache_policy::DeleteCachePolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_cache_policy::DeleteCachePolicyError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique identifier for the cache policy that you are deleting. To get the identifier, you can use <code>ListCachePolicies</code>.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The unique identifier for the cache policy that you are deleting. To get the identifier, you can use <code>ListCachePolicies</code>.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The version of the cache policy that you are deleting. The version is the cache policy's <code>ETag</code> value, which you can get using <code>ListCachePolicies</code>, <code>GetCachePolicy</code>, or <code>GetCachePolicyConfig</code>.</p>
    pub fn if_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.if_match(input.into());
        self
    }
    /// <p>The version of the cache policy that you are deleting. The version is the cache policy's <code>ETag</code> value, which you can get using <code>ListCachePolicies</code>, <code>GetCachePolicy</code>, or <code>GetCachePolicyConfig</code>.</p>
    pub fn set_if_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_if_match(input);
        self
    }
}
