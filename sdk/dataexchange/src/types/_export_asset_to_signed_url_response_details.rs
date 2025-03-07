// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details of the export to signed URL response.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExportAssetToSignedUrlResponseDetails {
    /// <p>The unique identifier for the asset associated with this export job.</p>
    #[doc(hidden)]
    pub asset_id: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier for the data set associated with this export job.</p>
    #[doc(hidden)]
    pub data_set_id: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier for the revision associated with this export response.</p>
    #[doc(hidden)]
    pub revision_id: ::std::option::Option<::std::string::String>,
    /// <p>The signed URL for the export request.</p>
    #[doc(hidden)]
    pub signed_url: ::std::option::Option<::std::string::String>,
    /// <p>The date and time that the signed URL expires, in ISO 8601 format.</p>
    #[doc(hidden)]
    pub signed_url_expires_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ExportAssetToSignedUrlResponseDetails {
    /// <p>The unique identifier for the asset associated with this export job.</p>
    pub fn asset_id(&self) -> ::std::option::Option<&str> {
        self.asset_id.as_deref()
    }
    /// <p>The unique identifier for the data set associated with this export job.</p>
    pub fn data_set_id(&self) -> ::std::option::Option<&str> {
        self.data_set_id.as_deref()
    }
    /// <p>The unique identifier for the revision associated with this export response.</p>
    pub fn revision_id(&self) -> ::std::option::Option<&str> {
        self.revision_id.as_deref()
    }
    /// <p>The signed URL for the export request.</p>
    pub fn signed_url(&self) -> ::std::option::Option<&str> {
        self.signed_url.as_deref()
    }
    /// <p>The date and time that the signed URL expires, in ISO 8601 format.</p>
    pub fn signed_url_expires_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.signed_url_expires_at.as_ref()
    }
}
impl ExportAssetToSignedUrlResponseDetails {
    /// Creates a new builder-style object to manufacture [`ExportAssetToSignedUrlResponseDetails`](crate::types::ExportAssetToSignedUrlResponseDetails).
    pub fn builder() -> crate::types::builders::ExportAssetToSignedUrlResponseDetailsBuilder {
        crate::types::builders::ExportAssetToSignedUrlResponseDetailsBuilder::default()
    }
}

/// A builder for [`ExportAssetToSignedUrlResponseDetails`](crate::types::ExportAssetToSignedUrlResponseDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExportAssetToSignedUrlResponseDetailsBuilder {
    pub(crate) asset_id: ::std::option::Option<::std::string::String>,
    pub(crate) data_set_id: ::std::option::Option<::std::string::String>,
    pub(crate) revision_id: ::std::option::Option<::std::string::String>,
    pub(crate) signed_url: ::std::option::Option<::std::string::String>,
    pub(crate) signed_url_expires_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ExportAssetToSignedUrlResponseDetailsBuilder {
    /// <p>The unique identifier for the asset associated with this export job.</p>
    pub fn asset_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.asset_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the asset associated with this export job.</p>
    pub fn set_asset_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.asset_id = input;
        self
    }
    /// <p>The unique identifier for the data set associated with this export job.</p>
    pub fn data_set_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.data_set_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the data set associated with this export job.</p>
    pub fn set_data_set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.data_set_id = input;
        self
    }
    /// <p>The unique identifier for the revision associated with this export response.</p>
    pub fn revision_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.revision_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the revision associated with this export response.</p>
    pub fn set_revision_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.revision_id = input;
        self
    }
    /// <p>The signed URL for the export request.</p>
    pub fn signed_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.signed_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The signed URL for the export request.</p>
    pub fn set_signed_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.signed_url = input;
        self
    }
    /// <p>The date and time that the signed URL expires, in ISO 8601 format.</p>
    pub fn signed_url_expires_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.signed_url_expires_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time that the signed URL expires, in ISO 8601 format.</p>
    pub fn set_signed_url_expires_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.signed_url_expires_at = input;
        self
    }
    /// Consumes the builder and constructs a [`ExportAssetToSignedUrlResponseDetails`](crate::types::ExportAssetToSignedUrlResponseDetails).
    pub fn build(self) -> crate::types::ExportAssetToSignedUrlResponseDetails {
        crate::types::ExportAssetToSignedUrlResponseDetails {
            asset_id: self.asset_id,
            data_set_id: self.data_set_id,
            revision_id: self.revision_id,
            signed_url: self.signed_url,
            signed_url_expires_at: self.signed_url_expires_at,
        }
    }
}
