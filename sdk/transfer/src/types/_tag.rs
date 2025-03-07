// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Creates a key-value pair for a specific resource. Tags are metadata that you can use to search for and group a resource for various purposes. You can apply tags to servers, users, and roles. A tag key can take more than one value. For example, to group servers for accounting purposes, you might create a tag called <code>Group</code> and assign the values <code>Research</code> and <code>Accounting</code> to that group.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Tag {
    /// <p>The name assigned to the tag that you create.</p>
    #[doc(hidden)]
    pub key: ::std::option::Option<::std::string::String>,
    /// <p>Contains one or more values that you assigned to the key name you create.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
}
impl Tag {
    /// <p>The name assigned to the tag that you create.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>Contains one or more values that you assigned to the key name you create.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`Tag`](crate::types::Tag).
    pub fn builder() -> crate::types::builders::TagBuilder {
        crate::types::builders::TagBuilder::default()
    }
}

/// A builder for [`Tag`](crate::types::Tag).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TagBuilder {
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl TagBuilder {
    /// <p>The name assigned to the tag that you create.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name assigned to the tag that you create.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>Contains one or more values that you assigned to the key name you create.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Contains one or more values that you assigned to the key name you create.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`Tag`](crate::types::Tag).
    pub fn build(self) -> crate::types::Tag {
        crate::types::Tag {
            key: self.key,
            value: self.value,
        }
    }
}
