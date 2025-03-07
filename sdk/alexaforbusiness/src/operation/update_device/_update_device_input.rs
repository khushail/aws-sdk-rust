// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateDeviceInput {
    /// <p>The ARN of the device to update. Required.</p>
    #[doc(hidden)]
    pub device_arn: ::std::option::Option<::std::string::String>,
    /// <p>The updated device name. Required.</p>
    #[doc(hidden)]
    pub device_name: ::std::option::Option<::std::string::String>,
}
impl UpdateDeviceInput {
    /// <p>The ARN of the device to update. Required.</p>
    pub fn device_arn(&self) -> ::std::option::Option<&str> {
        self.device_arn.as_deref()
    }
    /// <p>The updated device name. Required.</p>
    pub fn device_name(&self) -> ::std::option::Option<&str> {
        self.device_name.as_deref()
    }
}
impl UpdateDeviceInput {
    /// Creates a new builder-style object to manufacture [`UpdateDeviceInput`](crate::operation::update_device::UpdateDeviceInput).
    pub fn builder() -> crate::operation::update_device::builders::UpdateDeviceInputBuilder {
        crate::operation::update_device::builders::UpdateDeviceInputBuilder::default()
    }
}

/// A builder for [`UpdateDeviceInput`](crate::operation::update_device::UpdateDeviceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateDeviceInputBuilder {
    pub(crate) device_arn: ::std::option::Option<::std::string::String>,
    pub(crate) device_name: ::std::option::Option<::std::string::String>,
}
impl UpdateDeviceInputBuilder {
    /// <p>The ARN of the device to update. Required.</p>
    pub fn device_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.device_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the device to update. Required.</p>
    pub fn set_device_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.device_arn = input;
        self
    }
    /// <p>The updated device name. Required.</p>
    pub fn device_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.device_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The updated device name. Required.</p>
    pub fn set_device_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.device_name = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateDeviceInput`](crate::operation::update_device::UpdateDeviceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_device::UpdateDeviceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_device::UpdateDeviceInput {
            device_arn: self.device_arn,
            device_name: self.device_name,
        })
    }
}
