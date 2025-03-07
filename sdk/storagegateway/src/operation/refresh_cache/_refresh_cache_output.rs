// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>RefreshCacheOutput</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RefreshCacheOutput {
    /// <p>The Amazon Resource Name (ARN) of the file share.</p>
    #[doc(hidden)]
    pub file_share_arn: ::std::option::Option<::std::string::String>,
    /// <p>The randomly generated ID of the notification that was sent. This ID is in UUID format.</p>
    #[doc(hidden)]
    pub notification_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl RefreshCacheOutput {
    /// <p>The Amazon Resource Name (ARN) of the file share.</p>
    pub fn file_share_arn(&self) -> ::std::option::Option<&str> {
        self.file_share_arn.as_deref()
    }
    /// <p>The randomly generated ID of the notification that was sent. This ID is in UUID format.</p>
    pub fn notification_id(&self) -> ::std::option::Option<&str> {
        self.notification_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for RefreshCacheOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RefreshCacheOutput {
    /// Creates a new builder-style object to manufacture [`RefreshCacheOutput`](crate::operation::refresh_cache::RefreshCacheOutput).
    pub fn builder() -> crate::operation::refresh_cache::builders::RefreshCacheOutputBuilder {
        crate::operation::refresh_cache::builders::RefreshCacheOutputBuilder::default()
    }
}

/// A builder for [`RefreshCacheOutput`](crate::operation::refresh_cache::RefreshCacheOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RefreshCacheOutputBuilder {
    pub(crate) file_share_arn: ::std::option::Option<::std::string::String>,
    pub(crate) notification_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl RefreshCacheOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the file share.</p>
    pub fn file_share_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.file_share_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the file share.</p>
    pub fn set_file_share_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.file_share_arn = input;
        self
    }
    /// <p>The randomly generated ID of the notification that was sent. This ID is in UUID format.</p>
    pub fn notification_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.notification_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The randomly generated ID of the notification that was sent. This ID is in UUID format.</p>
    pub fn set_notification_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.notification_id = input;
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
    /// Consumes the builder and constructs a [`RefreshCacheOutput`](crate::operation::refresh_cache::RefreshCacheOutput).
    pub fn build(self) -> crate::operation::refresh_cache::RefreshCacheOutput {
        crate::operation::refresh_cache::RefreshCacheOutput {
            file_share_arn: self.file_share_arn,
            notification_id: self.notification_id,
            _request_id: self._request_id,
        }
    }
}
