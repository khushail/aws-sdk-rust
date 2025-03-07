// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListThemeVersionsOutput {
    /// <p>A structure containing a list of all the versions of the specified theme.</p>
    #[doc(hidden)]
    pub theme_version_summary_list:
        ::std::option::Option<::std::vec::Vec<crate::types::ThemeVersionSummary>>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The HTTP status of the request.</p>
    #[doc(hidden)]
    pub status: i32,
    /// <p>The Amazon Web Services request ID for this operation.</p>
    #[doc(hidden)]
    pub request_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListThemeVersionsOutput {
    /// <p>A structure containing a list of all the versions of the specified theme.</p>
    pub fn theme_version_summary_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::ThemeVersionSummary]> {
        self.theme_version_summary_list.as_deref()
    }
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The HTTP status of the request.</p>
    pub fn status(&self) -> i32 {
        self.status
    }
    /// <p>The Amazon Web Services request ID for this operation.</p>
    pub fn request_id(&self) -> ::std::option::Option<&str> {
        self.request_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListThemeVersionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListThemeVersionsOutput {
    /// Creates a new builder-style object to manufacture [`ListThemeVersionsOutput`](crate::operation::list_theme_versions::ListThemeVersionsOutput).
    pub fn builder(
    ) -> crate::operation::list_theme_versions::builders::ListThemeVersionsOutputBuilder {
        crate::operation::list_theme_versions::builders::ListThemeVersionsOutputBuilder::default()
    }
}

/// A builder for [`ListThemeVersionsOutput`](crate::operation::list_theme_versions::ListThemeVersionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListThemeVersionsOutputBuilder {
    pub(crate) theme_version_summary_list:
        ::std::option::Option<::std::vec::Vec<crate::types::ThemeVersionSummary>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<i32>,
    pub(crate) request_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListThemeVersionsOutputBuilder {
    /// Appends an item to `theme_version_summary_list`.
    ///
    /// To override the contents of this collection use [`set_theme_version_summary_list`](Self::set_theme_version_summary_list).
    ///
    /// <p>A structure containing a list of all the versions of the specified theme.</p>
    pub fn theme_version_summary_list(mut self, input: crate::types::ThemeVersionSummary) -> Self {
        let mut v = self.theme_version_summary_list.unwrap_or_default();
        v.push(input);
        self.theme_version_summary_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A structure containing a list of all the versions of the specified theme.</p>
    pub fn set_theme_version_summary_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ThemeVersionSummary>>,
    ) -> Self {
        self.theme_version_summary_list = input;
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
    /// <p>The HTTP status of the request.</p>
    pub fn status(mut self, input: i32) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The HTTP status of the request.</p>
    pub fn set_status(mut self, input: ::std::option::Option<i32>) -> Self {
        self.status = input;
        self
    }
    /// <p>The Amazon Web Services request ID for this operation.</p>
    pub fn request_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.request_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services request ID for this operation.</p>
    pub fn set_request_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.request_id = input;
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
    /// Consumes the builder and constructs a [`ListThemeVersionsOutput`](crate::operation::list_theme_versions::ListThemeVersionsOutput).
    pub fn build(self) -> crate::operation::list_theme_versions::ListThemeVersionsOutput {
        crate::operation::list_theme_versions::ListThemeVersionsOutput {
            theme_version_summary_list: self.theme_version_summary_list,
            next_token: self.next_token,
            status: self.status.unwrap_or_default(),
            request_id: self.request_id,
            _request_id: self._request_id,
        }
    }
}
