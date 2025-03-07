// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes key/value pairs.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Metadata {
    /// <p>The key name.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The value.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
}
impl Metadata {
    /// <p>The key name.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The value.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl Metadata {
    /// Creates a new builder-style object to manufacture [`Metadata`](crate::types::Metadata).
    pub fn builder() -> crate::types::builders::MetadataBuilder {
        crate::types::builders::MetadataBuilder::default()
    }
}

/// A builder for [`Metadata`](crate::types::Metadata).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MetadataBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl MetadataBuilder {
    /// <p>The key name.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The key name.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The value.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`Metadata`](crate::types::Metadata).
    pub fn build(self) -> crate::types::Metadata {
        crate::types::Metadata {
            name: self.name,
            value: self.value,
        }
    }
}
