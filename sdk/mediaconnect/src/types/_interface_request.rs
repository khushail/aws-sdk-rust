// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// The VPC interface that you want to designate where the media stream is coming from or going to.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InterfaceRequest {
    /// The name of the VPC interface.
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl InterfaceRequest {
    /// The name of the VPC interface.
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl InterfaceRequest {
    /// Creates a new builder-style object to manufacture [`InterfaceRequest`](crate::types::InterfaceRequest).
    pub fn builder() -> crate::types::builders::InterfaceRequestBuilder {
        crate::types::builders::InterfaceRequestBuilder::default()
    }
}

/// A builder for [`InterfaceRequest`](crate::types::InterfaceRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct InterfaceRequestBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl InterfaceRequestBuilder {
    /// The name of the VPC interface.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// The name of the VPC interface.
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`InterfaceRequest`](crate::types::InterfaceRequest).
    pub fn build(self) -> crate::types::InterfaceRequest {
        crate::types::InterfaceRequest { name: self.name }
    }
}
