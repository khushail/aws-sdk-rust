// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details about a beta appliance software update.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AlternateSoftwareMetadata {
    /// <p>The appliance software version.</p>
    #[doc(hidden)]
    pub version: ::std::option::Option<::std::string::String>,
}
impl AlternateSoftwareMetadata {
    /// <p>The appliance software version.</p>
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
}
impl AlternateSoftwareMetadata {
    /// Creates a new builder-style object to manufacture [`AlternateSoftwareMetadata`](crate::types::AlternateSoftwareMetadata).
    pub fn builder() -> crate::types::builders::AlternateSoftwareMetadataBuilder {
        crate::types::builders::AlternateSoftwareMetadataBuilder::default()
    }
}

/// A builder for [`AlternateSoftwareMetadata`](crate::types::AlternateSoftwareMetadata).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AlternateSoftwareMetadataBuilder {
    pub(crate) version: ::std::option::Option<::std::string::String>,
}
impl AlternateSoftwareMetadataBuilder {
    /// <p>The appliance software version.</p>
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The appliance software version.</p>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    /// Consumes the builder and constructs a [`AlternateSoftwareMetadata`](crate::types::AlternateSoftwareMetadata).
    pub fn build(self) -> crate::types::AlternateSoftwareMetadata {
        crate::types::AlternateSoftwareMetadata {
            version: self.version,
        }
    }
}
