// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The log options for a wireless gateway event and can be used to set log levels for a specific wireless gateway event.</p>
/// <p>For a LoRaWAN gateway, possible events for a log message are <code>CUPS_Request</code> and <code>Certificate</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct WirelessGatewayEventLogOption {
    /// <p>The event for a log message, if the log message is tied to a wireless gateway.</p>
    #[doc(hidden)]
    pub event: ::std::option::Option<crate::types::WirelessGatewayEvent>,
    /// <p>The log level for a log message. The log levels can be disabled, or set to <code>ERROR</code> to display less verbose logs containing only error information, or to <code>INFO</code> for more detailed logs.</p>
    #[doc(hidden)]
    pub log_level: ::std::option::Option<crate::types::LogLevel>,
}
impl WirelessGatewayEventLogOption {
    /// <p>The event for a log message, if the log message is tied to a wireless gateway.</p>
    pub fn event(&self) -> ::std::option::Option<&crate::types::WirelessGatewayEvent> {
        self.event.as_ref()
    }
    /// <p>The log level for a log message. The log levels can be disabled, or set to <code>ERROR</code> to display less verbose logs containing only error information, or to <code>INFO</code> for more detailed logs.</p>
    pub fn log_level(&self) -> ::std::option::Option<&crate::types::LogLevel> {
        self.log_level.as_ref()
    }
}
impl WirelessGatewayEventLogOption {
    /// Creates a new builder-style object to manufacture [`WirelessGatewayEventLogOption`](crate::types::WirelessGatewayEventLogOption).
    pub fn builder() -> crate::types::builders::WirelessGatewayEventLogOptionBuilder {
        crate::types::builders::WirelessGatewayEventLogOptionBuilder::default()
    }
}

/// A builder for [`WirelessGatewayEventLogOption`](crate::types::WirelessGatewayEventLogOption).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct WirelessGatewayEventLogOptionBuilder {
    pub(crate) event: ::std::option::Option<crate::types::WirelessGatewayEvent>,
    pub(crate) log_level: ::std::option::Option<crate::types::LogLevel>,
}
impl WirelessGatewayEventLogOptionBuilder {
    /// <p>The event for a log message, if the log message is tied to a wireless gateway.</p>
    pub fn event(mut self, input: crate::types::WirelessGatewayEvent) -> Self {
        self.event = ::std::option::Option::Some(input);
        self
    }
    /// <p>The event for a log message, if the log message is tied to a wireless gateway.</p>
    pub fn set_event(
        mut self,
        input: ::std::option::Option<crate::types::WirelessGatewayEvent>,
    ) -> Self {
        self.event = input;
        self
    }
    /// <p>The log level for a log message. The log levels can be disabled, or set to <code>ERROR</code> to display less verbose logs containing only error information, or to <code>INFO</code> for more detailed logs.</p>
    pub fn log_level(mut self, input: crate::types::LogLevel) -> Self {
        self.log_level = ::std::option::Option::Some(input);
        self
    }
    /// <p>The log level for a log message. The log levels can be disabled, or set to <code>ERROR</code> to display less verbose logs containing only error information, or to <code>INFO</code> for more detailed logs.</p>
    pub fn set_log_level(mut self, input: ::std::option::Option<crate::types::LogLevel>) -> Self {
        self.log_level = input;
        self
    }
    /// Consumes the builder and constructs a [`WirelessGatewayEventLogOption`](crate::types::WirelessGatewayEventLogOption).
    pub fn build(self) -> crate::types::WirelessGatewayEventLogOption {
        crate::types::WirelessGatewayEventLogOption {
            event: self.event,
            log_level: self.log_level,
        }
    }
}
