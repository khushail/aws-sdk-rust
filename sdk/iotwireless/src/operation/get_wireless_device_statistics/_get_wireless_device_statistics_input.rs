// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetWirelessDeviceStatisticsInput {
    /// <p>The ID of the wireless device for which to get the data.</p>
    #[doc(hidden)]
    pub wireless_device_id: ::std::option::Option<::std::string::String>,
}
impl GetWirelessDeviceStatisticsInput {
    /// <p>The ID of the wireless device for which to get the data.</p>
    pub fn wireless_device_id(&self) -> ::std::option::Option<&str> {
        self.wireless_device_id.as_deref()
    }
}
impl GetWirelessDeviceStatisticsInput {
    /// Creates a new builder-style object to manufacture [`GetWirelessDeviceStatisticsInput`](crate::operation::get_wireless_device_statistics::GetWirelessDeviceStatisticsInput).
    pub fn builder() -> crate::operation::get_wireless_device_statistics::builders::GetWirelessDeviceStatisticsInputBuilder{
        crate::operation::get_wireless_device_statistics::builders::GetWirelessDeviceStatisticsInputBuilder::default()
    }
}

/// A builder for [`GetWirelessDeviceStatisticsInput`](crate::operation::get_wireless_device_statistics::GetWirelessDeviceStatisticsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetWirelessDeviceStatisticsInputBuilder {
    pub(crate) wireless_device_id: ::std::option::Option<::std::string::String>,
}
impl GetWirelessDeviceStatisticsInputBuilder {
    /// <p>The ID of the wireless device for which to get the data.</p>
    pub fn wireless_device_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.wireless_device_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the wireless device for which to get the data.</p>
    pub fn set_wireless_device_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.wireless_device_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetWirelessDeviceStatisticsInput`](crate::operation::get_wireless_device_statistics::GetWirelessDeviceStatisticsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_wireless_device_statistics::GetWirelessDeviceStatisticsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_wireless_device_statistics::GetWirelessDeviceStatisticsInput {
                wireless_device_id: self.wireless_device_id,
            },
        )
    }
}
