// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartAttachmentUploadOutput {
    /// <p>A unique identifier for the attachment.</p>
    #[doc(hidden)]
    pub attachment_id: ::std::option::Option<::std::string::String>,
    /// <p>Fields to be used while uploading the attachment.</p>
    #[doc(hidden)]
    pub upload_metadata: ::std::option::Option<crate::types::UploadMetadata>,
    _request_id: Option<String>,
}
impl StartAttachmentUploadOutput {
    /// <p>A unique identifier for the attachment.</p>
    pub fn attachment_id(&self) -> ::std::option::Option<&str> {
        self.attachment_id.as_deref()
    }
    /// <p>Fields to be used while uploading the attachment.</p>
    pub fn upload_metadata(&self) -> ::std::option::Option<&crate::types::UploadMetadata> {
        self.upload_metadata.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for StartAttachmentUploadOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StartAttachmentUploadOutput {
    /// Creates a new builder-style object to manufacture [`StartAttachmentUploadOutput`](crate::operation::start_attachment_upload::StartAttachmentUploadOutput).
    pub fn builder(
    ) -> crate::operation::start_attachment_upload::builders::StartAttachmentUploadOutputBuilder
    {
        crate::operation::start_attachment_upload::builders::StartAttachmentUploadOutputBuilder::default()
    }
}

/// A builder for [`StartAttachmentUploadOutput`](crate::operation::start_attachment_upload::StartAttachmentUploadOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StartAttachmentUploadOutputBuilder {
    pub(crate) attachment_id: ::std::option::Option<::std::string::String>,
    pub(crate) upload_metadata: ::std::option::Option<crate::types::UploadMetadata>,
    _request_id: Option<String>,
}
impl StartAttachmentUploadOutputBuilder {
    /// <p>A unique identifier for the attachment.</p>
    pub fn attachment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.attachment_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the attachment.</p>
    pub fn set_attachment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.attachment_id = input;
        self
    }
    /// <p>Fields to be used while uploading the attachment.</p>
    pub fn upload_metadata(mut self, input: crate::types::UploadMetadata) -> Self {
        self.upload_metadata = ::std::option::Option::Some(input);
        self
    }
    /// <p>Fields to be used while uploading the attachment.</p>
    pub fn set_upload_metadata(
        mut self,
        input: ::std::option::Option<crate::types::UploadMetadata>,
    ) -> Self {
        self.upload_metadata = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`StartAttachmentUploadOutput`](crate::operation::start_attachment_upload::StartAttachmentUploadOutput).
    pub fn build(self) -> crate::operation::start_attachment_upload::StartAttachmentUploadOutput {
        crate::operation::start_attachment_upload::StartAttachmentUploadOutput {
            attachment_id: self.attachment_id,
            upload_metadata: self.upload_metadata,
            _request_id: self._request_id,
        }
    }
}
