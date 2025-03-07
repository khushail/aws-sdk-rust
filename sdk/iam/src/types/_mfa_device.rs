// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about an MFA device.</p>
/// <p>This data type is used as a response element in the <code>ListMFADevices</code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MfaDevice {
    /// <p>The user with whom the MFA device is associated.</p>
    #[doc(hidden)]
    pub user_name: ::std::option::Option<::std::string::String>,
    /// <p>The serial number that uniquely identifies the MFA device. For virtual MFA devices, the serial number is the device ARN.</p>
    #[doc(hidden)]
    pub serial_number: ::std::option::Option<::std::string::String>,
    /// <p>The date when the MFA device was enabled for the user.</p>
    #[doc(hidden)]
    pub enable_date: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl MfaDevice {
    /// <p>The user with whom the MFA device is associated.</p>
    pub fn user_name(&self) -> ::std::option::Option<&str> {
        self.user_name.as_deref()
    }
    /// <p>The serial number that uniquely identifies the MFA device. For virtual MFA devices, the serial number is the device ARN.</p>
    pub fn serial_number(&self) -> ::std::option::Option<&str> {
        self.serial_number.as_deref()
    }
    /// <p>The date when the MFA device was enabled for the user.</p>
    pub fn enable_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.enable_date.as_ref()
    }
}
impl MfaDevice {
    /// Creates a new builder-style object to manufacture [`MfaDevice`](crate::types::MfaDevice).
    pub fn builder() -> crate::types::builders::MfaDeviceBuilder {
        crate::types::builders::MfaDeviceBuilder::default()
    }
}

/// A builder for [`MfaDevice`](crate::types::MfaDevice).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MfaDeviceBuilder {
    pub(crate) user_name: ::std::option::Option<::std::string::String>,
    pub(crate) serial_number: ::std::option::Option<::std::string::String>,
    pub(crate) enable_date: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl MfaDeviceBuilder {
    /// <p>The user with whom the MFA device is associated.</p>
    pub fn user_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user with whom the MFA device is associated.</p>
    pub fn set_user_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_name = input;
        self
    }
    /// <p>The serial number that uniquely identifies the MFA device. For virtual MFA devices, the serial number is the device ARN.</p>
    pub fn serial_number(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.serial_number = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The serial number that uniquely identifies the MFA device. For virtual MFA devices, the serial number is the device ARN.</p>
    pub fn set_serial_number(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.serial_number = input;
        self
    }
    /// <p>The date when the MFA device was enabled for the user.</p>
    pub fn enable_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.enable_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date when the MFA device was enabled for the user.</p>
    pub fn set_enable_date(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.enable_date = input;
        self
    }
    /// Consumes the builder and constructs a [`MfaDevice`](crate::types::MfaDevice).
    pub fn build(self) -> crate::types::MfaDevice {
        crate::types::MfaDevice {
            user_name: self.user_name,
            serial_number: self.serial_number,
            enable_date: self.enable_date,
        }
    }
}
