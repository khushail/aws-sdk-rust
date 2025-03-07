// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListUnsupportedAppVersionResourcesOutput {
    /// <p>The unsupported resources for the application.</p>
    #[doc(hidden)]
    pub unsupported_resources:
        ::std::option::Option<::std::vec::Vec<crate::types::UnsupportedResource>>,
    /// <p>The identifier for a specific resolution.</p>
    #[doc(hidden)]
    pub resolution_id: ::std::option::Option<::std::string::String>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListUnsupportedAppVersionResourcesOutput {
    /// <p>The unsupported resources for the application.</p>
    pub fn unsupported_resources(
        &self,
    ) -> ::std::option::Option<&[crate::types::UnsupportedResource]> {
        self.unsupported_resources.as_deref()
    }
    /// <p>The identifier for a specific resolution.</p>
    pub fn resolution_id(&self) -> ::std::option::Option<&str> {
        self.resolution_id.as_deref()
    }
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListUnsupportedAppVersionResourcesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListUnsupportedAppVersionResourcesOutput {
    /// Creates a new builder-style object to manufacture [`ListUnsupportedAppVersionResourcesOutput`](crate::operation::list_unsupported_app_version_resources::ListUnsupportedAppVersionResourcesOutput).
    pub fn builder() -> crate::operation::list_unsupported_app_version_resources::builders::ListUnsupportedAppVersionResourcesOutputBuilder{
        crate::operation::list_unsupported_app_version_resources::builders::ListUnsupportedAppVersionResourcesOutputBuilder::default()
    }
}

/// A builder for [`ListUnsupportedAppVersionResourcesOutput`](crate::operation::list_unsupported_app_version_resources::ListUnsupportedAppVersionResourcesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListUnsupportedAppVersionResourcesOutputBuilder {
    pub(crate) unsupported_resources:
        ::std::option::Option<::std::vec::Vec<crate::types::UnsupportedResource>>,
    pub(crate) resolution_id: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListUnsupportedAppVersionResourcesOutputBuilder {
    /// Appends an item to `unsupported_resources`.
    ///
    /// To override the contents of this collection use [`set_unsupported_resources`](Self::set_unsupported_resources).
    ///
    /// <p>The unsupported resources for the application.</p>
    pub fn unsupported_resources(mut self, input: crate::types::UnsupportedResource) -> Self {
        let mut v = self.unsupported_resources.unwrap_or_default();
        v.push(input);
        self.unsupported_resources = ::std::option::Option::Some(v);
        self
    }
    /// <p>The unsupported resources for the application.</p>
    pub fn set_unsupported_resources(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::UnsupportedResource>>,
    ) -> Self {
        self.unsupported_resources = input;
        self
    }
    /// <p>The identifier for a specific resolution.</p>
    pub fn resolution_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.resolution_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for a specific resolution.</p>
    pub fn set_resolution_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.resolution_id = input;
        self
    }
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next set of results, or null if there are no more results.</p>
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
    /// Consumes the builder and constructs a [`ListUnsupportedAppVersionResourcesOutput`](crate::operation::list_unsupported_app_version_resources::ListUnsupportedAppVersionResourcesOutput).
    pub fn build(self) -> crate::operation::list_unsupported_app_version_resources::ListUnsupportedAppVersionResourcesOutput{
        crate::operation::list_unsupported_app_version_resources::ListUnsupportedAppVersionResourcesOutput {
            unsupported_resources: self.unsupported_resources
            ,
            resolution_id: self.resolution_id
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}
