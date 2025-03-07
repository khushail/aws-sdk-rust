// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListApplicationsInput {
    /// <p>The ID of the environment. </p>
    #[doc(hidden)]
    pub environment_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The token for the next page of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
}
impl ListApplicationsInput {
    /// <p>The ID of the environment. </p>
    pub fn environment_identifier(&self) -> ::std::option::Option<&str> {
        self.environment_identifier.as_deref()
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl ListApplicationsInput {
    /// Creates a new builder-style object to manufacture [`ListApplicationsInput`](crate::operation::list_applications::ListApplicationsInput).
    pub fn builder() -> crate::operation::list_applications::builders::ListApplicationsInputBuilder
    {
        crate::operation::list_applications::builders::ListApplicationsInputBuilder::default()
    }
}

/// A builder for [`ListApplicationsInput`](crate::operation::list_applications::ListApplicationsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListApplicationsInputBuilder {
    pub(crate) environment_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl ListApplicationsInputBuilder {
    /// <p>The ID of the environment. </p>
    pub fn environment_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.environment_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the environment. </p>
    pub fn set_environment_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.environment_identifier = input;
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`ListApplicationsInput`](crate::operation::list_applications::ListApplicationsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_applications::ListApplicationsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_applications::ListApplicationsInput {
            environment_identifier: self.environment_identifier,
            next_token: self.next_token,
            max_results: self.max_results,
        })
    }
}
