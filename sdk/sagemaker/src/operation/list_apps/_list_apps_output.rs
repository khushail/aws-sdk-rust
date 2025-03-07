// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListAppsOutput {
    /// <p>The list of apps.</p>
    #[doc(hidden)]
    pub apps: ::std::option::Option<::std::vec::Vec<crate::types::AppDetails>>,
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListAppsOutput {
    /// <p>The list of apps.</p>
    pub fn apps(&self) -> ::std::option::Option<&[crate::types::AppDetails]> {
        self.apps.as_deref()
    }
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListAppsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListAppsOutput {
    /// Creates a new builder-style object to manufacture [`ListAppsOutput`](crate::operation::list_apps::ListAppsOutput).
    pub fn builder() -> crate::operation::list_apps::builders::ListAppsOutputBuilder {
        crate::operation::list_apps::builders::ListAppsOutputBuilder::default()
    }
}

/// A builder for [`ListAppsOutput`](crate::operation::list_apps::ListAppsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListAppsOutputBuilder {
    pub(crate) apps: ::std::option::Option<::std::vec::Vec<crate::types::AppDetails>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListAppsOutputBuilder {
    /// Appends an item to `apps`.
    ///
    /// To override the contents of this collection use [`set_apps`](Self::set_apps).
    ///
    /// <p>The list of apps.</p>
    pub fn apps(mut self, input: crate::types::AppDetails) -> Self {
        let mut v = self.apps.unwrap_or_default();
        v.push(input);
        self.apps = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of apps.</p>
    pub fn set_apps(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AppDetails>>,
    ) -> Self {
        self.apps = input;
        self
    }
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
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
    /// Consumes the builder and constructs a [`ListAppsOutput`](crate::operation::list_apps::ListAppsOutput).
    pub fn build(self) -> crate::operation::list_apps::ListAppsOutput {
        crate::operation::list_apps::ListAppsOutput {
            apps: self.apps,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
