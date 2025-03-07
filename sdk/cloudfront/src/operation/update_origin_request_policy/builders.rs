// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_origin_request_policy::_update_origin_request_policy_output::UpdateOriginRequestPolicyOutputBuilder;

pub use crate::operation::update_origin_request_policy::_update_origin_request_policy_input::UpdateOriginRequestPolicyInputBuilder;

/// Fluent builder constructing a request to `UpdateOriginRequestPolicy`.
///
/// <p>Updates an origin request policy configuration.</p>
/// <p>When you update an origin request policy configuration, all the fields are updated with the values provided in the request. You cannot update some fields independent of others. To update an origin request policy configuration:</p>
/// <ol>
/// <li> <p>Use <code>GetOriginRequestPolicyConfig</code> to get the current configuration.</p> </li>
/// <li> <p>Locally modify the fields in the origin request policy configuration that you want to update.</p> </li>
/// <li> <p>Call <code>UpdateOriginRequestPolicy</code> by providing the entire origin request policy configuration, including the fields that you modified and those that you didn't.</p> </li>
/// </ol>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateOriginRequestPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_origin_request_policy::builders::UpdateOriginRequestPolicyInputBuilder,
}
impl UpdateOriginRequestPolicyFluentBuilder {
    /// Creates a new `UpdateOriginRequestPolicy`.
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
            crate::operation::update_origin_request_policy::UpdateOriginRequestPolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_origin_request_policy::UpdateOriginRequestPolicyError,
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
        crate::operation::update_origin_request_policy::UpdateOriginRequestPolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_origin_request_policy::UpdateOriginRequestPolicyError,
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
        crate::operation::update_origin_request_policy::UpdateOriginRequestPolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_origin_request_policy::UpdateOriginRequestPolicyError,
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
            crate::operation::update_origin_request_policy::UpdateOriginRequestPolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_origin_request_policy::UpdateOriginRequestPolicyError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>An origin request policy configuration.</p>
    pub fn origin_request_policy_config(
        mut self,
        input: crate::types::OriginRequestPolicyConfig,
    ) -> Self {
        self.inner = self.inner.origin_request_policy_config(input);
        self
    }
    /// <p>An origin request policy configuration.</p>
    pub fn set_origin_request_policy_config(
        mut self,
        input: ::std::option::Option<crate::types::OriginRequestPolicyConfig>,
    ) -> Self {
        self.inner = self.inner.set_origin_request_policy_config(input);
        self
    }
    /// <p>The unique identifier for the origin request policy that you are updating. The identifier is returned in a cache behavior's <code>OriginRequestPolicyId</code> field in the response to <code>GetDistributionConfig</code>.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The unique identifier for the origin request policy that you are updating. The identifier is returned in a cache behavior's <code>OriginRequestPolicyId</code> field in the response to <code>GetDistributionConfig</code>.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The version of the origin request policy that you are updating. The version is returned in the origin request policy's <code>ETag</code> field in the response to <code>GetOriginRequestPolicyConfig</code>.</p>
    pub fn if_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.if_match(input.into());
        self
    }
    /// <p>The version of the origin request policy that you are updating. The version is returned in the origin request policy's <code>ETag</code> field in the response to <code>GetOriginRequestPolicyConfig</code>.</p>
    pub fn set_if_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_if_match(input);
        self
    }
}
