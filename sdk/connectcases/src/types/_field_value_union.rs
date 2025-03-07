// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Object to store union of Field values.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub enum FieldValueUnion {
    /// <p>Can be either null, or have a Boolean value type. Only one value can be provided.</p>
    BooleanValue(bool),
    /// <p>Can be either null, or have a Double number value type. Only one value can be provided.</p>
    DoubleValue(f64),
    /// <p>String value type.</p>
    StringValue(::std::string::String),
    /// The `Unknown` variant represents cases where new union variant was received. Consider upgrading the SDK to the latest available version.
    /// An unknown enum variant
    ///
    /// _Note: If you encounter this error, consider upgrading your SDK to the latest version._
    /// The `Unknown` variant represents cases where the server sent a value that wasn't recognized
    /// by the client. This can happen when the server adds new functionality, but the client has not been updated.
    /// To investigate this, consider turning on debug logging to print the raw HTTP response.
    #[non_exhaustive]
    Unknown,
}
impl FieldValueUnion {
    /// Tries to convert the enum instance into [`BooleanValue`](crate::types::FieldValueUnion::BooleanValue), extracting the inner [`bool`](bool).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_boolean_value(&self) -> ::std::result::Result<&bool, &Self> {
        if let FieldValueUnion::BooleanValue(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`BooleanValue`](crate::types::FieldValueUnion::BooleanValue).
    pub fn is_boolean_value(&self) -> bool {
        self.as_boolean_value().is_ok()
    }
    /// Tries to convert the enum instance into [`DoubleValue`](crate::types::FieldValueUnion::DoubleValue), extracting the inner [`f64`](f64).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_double_value(&self) -> ::std::result::Result<&f64, &Self> {
        if let FieldValueUnion::DoubleValue(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`DoubleValue`](crate::types::FieldValueUnion::DoubleValue).
    pub fn is_double_value(&self) -> bool {
        self.as_double_value().is_ok()
    }
    /// Tries to convert the enum instance into [`StringValue`](crate::types::FieldValueUnion::StringValue), extracting the inner [`String`](::std::string::String).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_string_value(&self) -> ::std::result::Result<&::std::string::String, &Self> {
        if let FieldValueUnion::StringValue(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`StringValue`](crate::types::FieldValueUnion::StringValue).
    pub fn is_string_value(&self) -> bool {
        self.as_string_value().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}
