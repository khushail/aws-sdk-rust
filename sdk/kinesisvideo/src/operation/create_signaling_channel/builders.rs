// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_signaling_channel::_create_signaling_channel_output::CreateSignalingChannelOutputBuilder;

pub use crate::operation::create_signaling_channel::_create_signaling_channel_input::CreateSignalingChannelInputBuilder;

/// Fluent builder constructing a request to `CreateSignalingChannel`.
///
/// <p>Creates a signaling channel. </p>
/// <p> <code>CreateSignalingChannel</code> is an asynchronous operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateSignalingChannelFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_signaling_channel::builders::CreateSignalingChannelInputBuilder,
}
impl CreateSignalingChannelFluentBuilder {
    /// Creates a new `CreateSignalingChannel`.
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
            crate::operation::create_signaling_channel::CreateSignalingChannel,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_signaling_channel::CreateSignalingChannelError,
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
        crate::operation::create_signaling_channel::CreateSignalingChannelOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_signaling_channel::CreateSignalingChannelError,
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
        crate::operation::create_signaling_channel::CreateSignalingChannelOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_signaling_channel::CreateSignalingChannelError,
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
            crate::operation::create_signaling_channel::CreateSignalingChannel,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_signaling_channel::CreateSignalingChannelError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>A name for the signaling channel that you are creating. It must be unique for each Amazon Web Services account and Amazon Web Services Region.</p>
    pub fn channel_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.channel_name(input.into());
        self
    }
    /// <p>A name for the signaling channel that you are creating. It must be unique for each Amazon Web Services account and Amazon Web Services Region.</p>
    pub fn set_channel_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_channel_name(input);
        self
    }
    /// <p>A type of the signaling channel that you are creating. Currently, <code>SINGLE_MASTER</code> is the only supported channel type. </p>
    pub fn channel_type(mut self, input: crate::types::ChannelType) -> Self {
        self.inner = self.inner.channel_type(input);
        self
    }
    /// <p>A type of the signaling channel that you are creating. Currently, <code>SINGLE_MASTER</code> is the only supported channel type. </p>
    pub fn set_channel_type(
        mut self,
        input: ::std::option::Option<crate::types::ChannelType>,
    ) -> Self {
        self.inner = self.inner.set_channel_type(input);
        self
    }
    /// <p>A structure containing the configuration for the <code>SINGLE_MASTER</code> channel type. </p>
    pub fn single_master_configuration(
        mut self,
        input: crate::types::SingleMasterConfiguration,
    ) -> Self {
        self.inner = self.inner.single_master_configuration(input);
        self
    }
    /// <p>A structure containing the configuration for the <code>SINGLE_MASTER</code> channel type. </p>
    pub fn set_single_master_configuration(
        mut self,
        input: ::std::option::Option<crate::types::SingleMasterConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_single_master_configuration(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A set of tags (key-value pairs) that you want to associate with this channel.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A set of tags (key-value pairs) that you want to associate with this channel.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
