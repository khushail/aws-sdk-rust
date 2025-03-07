// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_channel_class::_update_channel_class_output::UpdateChannelClassOutputBuilder;

pub use crate::operation::update_channel_class::_update_channel_class_input::UpdateChannelClassInputBuilder;

/// Fluent builder constructing a request to `UpdateChannelClass`.
///
/// Changes the class of the channel.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateChannelClassFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_channel_class::builders::UpdateChannelClassInputBuilder,
}
impl UpdateChannelClassFluentBuilder {
    /// Creates a new `UpdateChannelClass`.
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
            crate::operation::update_channel_class::UpdateChannelClass,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_channel_class::UpdateChannelClassError,
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
        crate::operation::update_channel_class::UpdateChannelClassOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_channel_class::UpdateChannelClassError,
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
        crate::operation::update_channel_class::UpdateChannelClassOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_channel_class::UpdateChannelClassError,
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
            crate::operation::update_channel_class::UpdateChannelClass,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_channel_class::UpdateChannelClassError,
        >,
    > {
        self.customize_middleware().await
    }
    /// The channel class that you wish to update this channel to use.
    pub fn channel_class(mut self, input: crate::types::ChannelClass) -> Self {
        self.inner = self.inner.channel_class(input);
        self
    }
    /// The channel class that you wish to update this channel to use.
    pub fn set_channel_class(
        mut self,
        input: ::std::option::Option<crate::types::ChannelClass>,
    ) -> Self {
        self.inner = self.inner.set_channel_class(input);
        self
    }
    /// Channel Id of the channel whose class should be updated.
    pub fn channel_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.channel_id(input.into());
        self
    }
    /// Channel Id of the channel whose class should be updated.
    pub fn set_channel_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_channel_id(input);
        self
    }
    /// Appends an item to `Destinations`.
    ///
    /// To override the contents of this collection use [`set_destinations`](Self::set_destinations).
    ///
    /// A list of output destinations for this channel.
    pub fn destinations(mut self, input: crate::types::OutputDestination) -> Self {
        self.inner = self.inner.destinations(input);
        self
    }
    /// A list of output destinations for this channel.
    pub fn set_destinations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::OutputDestination>>,
    ) -> Self {
        self.inner = self.inner.set_destinations(input);
        self
    }
}
