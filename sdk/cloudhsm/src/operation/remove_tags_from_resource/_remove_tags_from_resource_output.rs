// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RemoveTagsFromResourceOutput {
    /// <p>The status of the operation.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl RemoveTagsFromResourceOutput {
    /// <p>The status of the operation.</p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for RemoveTagsFromResourceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RemoveTagsFromResourceOutput {
    /// Creates a new builder-style object to manufacture [`RemoveTagsFromResourceOutput`](crate::operation::remove_tags_from_resource::RemoveTagsFromResourceOutput).
    pub fn builder(
    ) -> crate::operation::remove_tags_from_resource::builders::RemoveTagsFromResourceOutputBuilder
    {
        crate::operation::remove_tags_from_resource::builders::RemoveTagsFromResourceOutputBuilder::default()
    }
}

/// A builder for [`RemoveTagsFromResourceOutput`](crate::operation::remove_tags_from_resource::RemoveTagsFromResourceOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RemoveTagsFromResourceOutputBuilder {
    pub(crate) status: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl RemoveTagsFromResourceOutputBuilder {
    /// <p>The status of the operation.</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The status of the operation.</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
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
    /// Consumes the builder and constructs a [`RemoveTagsFromResourceOutput`](crate::operation::remove_tags_from_resource::RemoveTagsFromResourceOutput).
    pub fn build(
        self,
    ) -> crate::operation::remove_tags_from_resource::RemoveTagsFromResourceOutput {
        crate::operation::remove_tags_from_resource::RemoveTagsFromResourceOutput {
            status: self.status,
            _request_id: self._request_id,
        }
    }
}
