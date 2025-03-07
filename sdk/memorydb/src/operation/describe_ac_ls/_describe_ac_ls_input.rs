// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeAcLsInput {
    /// <p>The name of the ACL</p>
    #[doc(hidden)]
    pub acl_name: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified MaxResults value, a token is included in the response so that the remaining results can be retrieved.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeAcLsInput {
    /// <p>The name of the ACL</p>
    pub fn acl_name(&self) -> ::std::option::Option<&str> {
        self.acl_name.as_deref()
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified MaxResults value, a token is included in the response so that the remaining results can be retrieved.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl DescribeAcLsInput {
    /// Creates a new builder-style object to manufacture [`DescribeAcLsInput`](crate::operation::describe_ac_ls::DescribeAcLsInput).
    pub fn builder() -> crate::operation::describe_ac_ls::builders::DescribeAcLsInputBuilder {
        crate::operation::describe_ac_ls::builders::DescribeAcLsInputBuilder::default()
    }
}

/// A builder for [`DescribeAcLsInput`](crate::operation::describe_ac_ls::DescribeAcLsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeAcLsInputBuilder {
    pub(crate) acl_name: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeAcLsInputBuilder {
    /// <p>The name of the ACL</p>
    pub fn acl_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.acl_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the ACL</p>
    pub fn set_acl_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.acl_name = input;
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified MaxResults value, a token is included in the response so that the remaining results can be retrieved.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified MaxResults value, a token is included in the response so that the remaining results can be retrieved.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeAcLsInput`](crate::operation::describe_ac_ls::DescribeAcLsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_ac_ls::DescribeAcLsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_ac_ls::DescribeAcLsInput {
            acl_name: self.acl_name,
            max_results: self.max_results,
            next_token: self.next_token,
        })
    }
}
