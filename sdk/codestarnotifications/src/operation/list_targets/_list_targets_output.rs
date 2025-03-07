// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTargetsOutput {
    /// <p>The list of notification rule targets. </p>
    #[doc(hidden)]
    pub targets: ::std::option::Option<::std::vec::Vec<crate::types::TargetSummary>>,
    /// <p>An enumeration token that can be used in a request to return the next batch of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTargetsOutput {
    /// <p>The list of notification rule targets. </p>
    pub fn targets(&self) -> ::std::option::Option<&[crate::types::TargetSummary]> {
        self.targets.as_deref()
    }
    /// <p>An enumeration token that can be used in a request to return the next batch of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListTargetsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListTargetsOutput {
    /// Creates a new builder-style object to manufacture [`ListTargetsOutput`](crate::operation::list_targets::ListTargetsOutput).
    pub fn builder() -> crate::operation::list_targets::builders::ListTargetsOutputBuilder {
        crate::operation::list_targets::builders::ListTargetsOutputBuilder::default()
    }
}

/// A builder for [`ListTargetsOutput`](crate::operation::list_targets::ListTargetsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListTargetsOutputBuilder {
    pub(crate) targets: ::std::option::Option<::std::vec::Vec<crate::types::TargetSummary>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTargetsOutputBuilder {
    /// Appends an item to `targets`.
    ///
    /// To override the contents of this collection use [`set_targets`](Self::set_targets).
    ///
    /// <p>The list of notification rule targets. </p>
    pub fn targets(mut self, input: crate::types::TargetSummary) -> Self {
        let mut v = self.targets.unwrap_or_default();
        v.push(input);
        self.targets = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of notification rule targets. </p>
    pub fn set_targets(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TargetSummary>>,
    ) -> Self {
        self.targets = input;
        self
    }
    /// <p>An enumeration token that can be used in a request to return the next batch of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An enumeration token that can be used in a request to return the next batch of results.</p>
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
    /// Consumes the builder and constructs a [`ListTargetsOutput`](crate::operation::list_targets::ListTargetsOutput).
    pub fn build(self) -> crate::operation::list_targets::ListTargetsOutput {
        crate::operation::list_targets::ListTargetsOutput {
            targets: self.targets,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
