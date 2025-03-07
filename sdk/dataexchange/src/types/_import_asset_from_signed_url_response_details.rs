// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details in the response for an import request, including the signed URL and other information.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImportAssetFromSignedUrlResponseDetails {
    /// <p>The name for the asset associated with this import job.</p>
    #[doc(hidden)]
    pub asset_name: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier for the data set associated with this import job.</p>
    #[doc(hidden)]
    pub data_set_id: ::std::option::Option<::std::string::String>,
    /// <p>The Base64-encoded Md5 hash for the asset, used to ensure the integrity of the file at that location.</p>
    #[doc(hidden)]
    pub md5_hash: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier for the revision associated with this import response.</p>
    #[doc(hidden)]
    pub revision_id: ::std::option::Option<::std::string::String>,
    /// <p>The signed URL.</p>
    #[doc(hidden)]
    pub signed_url: ::std::option::Option<::std::string::String>,
    /// <p>The time and date at which the signed URL expires, in ISO 8601 format.</p>
    #[doc(hidden)]
    pub signed_url_expires_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ImportAssetFromSignedUrlResponseDetails {
    /// <p>The name for the asset associated with this import job.</p>
    pub fn asset_name(&self) -> ::std::option::Option<&str> {
        self.asset_name.as_deref()
    }
    /// <p>The unique identifier for the data set associated with this import job.</p>
    pub fn data_set_id(&self) -> ::std::option::Option<&str> {
        self.data_set_id.as_deref()
    }
    /// <p>The Base64-encoded Md5 hash for the asset, used to ensure the integrity of the file at that location.</p>
    pub fn md5_hash(&self) -> ::std::option::Option<&str> {
        self.md5_hash.as_deref()
    }
    /// <p>The unique identifier for the revision associated with this import response.</p>
    pub fn revision_id(&self) -> ::std::option::Option<&str> {
        self.revision_id.as_deref()
    }
    /// <p>The signed URL.</p>
    pub fn signed_url(&self) -> ::std::option::Option<&str> {
        self.signed_url.as_deref()
    }
    /// <p>The time and date at which the signed URL expires, in ISO 8601 format.</p>
    pub fn signed_url_expires_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.signed_url_expires_at.as_ref()
    }
}
impl ImportAssetFromSignedUrlResponseDetails {
    /// Creates a new builder-style object to manufacture [`ImportAssetFromSignedUrlResponseDetails`](crate::types::ImportAssetFromSignedUrlResponseDetails).
    pub fn builder() -> crate::types::builders::ImportAssetFromSignedUrlResponseDetailsBuilder {
        crate::types::builders::ImportAssetFromSignedUrlResponseDetailsBuilder::default()
    }
}

/// A builder for [`ImportAssetFromSignedUrlResponseDetails`](crate::types::ImportAssetFromSignedUrlResponseDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ImportAssetFromSignedUrlResponseDetailsBuilder {
    pub(crate) asset_name: ::std::option::Option<::std::string::String>,
    pub(crate) data_set_id: ::std::option::Option<::std::string::String>,
    pub(crate) md5_hash: ::std::option::Option<::std::string::String>,
    pub(crate) revision_id: ::std::option::Option<::std::string::String>,
    pub(crate) signed_url: ::std::option::Option<::std::string::String>,
    pub(crate) signed_url_expires_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ImportAssetFromSignedUrlResponseDetailsBuilder {
    /// <p>The name for the asset associated with this import job.</p>
    pub fn asset_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.asset_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name for the asset associated with this import job.</p>
    pub fn set_asset_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.asset_name = input;
        self
    }
    /// <p>The unique identifier for the data set associated with this import job.</p>
    pub fn data_set_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.data_set_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the data set associated with this import job.</p>
    pub fn set_data_set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.data_set_id = input;
        self
    }
    /// <p>The Base64-encoded Md5 hash for the asset, used to ensure the integrity of the file at that location.</p>
    pub fn md5_hash(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.md5_hash = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Base64-encoded Md5 hash for the asset, used to ensure the integrity of the file at that location.</p>
    pub fn set_md5_hash(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.md5_hash = input;
        self
    }
    /// <p>The unique identifier for the revision associated with this import response.</p>
    pub fn revision_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.revision_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the revision associated with this import response.</p>
    pub fn set_revision_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.revision_id = input;
        self
    }
    /// <p>The signed URL.</p>
    pub fn signed_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.signed_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The signed URL.</p>
    pub fn set_signed_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.signed_url = input;
        self
    }
    /// <p>The time and date at which the signed URL expires, in ISO 8601 format.</p>
    pub fn signed_url_expires_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.signed_url_expires_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time and date at which the signed URL expires, in ISO 8601 format.</p>
    pub fn set_signed_url_expires_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.signed_url_expires_at = input;
        self
    }
    /// Consumes the builder and constructs a [`ImportAssetFromSignedUrlResponseDetails`](crate::types::ImportAssetFromSignedUrlResponseDetails).
    pub fn build(self) -> crate::types::ImportAssetFromSignedUrlResponseDetails {
        crate::types::ImportAssetFromSignedUrlResponseDetails {
            asset_name: self.asset_name,
            data_set_id: self.data_set_id,
            md5_hash: self.md5_hash,
            revision_id: self.revision_id,
            signed_url: self.signed_url,
            signed_url_expires_at: self.signed_url_expires_at,
        }
    }
}
