// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeRuntimeVersionsOutput {
    /// <p>An array of objects that display the details about each Synthetics canary runtime version.</p>
    #[doc(hidden)]
    pub runtime_versions: ::std::option::Option<::std::vec::Vec<crate::types::RuntimeVersion>>,
    /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeRuntimeVersions</code> operation to retrieve the next set of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeRuntimeVersionsOutput {
    /// <p>An array of objects that display the details about each Synthetics canary runtime version.</p>
    pub fn runtime_versions(&self) -> ::std::option::Option<&[crate::types::RuntimeVersion]> {
        self.runtime_versions.as_deref()
    }
    /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeRuntimeVersions</code> operation to retrieve the next set of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeRuntimeVersionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeRuntimeVersionsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeRuntimeVersionsOutput`](crate::operation::describe_runtime_versions::DescribeRuntimeVersionsOutput).
    pub fn builder(
    ) -> crate::operation::describe_runtime_versions::builders::DescribeRuntimeVersionsOutputBuilder
    {
        crate::operation::describe_runtime_versions::builders::DescribeRuntimeVersionsOutputBuilder::default()
    }
}

/// A builder for [`DescribeRuntimeVersionsOutput`](crate::operation::describe_runtime_versions::DescribeRuntimeVersionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeRuntimeVersionsOutputBuilder {
    pub(crate) runtime_versions:
        ::std::option::Option<::std::vec::Vec<crate::types::RuntimeVersion>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeRuntimeVersionsOutputBuilder {
    /// Appends an item to `runtime_versions`.
    ///
    /// To override the contents of this collection use [`set_runtime_versions`](Self::set_runtime_versions).
    ///
    /// <p>An array of objects that display the details about each Synthetics canary runtime version.</p>
    pub fn runtime_versions(mut self, input: crate::types::RuntimeVersion) -> Self {
        let mut v = self.runtime_versions.unwrap_or_default();
        v.push(input);
        self.runtime_versions = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of objects that display the details about each Synthetics canary runtime version.</p>
    pub fn set_runtime_versions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RuntimeVersion>>,
    ) -> Self {
        self.runtime_versions = input;
        self
    }
    /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeRuntimeVersions</code> operation to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeRuntimeVersions</code> operation to retrieve the next set of results.</p>
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
    /// Consumes the builder and constructs a [`DescribeRuntimeVersionsOutput`](crate::operation::describe_runtime_versions::DescribeRuntimeVersionsOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_runtime_versions::DescribeRuntimeVersionsOutput {
        crate::operation::describe_runtime_versions::DescribeRuntimeVersionsOutput {
            runtime_versions: self.runtime_versions,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
