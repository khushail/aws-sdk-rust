// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The data of a room profile.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProfileData {
    /// <p>The ARN of a room profile.</p>
    #[doc(hidden)]
    pub profile_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of a room profile.</p>
    #[doc(hidden)]
    pub profile_name: ::std::option::Option<::std::string::String>,
    /// <p>Retrieves if the profile data is default or not.</p>
    #[doc(hidden)]
    pub is_default: ::std::option::Option<bool>,
    /// <p>The address of a room profile.</p>
    #[doc(hidden)]
    pub address: ::std::option::Option<::std::string::String>,
    /// <p>The time zone of a room profile.</p>
    #[doc(hidden)]
    pub timezone: ::std::option::Option<::std::string::String>,
    /// <p>The distance unit of a room profile.</p>
    #[doc(hidden)]
    pub distance_unit: ::std::option::Option<crate::types::DistanceUnit>,
    /// <p>The temperature unit of a room profile.</p>
    #[doc(hidden)]
    pub temperature_unit: ::std::option::Option<crate::types::TemperatureUnit>,
    /// <p>The wake word of a room profile.</p>
    #[doc(hidden)]
    pub wake_word: ::std::option::Option<crate::types::WakeWord>,
    /// <p>The locale of a room profile. (This is currently available only to a limited preview audience.)</p>
    #[doc(hidden)]
    pub locale: ::std::option::Option<::std::string::String>,
}
impl ProfileData {
    /// <p>The ARN of a room profile.</p>
    pub fn profile_arn(&self) -> ::std::option::Option<&str> {
        self.profile_arn.as_deref()
    }
    /// <p>The name of a room profile.</p>
    pub fn profile_name(&self) -> ::std::option::Option<&str> {
        self.profile_name.as_deref()
    }
    /// <p>Retrieves if the profile data is default or not.</p>
    pub fn is_default(&self) -> ::std::option::Option<bool> {
        self.is_default
    }
    /// <p>The address of a room profile.</p>
    pub fn address(&self) -> ::std::option::Option<&str> {
        self.address.as_deref()
    }
    /// <p>The time zone of a room profile.</p>
    pub fn timezone(&self) -> ::std::option::Option<&str> {
        self.timezone.as_deref()
    }
    /// <p>The distance unit of a room profile.</p>
    pub fn distance_unit(&self) -> ::std::option::Option<&crate::types::DistanceUnit> {
        self.distance_unit.as_ref()
    }
    /// <p>The temperature unit of a room profile.</p>
    pub fn temperature_unit(&self) -> ::std::option::Option<&crate::types::TemperatureUnit> {
        self.temperature_unit.as_ref()
    }
    /// <p>The wake word of a room profile.</p>
    pub fn wake_word(&self) -> ::std::option::Option<&crate::types::WakeWord> {
        self.wake_word.as_ref()
    }
    /// <p>The locale of a room profile. (This is currently available only to a limited preview audience.)</p>
    pub fn locale(&self) -> ::std::option::Option<&str> {
        self.locale.as_deref()
    }
}
impl ProfileData {
    /// Creates a new builder-style object to manufacture [`ProfileData`](crate::types::ProfileData).
    pub fn builder() -> crate::types::builders::ProfileDataBuilder {
        crate::types::builders::ProfileDataBuilder::default()
    }
}

/// A builder for [`ProfileData`](crate::types::ProfileData).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ProfileDataBuilder {
    pub(crate) profile_arn: ::std::option::Option<::std::string::String>,
    pub(crate) profile_name: ::std::option::Option<::std::string::String>,
    pub(crate) is_default: ::std::option::Option<bool>,
    pub(crate) address: ::std::option::Option<::std::string::String>,
    pub(crate) timezone: ::std::option::Option<::std::string::String>,
    pub(crate) distance_unit: ::std::option::Option<crate::types::DistanceUnit>,
    pub(crate) temperature_unit: ::std::option::Option<crate::types::TemperatureUnit>,
    pub(crate) wake_word: ::std::option::Option<crate::types::WakeWord>,
    pub(crate) locale: ::std::option::Option<::std::string::String>,
}
impl ProfileDataBuilder {
    /// <p>The ARN of a room profile.</p>
    pub fn profile_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.profile_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of a room profile.</p>
    pub fn set_profile_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.profile_arn = input;
        self
    }
    /// <p>The name of a room profile.</p>
    pub fn profile_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.profile_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of a room profile.</p>
    pub fn set_profile_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.profile_name = input;
        self
    }
    /// <p>Retrieves if the profile data is default or not.</p>
    pub fn is_default(mut self, input: bool) -> Self {
        self.is_default = ::std::option::Option::Some(input);
        self
    }
    /// <p>Retrieves if the profile data is default or not.</p>
    pub fn set_is_default(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_default = input;
        self
    }
    /// <p>The address of a room profile.</p>
    pub fn address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The address of a room profile.</p>
    pub fn set_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.address = input;
        self
    }
    /// <p>The time zone of a room profile.</p>
    pub fn timezone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.timezone = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The time zone of a room profile.</p>
    pub fn set_timezone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.timezone = input;
        self
    }
    /// <p>The distance unit of a room profile.</p>
    pub fn distance_unit(mut self, input: crate::types::DistanceUnit) -> Self {
        self.distance_unit = ::std::option::Option::Some(input);
        self
    }
    /// <p>The distance unit of a room profile.</p>
    pub fn set_distance_unit(
        mut self,
        input: ::std::option::Option<crate::types::DistanceUnit>,
    ) -> Self {
        self.distance_unit = input;
        self
    }
    /// <p>The temperature unit of a room profile.</p>
    pub fn temperature_unit(mut self, input: crate::types::TemperatureUnit) -> Self {
        self.temperature_unit = ::std::option::Option::Some(input);
        self
    }
    /// <p>The temperature unit of a room profile.</p>
    pub fn set_temperature_unit(
        mut self,
        input: ::std::option::Option<crate::types::TemperatureUnit>,
    ) -> Self {
        self.temperature_unit = input;
        self
    }
    /// <p>The wake word of a room profile.</p>
    pub fn wake_word(mut self, input: crate::types::WakeWord) -> Self {
        self.wake_word = ::std::option::Option::Some(input);
        self
    }
    /// <p>The wake word of a room profile.</p>
    pub fn set_wake_word(mut self, input: ::std::option::Option<crate::types::WakeWord>) -> Self {
        self.wake_word = input;
        self
    }
    /// <p>The locale of a room profile. (This is currently available only to a limited preview audience.)</p>
    pub fn locale(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.locale = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The locale of a room profile. (This is currently available only to a limited preview audience.)</p>
    pub fn set_locale(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.locale = input;
        self
    }
    /// Consumes the builder and constructs a [`ProfileData`](crate::types::ProfileData).
    pub fn build(self) -> crate::types::ProfileData {
        crate::types::ProfileData {
            profile_arn: self.profile_arn,
            profile_name: self.profile_name,
            is_default: self.is_default,
            address: self.address,
            timezone: self.timezone,
            distance_unit: self.distance_unit,
            temperature_unit: self.temperature_unit,
            wake_word: self.wake_word,
            locale: self.locale,
        }
    }
}
