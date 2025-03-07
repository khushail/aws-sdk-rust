// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the tracker resource details.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ListDevicePositionsResponseEntry {
    /// <p>The ID of the device for this position.</p>
    #[doc(hidden)]
    pub device_id: ::std::option::Option<::std::string::String>,
    /// <p>The timestamp at which the device position was determined. Uses <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    #[doc(hidden)]
    pub sample_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The last known device position. Empty if no positions currently stored.</p>
    #[doc(hidden)]
    pub position: ::std::option::Option<::std::vec::Vec<f64>>,
    /// <p>The accuracy of the device position.</p>
    #[doc(hidden)]
    pub accuracy: ::std::option::Option<crate::types::PositionalAccuracy>,
    /// <p>The properties associated with the position.</p>
    #[doc(hidden)]
    pub position_properties: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl ListDevicePositionsResponseEntry {
    /// <p>The ID of the device for this position.</p>
    pub fn device_id(&self) -> ::std::option::Option<&str> {
        self.device_id.as_deref()
    }
    /// <p>The timestamp at which the device position was determined. Uses <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    pub fn sample_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.sample_time.as_ref()
    }
    /// <p>The last known device position. Empty if no positions currently stored.</p>
    pub fn position(&self) -> ::std::option::Option<&[f64]> {
        self.position.as_deref()
    }
    /// <p>The accuracy of the device position.</p>
    pub fn accuracy(&self) -> ::std::option::Option<&crate::types::PositionalAccuracy> {
        self.accuracy.as_ref()
    }
    /// <p>The properties associated with the position.</p>
    pub fn position_properties(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.position_properties.as_ref()
    }
}
impl ::std::fmt::Debug for ListDevicePositionsResponseEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ListDevicePositionsResponseEntry");
        formatter.field("device_id", &self.device_id);
        formatter.field("sample_time", &self.sample_time);
        formatter.field("position", &"*** Sensitive Data Redacted ***");
        formatter.field("accuracy", &self.accuracy);
        formatter.field("position_properties", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl ListDevicePositionsResponseEntry {
    /// Creates a new builder-style object to manufacture [`ListDevicePositionsResponseEntry`](crate::types::ListDevicePositionsResponseEntry).
    pub fn builder() -> crate::types::builders::ListDevicePositionsResponseEntryBuilder {
        crate::types::builders::ListDevicePositionsResponseEntryBuilder::default()
    }
}

/// A builder for [`ListDevicePositionsResponseEntry`](crate::types::ListDevicePositionsResponseEntry).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct ListDevicePositionsResponseEntryBuilder {
    pub(crate) device_id: ::std::option::Option<::std::string::String>,
    pub(crate) sample_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) position: ::std::option::Option<::std::vec::Vec<f64>>,
    pub(crate) accuracy: ::std::option::Option<crate::types::PositionalAccuracy>,
    pub(crate) position_properties: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl ListDevicePositionsResponseEntryBuilder {
    /// <p>The ID of the device for this position.</p>
    pub fn device_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.device_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the device for this position.</p>
    pub fn set_device_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.device_id = input;
        self
    }
    /// <p>The timestamp at which the device position was determined. Uses <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    pub fn sample_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.sample_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp at which the device position was determined. Uses <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    pub fn set_sample_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.sample_time = input;
        self
    }
    /// Appends an item to `position`.
    ///
    /// To override the contents of this collection use [`set_position`](Self::set_position).
    ///
    /// <p>The last known device position. Empty if no positions currently stored.</p>
    pub fn position(mut self, input: f64) -> Self {
        let mut v = self.position.unwrap_or_default();
        v.push(input);
        self.position = ::std::option::Option::Some(v);
        self
    }
    /// <p>The last known device position. Empty if no positions currently stored.</p>
    pub fn set_position(mut self, input: ::std::option::Option<::std::vec::Vec<f64>>) -> Self {
        self.position = input;
        self
    }
    /// <p>The accuracy of the device position.</p>
    pub fn accuracy(mut self, input: crate::types::PositionalAccuracy) -> Self {
        self.accuracy = ::std::option::Option::Some(input);
        self
    }
    /// <p>The accuracy of the device position.</p>
    pub fn set_accuracy(
        mut self,
        input: ::std::option::Option<crate::types::PositionalAccuracy>,
    ) -> Self {
        self.accuracy = input;
        self
    }
    /// Adds a key-value pair to `position_properties`.
    ///
    /// To override the contents of this collection use [`set_position_properties`](Self::set_position_properties).
    ///
    /// <p>The properties associated with the position.</p>
    pub fn position_properties(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.position_properties.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.position_properties = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The properties associated with the position.</p>
    pub fn set_position_properties(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.position_properties = input;
        self
    }
    /// Consumes the builder and constructs a [`ListDevicePositionsResponseEntry`](crate::types::ListDevicePositionsResponseEntry).
    pub fn build(self) -> crate::types::ListDevicePositionsResponseEntry {
        crate::types::ListDevicePositionsResponseEntry {
            device_id: self.device_id,
            sample_time: self.sample_time,
            position: self.position,
            accuracy: self.accuracy,
            position_properties: self.position_properties,
        }
    }
}
impl ::std::fmt::Debug for ListDevicePositionsResponseEntryBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ListDevicePositionsResponseEntryBuilder");
        formatter.field("device_id", &self.device_id);
        formatter.field("sample_time", &self.sample_time);
        formatter.field("position", &"*** Sensitive Data Redacted ***");
        formatter.field("accuracy", &self.accuracy);
        formatter.field("position_properties", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
