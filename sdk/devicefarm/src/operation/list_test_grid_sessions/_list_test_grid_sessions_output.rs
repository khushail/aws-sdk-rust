// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTestGridSessionsOutput {
    /// <p>The sessions that match the criteria in a <code>ListTestGridSessionsRequest</code>. </p>
    #[doc(hidden)]
    pub test_grid_sessions: ::std::option::Option<::std::vec::Vec<crate::types::TestGridSession>>,
    /// <p>Pagination token.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTestGridSessionsOutput {
    /// <p>The sessions that match the criteria in a <code>ListTestGridSessionsRequest</code>. </p>
    pub fn test_grid_sessions(&self) -> ::std::option::Option<&[crate::types::TestGridSession]> {
        self.test_grid_sessions.as_deref()
    }
    /// <p>Pagination token.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListTestGridSessionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListTestGridSessionsOutput {
    /// Creates a new builder-style object to manufacture [`ListTestGridSessionsOutput`](crate::operation::list_test_grid_sessions::ListTestGridSessionsOutput).
    pub fn builder(
    ) -> crate::operation::list_test_grid_sessions::builders::ListTestGridSessionsOutputBuilder
    {
        crate::operation::list_test_grid_sessions::builders::ListTestGridSessionsOutputBuilder::default()
    }
}

/// A builder for [`ListTestGridSessionsOutput`](crate::operation::list_test_grid_sessions::ListTestGridSessionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListTestGridSessionsOutputBuilder {
    pub(crate) test_grid_sessions:
        ::std::option::Option<::std::vec::Vec<crate::types::TestGridSession>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTestGridSessionsOutputBuilder {
    /// Appends an item to `test_grid_sessions`.
    ///
    /// To override the contents of this collection use [`set_test_grid_sessions`](Self::set_test_grid_sessions).
    ///
    /// <p>The sessions that match the criteria in a <code>ListTestGridSessionsRequest</code>. </p>
    pub fn test_grid_sessions(mut self, input: crate::types::TestGridSession) -> Self {
        let mut v = self.test_grid_sessions.unwrap_or_default();
        v.push(input);
        self.test_grid_sessions = ::std::option::Option::Some(v);
        self
    }
    /// <p>The sessions that match the criteria in a <code>ListTestGridSessionsRequest</code>. </p>
    pub fn set_test_grid_sessions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TestGridSession>>,
    ) -> Self {
        self.test_grid_sessions = input;
        self
    }
    /// <p>Pagination token.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Pagination token.</p>
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
    /// Consumes the builder and constructs a [`ListTestGridSessionsOutput`](crate::operation::list_test_grid_sessions::ListTestGridSessionsOutput).
    pub fn build(self) -> crate::operation::list_test_grid_sessions::ListTestGridSessionsOutput {
        crate::operation::list_test_grid_sessions::ListTestGridSessionsOutput {
            test_grid_sessions: self.test_grid_sessions,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
