// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A metadata key-value pair to store with an object.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MetadataEntry {
    /// <p>Name of the Object.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>Value of the Object.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
}
impl MetadataEntry {
    /// <p>Name of the Object.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Value of the Object.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl MetadataEntry {
    /// Creates a new builder-style object to manufacture [`MetadataEntry`](crate::types::MetadataEntry).
    pub fn builder() -> crate::types::builders::MetadataEntryBuilder {
        crate::types::builders::MetadataEntryBuilder::default()
    }
}

/// A builder for [`MetadataEntry`](crate::types::MetadataEntry).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MetadataEntryBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl MetadataEntryBuilder {
    /// <p>Name of the Object.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of the Object.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Value of the Object.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Value of the Object.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`MetadataEntry`](crate::types::MetadataEntry).
    pub fn build(self) -> crate::types::MetadataEntry {
        crate::types::MetadataEntry {
            name: self.name,
            value: self.value,
        }
    }
}
