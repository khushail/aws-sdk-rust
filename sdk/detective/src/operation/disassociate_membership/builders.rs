// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_membership::_disassociate_membership_output::DisassociateMembershipOutputBuilder;

pub use crate::operation::disassociate_membership::_disassociate_membership_input::DisassociateMembershipInputBuilder;

/// Fluent builder constructing a request to `DisassociateMembership`.
///
/// <p>Removes the member account from the specified behavior graph. This operation can only be called by an invited member account that has the <code>ENABLED</code> status.</p>
/// <p> <code>DisassociateMembership</code> cannot be called by an organization account in the organization behavior graph. For the organization behavior graph, the Detective administrator account determines which organization accounts to enable or disable as member accounts.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateMembershipFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_membership::builders::DisassociateMembershipInputBuilder,
}
impl DisassociateMembershipFluentBuilder {
    /// Creates a new `DisassociateMembership`.
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
            crate::operation::disassociate_membership::DisassociateMembership,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_membership::DisassociateMembershipError,
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
        crate::operation::disassociate_membership::DisassociateMembershipOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_membership::DisassociateMembershipError,
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
        crate::operation::disassociate_membership::DisassociateMembershipOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_membership::DisassociateMembershipError,
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
            crate::operation::disassociate_membership::DisassociateMembership,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_membership::DisassociateMembershipError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ARN of the behavior graph to remove the member account from.</p>
    /// <p>The member account's member status in the behavior graph must be <code>ENABLED</code>.</p>
    pub fn graph_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.graph_arn(input.into());
        self
    }
    /// <p>The ARN of the behavior graph to remove the member account from.</p>
    /// <p>The member account's member status in the behavior graph must be <code>ENABLED</code>.</p>
    pub fn set_graph_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_graph_arn(input);
        self
    }
}
