// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A list of Lambda functions.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListFunctionsOutput {
    /// <p>The pagination token that's included if more results are available.</p>
    #[doc(hidden)]
    pub next_marker: ::std::option::Option<::std::string::String>,
    /// <p>A list of Lambda functions.</p>
    #[doc(hidden)]
    pub functions: ::std::option::Option<::std::vec::Vec<crate::types::FunctionConfiguration>>,
    _request_id: Option<String>,
}
impl ListFunctionsOutput {
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn next_marker(&self) -> ::std::option::Option<&str> {
        self.next_marker.as_deref()
    }
    /// <p>A list of Lambda functions.</p>
    pub fn functions(&self) -> ::std::option::Option<&[crate::types::FunctionConfiguration]> {
        self.functions.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListFunctionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListFunctionsOutput {
    /// Creates a new builder-style object to manufacture [`ListFunctionsOutput`](crate::operation::list_functions::ListFunctionsOutput).
    pub fn builder() -> crate::operation::list_functions::builders::ListFunctionsOutputBuilder {
        crate::operation::list_functions::builders::ListFunctionsOutputBuilder::default()
    }
}

/// A builder for [`ListFunctionsOutput`](crate::operation::list_functions::ListFunctionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListFunctionsOutputBuilder {
    pub(crate) next_marker: ::std::option::Option<::std::string::String>,
    pub(crate) functions:
        ::std::option::Option<::std::vec::Vec<crate::types::FunctionConfiguration>>,
    _request_id: Option<String>,
}
impl ListFunctionsOutputBuilder {
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn next_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn set_next_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_marker = input;
        self
    }
    /// Appends an item to `functions`.
    ///
    /// To override the contents of this collection use [`set_functions`](Self::set_functions).
    ///
    /// <p>A list of Lambda functions.</p>
    pub fn functions(mut self, input: crate::types::FunctionConfiguration) -> Self {
        let mut v = self.functions.unwrap_or_default();
        v.push(input);
        self.functions = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of Lambda functions.</p>
    pub fn set_functions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::FunctionConfiguration>>,
    ) -> Self {
        self.functions = input;
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
    /// Consumes the builder and constructs a [`ListFunctionsOutput`](crate::operation::list_functions::ListFunctionsOutput).
    pub fn build(self) -> crate::operation::list_functions::ListFunctionsOutput {
        crate::operation::list_functions::ListFunctionsOutput {
            next_marker: self.next_marker,
            functions: self.functions,
            _request_id: self._request_id,
        }
    }
}
