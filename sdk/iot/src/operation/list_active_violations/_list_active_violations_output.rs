// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListActiveViolationsOutput {
    /// <p>The list of active violations.</p>
    #[doc(hidden)]
    pub active_violations: ::std::option::Option<::std::vec::Vec<crate::types::ActiveViolation>>,
    /// <p>A token that can be used to retrieve the next set of results, or <code>null</code> if there are no additional results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListActiveViolationsOutput {
    /// <p>The list of active violations.</p>
    pub fn active_violations(&self) -> ::std::option::Option<&[crate::types::ActiveViolation]> {
        self.active_violations.as_deref()
    }
    /// <p>A token that can be used to retrieve the next set of results, or <code>null</code> if there are no additional results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListActiveViolationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListActiveViolationsOutput {
    /// Creates a new builder-style object to manufacture [`ListActiveViolationsOutput`](crate::operation::list_active_violations::ListActiveViolationsOutput).
    pub fn builder(
    ) -> crate::operation::list_active_violations::builders::ListActiveViolationsOutputBuilder {
        crate::operation::list_active_violations::builders::ListActiveViolationsOutputBuilder::default()
    }
}

/// A builder for [`ListActiveViolationsOutput`](crate::operation::list_active_violations::ListActiveViolationsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListActiveViolationsOutputBuilder {
    pub(crate) active_violations:
        ::std::option::Option<::std::vec::Vec<crate::types::ActiveViolation>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListActiveViolationsOutputBuilder {
    /// Appends an item to `active_violations`.
    ///
    /// To override the contents of this collection use [`set_active_violations`](Self::set_active_violations).
    ///
    /// <p>The list of active violations.</p>
    pub fn active_violations(mut self, input: crate::types::ActiveViolation) -> Self {
        let mut v = self.active_violations.unwrap_or_default();
        v.push(input);
        self.active_violations = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of active violations.</p>
    pub fn set_active_violations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ActiveViolation>>,
    ) -> Self {
        self.active_violations = input;
        self
    }
    /// <p>A token that can be used to retrieve the next set of results, or <code>null</code> if there are no additional results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token that can be used to retrieve the next set of results, or <code>null</code> if there are no additional results.</p>
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
    /// Consumes the builder and constructs a [`ListActiveViolationsOutput`](crate::operation::list_active_violations::ListActiveViolationsOutput).
    pub fn build(self) -> crate::operation::list_active_violations::ListActiveViolationsOutput {
        crate::operation::list_active_violations::ListActiveViolationsOutput {
            active_violations: self.active_violations,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
