// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutMessagingStreamingConfigurationsInput {
    /// <p>The ARN of the streaming configuration.</p>
    #[doc(hidden)]
    pub app_instance_arn: ::std::option::Option<::std::string::String>,
    /// <p>The streaming configurations.</p>
    #[doc(hidden)]
    pub streaming_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::StreamingConfiguration>>,
}
impl PutMessagingStreamingConfigurationsInput {
    /// <p>The ARN of the streaming configuration.</p>
    pub fn app_instance_arn(&self) -> ::std::option::Option<&str> {
        self.app_instance_arn.as_deref()
    }
    /// <p>The streaming configurations.</p>
    pub fn streaming_configurations(
        &self,
    ) -> ::std::option::Option<&[crate::types::StreamingConfiguration]> {
        self.streaming_configurations.as_deref()
    }
}
impl PutMessagingStreamingConfigurationsInput {
    /// Creates a new builder-style object to manufacture [`PutMessagingStreamingConfigurationsInput`](crate::operation::put_messaging_streaming_configurations::PutMessagingStreamingConfigurationsInput).
    pub fn builder() -> crate::operation::put_messaging_streaming_configurations::builders::PutMessagingStreamingConfigurationsInputBuilder{
        crate::operation::put_messaging_streaming_configurations::builders::PutMessagingStreamingConfigurationsInputBuilder::default()
    }
}

/// A builder for [`PutMessagingStreamingConfigurationsInput`](crate::operation::put_messaging_streaming_configurations::PutMessagingStreamingConfigurationsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PutMessagingStreamingConfigurationsInputBuilder {
    pub(crate) app_instance_arn: ::std::option::Option<::std::string::String>,
    pub(crate) streaming_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::StreamingConfiguration>>,
}
impl PutMessagingStreamingConfigurationsInputBuilder {
    /// <p>The ARN of the streaming configuration.</p>
    pub fn app_instance_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.app_instance_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the streaming configuration.</p>
    pub fn set_app_instance_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.app_instance_arn = input;
        self
    }
    /// Appends an item to `streaming_configurations`.
    ///
    /// To override the contents of this collection use [`set_streaming_configurations`](Self::set_streaming_configurations).
    ///
    /// <p>The streaming configurations.</p>
    pub fn streaming_configurations(mut self, input: crate::types::StreamingConfiguration) -> Self {
        let mut v = self.streaming_configurations.unwrap_or_default();
        v.push(input);
        self.streaming_configurations = ::std::option::Option::Some(v);
        self
    }
    /// <p>The streaming configurations.</p>
    pub fn set_streaming_configurations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::StreamingConfiguration>>,
    ) -> Self {
        self.streaming_configurations = input;
        self
    }
    /// Consumes the builder and constructs a [`PutMessagingStreamingConfigurationsInput`](crate::operation::put_messaging_streaming_configurations::PutMessagingStreamingConfigurationsInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::put_messaging_streaming_configurations::PutMessagingStreamingConfigurationsInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::put_messaging_streaming_configurations::PutMessagingStreamingConfigurationsInput {
                app_instance_arn: self.app_instance_arn
                ,
                streaming_configurations: self.streaming_configurations
                ,
            }
        )
    }
}
