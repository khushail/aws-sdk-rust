// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateLogLevelsByResourceTypesInput {
    /// <p>The log level for a log message. The log levels can be disabled, or set to <code>ERROR</code> to display less verbose logs containing only error information, or to <code>INFO</code> for more detailed logs.</p>
    #[doc(hidden)]
    pub default_log_level: ::std::option::Option<crate::types::LogLevel>,
    /// <p>The list of wireless device log options.</p>
    #[doc(hidden)]
    pub wireless_device_log_options:
        ::std::option::Option<::std::vec::Vec<crate::types::WirelessDeviceLogOption>>,
    /// <p>The list of wireless gateway log options.</p>
    #[doc(hidden)]
    pub wireless_gateway_log_options:
        ::std::option::Option<::std::vec::Vec<crate::types::WirelessGatewayLogOption>>,
}
impl UpdateLogLevelsByResourceTypesInput {
    /// <p>The log level for a log message. The log levels can be disabled, or set to <code>ERROR</code> to display less verbose logs containing only error information, or to <code>INFO</code> for more detailed logs.</p>
    pub fn default_log_level(&self) -> ::std::option::Option<&crate::types::LogLevel> {
        self.default_log_level.as_ref()
    }
    /// <p>The list of wireless device log options.</p>
    pub fn wireless_device_log_options(
        &self,
    ) -> ::std::option::Option<&[crate::types::WirelessDeviceLogOption]> {
        self.wireless_device_log_options.as_deref()
    }
    /// <p>The list of wireless gateway log options.</p>
    pub fn wireless_gateway_log_options(
        &self,
    ) -> ::std::option::Option<&[crate::types::WirelessGatewayLogOption]> {
        self.wireless_gateway_log_options.as_deref()
    }
}
impl UpdateLogLevelsByResourceTypesInput {
    /// Creates a new builder-style object to manufacture [`UpdateLogLevelsByResourceTypesInput`](crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesInput).
    pub fn builder() -> crate::operation::update_log_levels_by_resource_types::builders::UpdateLogLevelsByResourceTypesInputBuilder{
        crate::operation::update_log_levels_by_resource_types::builders::UpdateLogLevelsByResourceTypesInputBuilder::default()
    }
}

/// A builder for [`UpdateLogLevelsByResourceTypesInput`](crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateLogLevelsByResourceTypesInputBuilder {
    pub(crate) default_log_level: ::std::option::Option<crate::types::LogLevel>,
    pub(crate) wireless_device_log_options:
        ::std::option::Option<::std::vec::Vec<crate::types::WirelessDeviceLogOption>>,
    pub(crate) wireless_gateway_log_options:
        ::std::option::Option<::std::vec::Vec<crate::types::WirelessGatewayLogOption>>,
}
impl UpdateLogLevelsByResourceTypesInputBuilder {
    /// <p>The log level for a log message. The log levels can be disabled, or set to <code>ERROR</code> to display less verbose logs containing only error information, or to <code>INFO</code> for more detailed logs.</p>
    pub fn default_log_level(mut self, input: crate::types::LogLevel) -> Self {
        self.default_log_level = ::std::option::Option::Some(input);
        self
    }
    /// <p>The log level for a log message. The log levels can be disabled, or set to <code>ERROR</code> to display less verbose logs containing only error information, or to <code>INFO</code> for more detailed logs.</p>
    pub fn set_default_log_level(
        mut self,
        input: ::std::option::Option<crate::types::LogLevel>,
    ) -> Self {
        self.default_log_level = input;
        self
    }
    /// Appends an item to `wireless_device_log_options`.
    ///
    /// To override the contents of this collection use [`set_wireless_device_log_options`](Self::set_wireless_device_log_options).
    ///
    /// <p>The list of wireless device log options.</p>
    pub fn wireless_device_log_options(
        mut self,
        input: crate::types::WirelessDeviceLogOption,
    ) -> Self {
        let mut v = self.wireless_device_log_options.unwrap_or_default();
        v.push(input);
        self.wireless_device_log_options = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of wireless device log options.</p>
    pub fn set_wireless_device_log_options(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::WirelessDeviceLogOption>>,
    ) -> Self {
        self.wireless_device_log_options = input;
        self
    }
    /// Appends an item to `wireless_gateway_log_options`.
    ///
    /// To override the contents of this collection use [`set_wireless_gateway_log_options`](Self::set_wireless_gateway_log_options).
    ///
    /// <p>The list of wireless gateway log options.</p>
    pub fn wireless_gateway_log_options(
        mut self,
        input: crate::types::WirelessGatewayLogOption,
    ) -> Self {
        let mut v = self.wireless_gateway_log_options.unwrap_or_default();
        v.push(input);
        self.wireless_gateway_log_options = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of wireless gateway log options.</p>
    pub fn set_wireless_gateway_log_options(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::WirelessGatewayLogOption>>,
    ) -> Self {
        self.wireless_gateway_log_options = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateLogLevelsByResourceTypesInput`](crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesInput {
                default_log_level: self.default_log_level
                ,
                wireless_device_log_options: self.wireless_device_log_options
                ,
                wireless_gateway_log_options: self.wireless_gateway_log_options
                ,
            }
        )
    }
}
