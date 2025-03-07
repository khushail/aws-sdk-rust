// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetChangeLogsOutput {
    /// <p>The list of user activity for the control. </p>
    #[doc(hidden)]
    pub change_logs: ::std::option::Option<::std::vec::Vec<crate::types::ChangeLog>>,
    /// <p>The pagination token that's used to fetch the next set of results. </p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetChangeLogsOutput {
    /// <p>The list of user activity for the control. </p>
    pub fn change_logs(&self) -> ::std::option::Option<&[crate::types::ChangeLog]> {
        self.change_logs.as_deref()
    }
    /// <p>The pagination token that's used to fetch the next set of results. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetChangeLogsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetChangeLogsOutput {
    /// Creates a new builder-style object to manufacture [`GetChangeLogsOutput`](crate::operation::get_change_logs::GetChangeLogsOutput).
    pub fn builder() -> crate::operation::get_change_logs::builders::GetChangeLogsOutputBuilder {
        crate::operation::get_change_logs::builders::GetChangeLogsOutputBuilder::default()
    }
}

/// A builder for [`GetChangeLogsOutput`](crate::operation::get_change_logs::GetChangeLogsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetChangeLogsOutputBuilder {
    pub(crate) change_logs: ::std::option::Option<::std::vec::Vec<crate::types::ChangeLog>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetChangeLogsOutputBuilder {
    /// Appends an item to `change_logs`.
    ///
    /// To override the contents of this collection use [`set_change_logs`](Self::set_change_logs).
    ///
    /// <p>The list of user activity for the control. </p>
    pub fn change_logs(mut self, input: crate::types::ChangeLog) -> Self {
        let mut v = self.change_logs.unwrap_or_default();
        v.push(input);
        self.change_logs = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of user activity for the control. </p>
    pub fn set_change_logs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ChangeLog>>,
    ) -> Self {
        self.change_logs = input;
        self
    }
    /// <p>The pagination token that's used to fetch the next set of results. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pagination token that's used to fetch the next set of results. </p>
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
    /// Consumes the builder and constructs a [`GetChangeLogsOutput`](crate::operation::get_change_logs::GetChangeLogsOutput).
    pub fn build(self) -> crate::operation::get_change_logs::GetChangeLogsOutput {
        crate::operation::get_change_logs::GetChangeLogsOutput {
            change_logs: self.change_logs,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
