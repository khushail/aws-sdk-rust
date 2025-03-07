// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::restore_core_network_policy_version::_restore_core_network_policy_version_output::RestoreCoreNetworkPolicyVersionOutputBuilder;

pub use crate::operation::restore_core_network_policy_version::_restore_core_network_policy_version_input::RestoreCoreNetworkPolicyVersionInputBuilder;

/// Fluent builder constructing a request to `RestoreCoreNetworkPolicyVersion`.
///
/// <p>Restores a previous policy version as a new, immutable version of a core network policy. A subsequent change set is created showing the differences between the LIVE policy and restored policy.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RestoreCoreNetworkPolicyVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::restore_core_network_policy_version::builders::RestoreCoreNetworkPolicyVersionInputBuilder,
}
impl RestoreCoreNetworkPolicyVersionFluentBuilder {
    /// Creates a new `RestoreCoreNetworkPolicyVersion`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::restore_core_network_policy_version::RestoreCoreNetworkPolicyVersion, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::restore_core_network_policy_version::RestoreCoreNetworkPolicyVersionError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::restore_core_network_policy_version::RestoreCoreNetworkPolicyVersionOutput, ::aws_smithy_http::result::SdkError<crate::operation::restore_core_network_policy_version::RestoreCoreNetworkPolicyVersionError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::restore_core_network_policy_version::RestoreCoreNetworkPolicyVersionOutput, ::aws_smithy_http::result::SdkError<crate::operation::restore_core_network_policy_version::RestoreCoreNetworkPolicyVersionError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::restore_core_network_policy_version::RestoreCoreNetworkPolicyVersion, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::restore_core_network_policy_version::RestoreCoreNetworkPolicyVersionError>
    >{
        self.customize_middleware().await
    }
    /// <p>The ID of a core network.</p>
    pub fn core_network_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.core_network_id(input.into());
        self
    }
    /// <p>The ID of a core network.</p>
    pub fn set_core_network_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_core_network_id(input);
        self
    }
    /// <p>The ID of the policy version to restore.</p>
    pub fn policy_version_id(mut self, input: i32) -> Self {
        self.inner = self.inner.policy_version_id(input);
        self
    }
    /// <p>The ID of the policy version to restore.</p>
    pub fn set_policy_version_id(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_policy_version_id(input);
        self
    }
}
