// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeLogStreamsOutput {
    /// <p>The log streams.</p>
    #[doc(hidden)]
    pub log_streams: ::std::option::Option<::std::vec::Vec<crate::types::LogStream>>,
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeLogStreamsOutput {
    /// <p>The log streams.</p>
    pub fn log_streams(&self) -> ::std::option::Option<&[crate::types::LogStream]> {
        self.log_streams.as_deref()
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeLogStreamsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeLogStreamsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeLogStreamsOutput`](crate::operation::describe_log_streams::DescribeLogStreamsOutput).
    pub fn builder(
    ) -> crate::operation::describe_log_streams::builders::DescribeLogStreamsOutputBuilder {
        crate::operation::describe_log_streams::builders::DescribeLogStreamsOutputBuilder::default()
    }
}

/// A builder for [`DescribeLogStreamsOutput`](crate::operation::describe_log_streams::DescribeLogStreamsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeLogStreamsOutputBuilder {
    pub(crate) log_streams: ::std::option::Option<::std::vec::Vec<crate::types::LogStream>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeLogStreamsOutputBuilder {
    /// Appends an item to `log_streams`.
    ///
    /// To override the contents of this collection use [`set_log_streams`](Self::set_log_streams).
    ///
    /// <p>The log streams.</p>
    pub fn log_streams(mut self, input: crate::types::LogStream) -> Self {
        let mut v = self.log_streams.unwrap_or_default();
        v.push(input);
        self.log_streams = ::std::option::Option::Some(v);
        self
    }
    /// <p>The log streams.</p>
    pub fn set_log_streams(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::LogStream>>,
    ) -> Self {
        self.log_streams = input;
        self
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
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
    /// Consumes the builder and constructs a [`DescribeLogStreamsOutput`](crate::operation::describe_log_streams::DescribeLogStreamsOutput).
    pub fn build(self) -> crate::operation::describe_log_streams::DescribeLogStreamsOutput {
        crate::operation::describe_log_streams::DescribeLogStreamsOutput {
            log_streams: self.log_streams,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
