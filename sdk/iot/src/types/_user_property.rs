// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A key-value pair that you define in the header. Both the key and the value are either literal strings or valid <a href="https://docs.aws.amazon.com/iot/latest/developerguide/iot-substitution-templates.html">substitution templates</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UserProperty {
    /// <p>A key to be specified in <code>UserProperty</code>.</p>
    #[doc(hidden)]
    pub key: ::std::option::Option<::std::string::String>,
    /// <p>A value to be specified in <code>UserProperty</code>.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
}
impl UserProperty {
    /// <p>A key to be specified in <code>UserProperty</code>.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>A value to be specified in <code>UserProperty</code>.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl UserProperty {
    /// Creates a new builder-style object to manufacture [`UserProperty`](crate::types::UserProperty).
    pub fn builder() -> crate::types::builders::UserPropertyBuilder {
        crate::types::builders::UserPropertyBuilder::default()
    }
}

/// A builder for [`UserProperty`](crate::types::UserProperty).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UserPropertyBuilder {
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl UserPropertyBuilder {
    /// <p>A key to be specified in <code>UserProperty</code>.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A key to be specified in <code>UserProperty</code>.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>A value to be specified in <code>UserProperty</code>.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A value to be specified in <code>UserProperty</code>.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`UserProperty`](crate::types::UserProperty).
    pub fn build(self) -> crate::types::UserProperty {
        crate::types::UserProperty {
            key: self.key,
            value: self.value,
        }
    }
}
