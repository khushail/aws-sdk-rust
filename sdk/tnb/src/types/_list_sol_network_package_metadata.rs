// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Metadata related to a network package.</p>
/// <p>A network package is a .zip file in CSAR (Cloud Service Archive) format defines the function packages you want to deploy and the Amazon Web Services infrastructure you want to deploy them on.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListSolNetworkPackageMetadata {
    /// <p>The date that the resource was created.</p>
    #[doc(hidden)]
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date that the resource was last modified.</p>
    #[doc(hidden)]
    pub last_modified: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ListSolNetworkPackageMetadata {
    /// <p>The date that the resource was created.</p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>The date that the resource was last modified.</p>
    pub fn last_modified(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modified.as_ref()
    }
}
impl ListSolNetworkPackageMetadata {
    /// Creates a new builder-style object to manufacture [`ListSolNetworkPackageMetadata`](crate::types::ListSolNetworkPackageMetadata).
    pub fn builder() -> crate::types::builders::ListSolNetworkPackageMetadataBuilder {
        crate::types::builders::ListSolNetworkPackageMetadataBuilder::default()
    }
}

/// A builder for [`ListSolNetworkPackageMetadata`](crate::types::ListSolNetworkPackageMetadata).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListSolNetworkPackageMetadataBuilder {
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_modified: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ListSolNetworkPackageMetadataBuilder {
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
    /// Consumes the builder and constructs a [`ListSolNetworkPackageMetadata`](crate::types::ListSolNetworkPackageMetadata).
    pub fn build(self) -> crate::types::ListSolNetworkPackageMetadata {
        crate::types::ListSolNetworkPackageMetadata {
            created_at: self.created_at,
            last_modified: self.last_modified,
        }
    }
}
