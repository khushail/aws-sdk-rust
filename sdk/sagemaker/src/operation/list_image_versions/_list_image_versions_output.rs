// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListImageVersionsOutput {
    /// <p>A list of versions and their properties.</p>
    #[doc(hidden)]
    pub image_versions: ::std::option::Option<::std::vec::Vec<crate::types::ImageVersion>>,
    /// <p>A token for getting the next set of versions, if there are any.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListImageVersionsOutput {
    /// <p>A list of versions and their properties.</p>
    pub fn image_versions(&self) -> ::std::option::Option<&[crate::types::ImageVersion]> {
        self.image_versions.as_deref()
    }
    /// <p>A token for getting the next set of versions, if there are any.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListImageVersionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListImageVersionsOutput {
    /// Creates a new builder-style object to manufacture [`ListImageVersionsOutput`](crate::operation::list_image_versions::ListImageVersionsOutput).
    pub fn builder(
    ) -> crate::operation::list_image_versions::builders::ListImageVersionsOutputBuilder {
        crate::operation::list_image_versions::builders::ListImageVersionsOutputBuilder::default()
    }
}

/// A builder for [`ListImageVersionsOutput`](crate::operation::list_image_versions::ListImageVersionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListImageVersionsOutputBuilder {
    pub(crate) image_versions: ::std::option::Option<::std::vec::Vec<crate::types::ImageVersion>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListImageVersionsOutputBuilder {
    /// Appends an item to `image_versions`.
    ///
    /// To override the contents of this collection use [`set_image_versions`](Self::set_image_versions).
    ///
    /// <p>A list of versions and their properties.</p>
    pub fn image_versions(mut self, input: crate::types::ImageVersion) -> Self {
        let mut v = self.image_versions.unwrap_or_default();
        v.push(input);
        self.image_versions = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of versions and their properties.</p>
    pub fn set_image_versions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ImageVersion>>,
    ) -> Self {
        self.image_versions = input;
        self
    }
    /// <p>A token for getting the next set of versions, if there are any.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token for getting the next set of versions, if there are any.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`ListImageVersionsOutput`](crate::operation::list_image_versions::ListImageVersionsOutput).
    pub fn build(self) -> crate::operation::list_image_versions::ListImageVersionsOutput {
        crate::operation::list_image_versions::ListImageVersionsOutput {
            image_versions: self.image_versions,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
