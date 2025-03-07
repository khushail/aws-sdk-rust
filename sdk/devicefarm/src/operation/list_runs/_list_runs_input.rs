// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a request to the list runs operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListRunsInput {
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to list runs.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListRunsInput {
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to list runs.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListRunsInput {
    /// Creates a new builder-style object to manufacture [`ListRunsInput`](crate::operation::list_runs::ListRunsInput).
    pub fn builder() -> crate::operation::list_runs::builders::ListRunsInputBuilder {
        crate::operation::list_runs::builders::ListRunsInputBuilder::default()
    }
}

/// A builder for [`ListRunsInput`](crate::operation::list_runs::ListRunsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListRunsInputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListRunsInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to list runs.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to list runs.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListRunsInput`](crate::operation::list_runs::ListRunsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_runs::ListRunsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_runs::ListRunsInput {
            arn: self.arn,
            next_token: self.next_token,
        })
    }
}
