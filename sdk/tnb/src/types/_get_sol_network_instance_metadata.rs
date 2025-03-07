// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The metadata of a network instance.</p>
/// <p>A network instance is a single network created in Amazon Web Services TNB that can be deployed and on which life-cycle operations (like terminate, update, and delete) can be performed.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetSolNetworkInstanceMetadata {
    /// <p>The date that the resource was created.</p>
    #[doc(hidden)]
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date that the resource was last modified.</p>
    #[doc(hidden)]
    pub last_modified: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl GetSolNetworkInstanceMetadata {
    /// <p>The date that the resource was created.</p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>The date that the resource was last modified.</p>
    pub fn last_modified(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modified.as_ref()
    }
}
impl GetSolNetworkInstanceMetadata {
    /// Creates a new builder-style object to manufacture [`GetSolNetworkInstanceMetadata`](crate::types::GetSolNetworkInstanceMetadata).
    pub fn builder() -> crate::types::builders::GetSolNetworkInstanceMetadataBuilder {
        crate::types::builders::GetSolNetworkInstanceMetadataBuilder::default()
    }
}

/// A builder for [`GetSolNetworkInstanceMetadata`](crate::types::GetSolNetworkInstanceMetadata).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetSolNetworkInstanceMetadataBuilder {
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_modified: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl GetSolNetworkInstanceMetadataBuilder {
    /// <p>The date that the resource was created.</p>
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date that the resource was created.</p>
    pub fn set_created_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_at = input;
        self
    }
    /// <p>The date that the resource was last modified.</p>
    pub fn last_modified(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date that the resource was last modified.</p>
    pub fn set_last_modified(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_modified = input;
        self
    }
    /// Consumes the builder and constructs a [`GetSolNetworkInstanceMetadata`](crate::types::GetSolNetworkInstanceMetadata).
    pub fn build(self) -> crate::types::GetSolNetworkInstanceMetadata {
        crate::types::GetSolNetworkInstanceMetadata {
            created_at: self.created_at,
            last_modified: self.last_modified,
        }
    }
}
