// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ConfigureLogsForChannelOutput {
    /// <p>The name of the channel.</p>
    #[doc(hidden)]
    pub channel_name: ::std::option::Option<::std::string::String>,
    /// <p>The types of logs collected.</p>
    #[doc(hidden)]
    pub log_types: ::std::option::Option<::std::vec::Vec<crate::types::LogType>>,
    _request_id: Option<String>,
}
impl ConfigureLogsForChannelOutput {
    /// <p>The name of the channel.</p>
    pub fn channel_name(&self) -> ::std::option::Option<&str> {
        self.channel_name.as_deref()
    }
    /// <p>The types of logs collected.</p>
    pub fn log_types(&self) -> ::std::option::Option<&[crate::types::LogType]> {
        self.log_types.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ConfigureLogsForChannelOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ConfigureLogsForChannelOutput {
    /// Creates a new builder-style object to manufacture [`ConfigureLogsForChannelOutput`](crate::operation::configure_logs_for_channel::ConfigureLogsForChannelOutput).
    pub fn builder(
    ) -> crate::operation::configure_logs_for_channel::builders::ConfigureLogsForChannelOutputBuilder
    {
        crate::operation::configure_logs_for_channel::builders::ConfigureLogsForChannelOutputBuilder::default()
    }
}

/// A builder for [`ConfigureLogsForChannelOutput`](crate::operation::configure_logs_for_channel::ConfigureLogsForChannelOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ConfigureLogsForChannelOutputBuilder {
    pub(crate) channel_name: ::std::option::Option<::std::string::String>,
    pub(crate) log_types: ::std::option::Option<::std::vec::Vec<crate::types::LogType>>,
    _request_id: Option<String>,
}
impl ConfigureLogsForChannelOutputBuilder {
    /// <p>The name of the channel.</p>
    pub fn channel_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the channel.</p>
    pub fn set_channel_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_name = input;
        self
    }
    /// Appends an item to `log_types`.
    ///
    /// To override the contents of this collection use [`set_log_types`](Self::set_log_types).
    ///
    /// <p>The types of logs collected.</p>
    pub fn log_types(mut self, input: crate::types::LogType) -> Self {
        let mut v = self.log_types.unwrap_or_default();
        v.push(input);
        self.log_types = ::std::option::Option::Some(v);
        self
    }
    /// <p>The types of logs collected.</p>
    pub fn set_log_types(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::LogType>>,
    ) -> Self {
        self.log_types = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ConfigureLogsForChannelOutput`](crate::operation::configure_logs_for_channel::ConfigureLogsForChannelOutput).
    pub fn build(
        self,
    ) -> crate::operation::configure_logs_for_channel::ConfigureLogsForChannelOutput {
        crate::operation::configure_logs_for_channel::ConfigureLogsForChannelOutput {
            channel_name: self.channel_name,
            log_types: self.log_types,
            _request_id: self._request_id,
        }
    }
}
