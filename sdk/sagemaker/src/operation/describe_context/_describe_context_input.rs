// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeContextInput {
    /// <p>The name of the context to describe.</p>
    #[doc(hidden)]
    pub context_name: ::std::option::Option<::std::string::String>,
}
impl DescribeContextInput {
    /// <p>The name of the context to describe.</p>
    pub fn context_name(&self) -> ::std::option::Option<&str> {
        self.context_name.as_deref()
    }
}
impl DescribeContextInput {
    /// Creates a new builder-style object to manufacture [`DescribeContextInput`](crate::operation::describe_context::DescribeContextInput).
    pub fn builder() -> crate::operation::describe_context::builders::DescribeContextInputBuilder {
        crate::operation::describe_context::builders::DescribeContextInputBuilder::default()
    }
}

/// A builder for [`DescribeContextInput`](crate::operation::describe_context::DescribeContextInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeContextInputBuilder {
    pub(crate) context_name: ::std::option::Option<::std::string::String>,
}
impl DescribeContextInputBuilder {
    /// <p>The name of the context to describe.</p>
    pub fn context_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.context_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the context to describe.</p>
    pub fn set_context_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.context_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeContextInput`](crate::operation::describe_context::DescribeContextInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_context::DescribeContextInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_context::DescribeContextInput {
            context_name: self.context_name,
        })
    }
}
