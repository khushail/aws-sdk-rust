// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a pre-existing file or directory on the host machine that the volume maps to.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct HostPath {
    /// <p>Path of the file or directory on the host that the volume maps to.</p>
    #[doc(hidden)]
    pub path: ::std::option::Option<::std::string::String>,
}
impl HostPath {
    /// <p>Path of the file or directory on the host that the volume maps to.</p>
    pub fn path(&self) -> ::std::option::Option<&str> {
        self.path.as_deref()
    }
}
impl HostPath {
    /// Creates a new builder-style object to manufacture [`HostPath`](crate::types::HostPath).
    pub fn builder() -> crate::types::builders::HostPathBuilder {
        crate::types::builders::HostPathBuilder::default()
    }
}

/// A builder for [`HostPath`](crate::types::HostPath).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct HostPathBuilder {
    pub(crate) path: ::std::option::Option<::std::string::String>,
}
impl HostPathBuilder {
    /// <p>Path of the file or directory on the host that the volume maps to.</p>
    pub fn path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Path of the file or directory on the host that the volume maps to.</p>
    pub fn set_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.path = input;
        self
    }
    /// Consumes the builder and constructs a [`HostPath`](crate::types::HostPath).
    pub fn build(self) -> crate::types::HostPath {
        crate::types::HostPath { path: self.path }
    }
}
