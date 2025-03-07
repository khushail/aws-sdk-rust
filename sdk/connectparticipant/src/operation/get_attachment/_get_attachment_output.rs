// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAttachmentOutput {
    /// <p>This is the pre-signed URL that can be used for uploading the file to Amazon S3 when used in response to <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_StartAttachmentUpload.html">StartAttachmentUpload</a>.</p>
    #[doc(hidden)]
    pub url: ::std::option::Option<::std::string::String>,
    /// <p>The expiration time of the URL in ISO timestamp. It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[doc(hidden)]
    pub url_expiry: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetAttachmentOutput {
    /// <p>This is the pre-signed URL that can be used for uploading the file to Amazon S3 when used in response to <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_StartAttachmentUpload.html">StartAttachmentUpload</a>.</p>
    pub fn url(&self) -> ::std::option::Option<&str> {
        self.url.as_deref()
    }
    /// <p>The expiration time of the URL in ISO timestamp. It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    pub fn url_expiry(&self) -> ::std::option::Option<&str> {
        self.url_expiry.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetAttachmentOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetAttachmentOutput {
    /// Creates a new builder-style object to manufacture [`GetAttachmentOutput`](crate::operation::get_attachment::GetAttachmentOutput).
    pub fn builder() -> crate::operation::get_attachment::builders::GetAttachmentOutputBuilder {
        crate::operation::get_attachment::builders::GetAttachmentOutputBuilder::default()
    }
}

/// A builder for [`GetAttachmentOutput`](crate::operation::get_attachment::GetAttachmentOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetAttachmentOutputBuilder {
    pub(crate) url: ::std::option::Option<::std::string::String>,
    pub(crate) url_expiry: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetAttachmentOutputBuilder {
    /// <p>This is the pre-signed URL that can be used for uploading the file to Amazon S3 when used in response to <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_StartAttachmentUpload.html">StartAttachmentUpload</a>.</p>
    pub fn url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>This is the pre-signed URL that can be used for uploading the file to Amazon S3 when used in response to <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_StartAttachmentUpload.html">StartAttachmentUpload</a>.</p>
    pub fn set_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.url = input;
        self
    }
    /// <p>The expiration time of the URL in ISO timestamp. It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    pub fn url_expiry(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.url_expiry = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The expiration time of the URL in ISO timestamp. It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    pub fn set_url_expiry(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.url_expiry = input;
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
    /// Consumes the builder and constructs a [`GetAttachmentOutput`](crate::operation::get_attachment::GetAttachmentOutput).
    pub fn build(self) -> crate::operation::get_attachment::GetAttachmentOutput {
        crate::operation::get_attachment::GetAttachmentOutput {
            url: self.url,
            url_expiry: self.url_expiry,
            _request_id: self._request_id,
        }
    }
}
