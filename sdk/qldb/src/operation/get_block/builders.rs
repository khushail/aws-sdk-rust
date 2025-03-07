// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_block::_get_block_output::GetBlockOutputBuilder;

pub use crate::operation::get_block::_get_block_input::GetBlockInputBuilder;

/// Fluent builder constructing a request to `GetBlock`.
///
/// <p>Returns a block object at a specified address in a journal. Also returns a proof of the specified block for verification if <code>DigestTipAddress</code> is provided.</p>
/// <p>For information about the data contents in a block, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/journal-contents.html">Journal contents</a> in the <i>Amazon QLDB Developer Guide</i>.</p>
/// <p>If the specified ledger doesn't exist or is in <code>DELETING</code> status, then throws <code>ResourceNotFoundException</code>.</p>
/// <p>If the specified ledger is in <code>CREATING</code> status, then throws <code>ResourcePreconditionNotMetException</code>.</p>
/// <p>If no block exists with the specified address, then throws <code>InvalidParameterException</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetBlockFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_block::builders::GetBlockInputBuilder,
}
impl GetBlockFluentBuilder {
    /// Creates a new `GetBlock`.
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
            crate::operation::get_block::GetBlock,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_block::GetBlockError>,
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
        crate::operation::get_block::GetBlockOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::get_block::GetBlockError>,
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
        crate::operation::get_block::GetBlockOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::get_block::GetBlockError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::get_block::GetBlock,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_block::GetBlockError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the ledger.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the ledger.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The location of the block that you want to request. An address is an Amazon Ion structure that has two fields: <code>strandId</code> and <code>sequenceNo</code>.</p>
    /// <p>For example: <code>{strandId:"BlFTjlSXze9BIh1KOszcE3",sequenceNo:14}</code>.</p>
    pub fn block_address(mut self, input: crate::types::ValueHolder) -> Self {
        self.inner = self.inner.block_address(input);
        self
    }
    /// <p>The location of the block that you want to request. An address is an Amazon Ion structure that has two fields: <code>strandId</code> and <code>sequenceNo</code>.</p>
    /// <p>For example: <code>{strandId:"BlFTjlSXze9BIh1KOszcE3",sequenceNo:14}</code>.</p>
    pub fn set_block_address(
        mut self,
        input: ::std::option::Option<crate::types::ValueHolder>,
    ) -> Self {
        self.inner = self.inner.set_block_address(input);
        self
    }
    /// <p>The latest block location covered by the digest for which to request a proof. An address is an Amazon Ion structure that has two fields: <code>strandId</code> and <code>sequenceNo</code>.</p>
    /// <p>For example: <code>{strandId:"BlFTjlSXze9BIh1KOszcE3",sequenceNo:49}</code>.</p>
    pub fn digest_tip_address(mut self, input: crate::types::ValueHolder) -> Self {
        self.inner = self.inner.digest_tip_address(input);
        self
    }
    /// <p>The latest block location covered by the digest for which to request a proof. An address is an Amazon Ion structure that has two fields: <code>strandId</code> and <code>sequenceNo</code>.</p>
    /// <p>For example: <code>{strandId:"BlFTjlSXze9BIh1KOszcE3",sequenceNo:49}</code>.</p>
    pub fn set_digest_tip_address(
        mut self,
        input: ::std::option::Option<crate::types::ValueHolder>,
    ) -> Self {
        self.inner = self.inner.set_digest_tip_address(input);
        self
    }
}
