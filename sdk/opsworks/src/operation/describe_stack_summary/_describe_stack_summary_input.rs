// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeStackSummaryInput {
    /// <p>The stack ID.</p>
    #[doc(hidden)]
    pub stack_id: ::std::option::Option<::std::string::String>,
}
impl DescribeStackSummaryInput {
    /// <p>The stack ID.</p>
    pub fn stack_id(&self) -> ::std::option::Option<&str> {
        self.stack_id.as_deref()
    }
}
impl DescribeStackSummaryInput {
    /// Creates a new builder-style object to manufacture [`DescribeStackSummaryInput`](crate::operation::describe_stack_summary::DescribeStackSummaryInput).
    pub fn builder(
    ) -> crate::operation::describe_stack_summary::builders::DescribeStackSummaryInputBuilder {
        crate::operation::describe_stack_summary::builders::DescribeStackSummaryInputBuilder::default()
    }
}

/// A builder for [`DescribeStackSummaryInput`](crate::operation::describe_stack_summary::DescribeStackSummaryInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeStackSummaryInputBuilder {
    pub(crate) stack_id: ::std::option::Option<::std::string::String>,
}
impl DescribeStackSummaryInputBuilder {
    /// <p>The stack ID.</p>
    pub fn stack_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stack_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The stack ID.</p>
    pub fn set_stack_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stack_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeStackSummaryInput`](crate::operation::describe_stack_summary::DescribeStackSummaryInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_stack_summary::DescribeStackSummaryInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_stack_summary::DescribeStackSummaryInput {
                stack_id: self.stack_id,
            },
        )
    }
}
