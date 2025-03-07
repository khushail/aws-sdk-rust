// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>DescribeTaskExecutionRequest</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeTaskExecutionInput {
    /// <p>The Amazon Resource Name (ARN) of the task that is being executed.</p>
    #[doc(hidden)]
    pub task_execution_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeTaskExecutionInput {
    /// <p>The Amazon Resource Name (ARN) of the task that is being executed.</p>
    pub fn task_execution_arn(&self) -> ::std::option::Option<&str> {
        self.task_execution_arn.as_deref()
    }
}
impl DescribeTaskExecutionInput {
    /// Creates a new builder-style object to manufacture [`DescribeTaskExecutionInput`](crate::operation::describe_task_execution::DescribeTaskExecutionInput).
    pub fn builder(
    ) -> crate::operation::describe_task_execution::builders::DescribeTaskExecutionInputBuilder
    {
        crate::operation::describe_task_execution::builders::DescribeTaskExecutionInputBuilder::default()
    }
}

/// A builder for [`DescribeTaskExecutionInput`](crate::operation::describe_task_execution::DescribeTaskExecutionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeTaskExecutionInputBuilder {
    pub(crate) task_execution_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeTaskExecutionInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the task that is being executed.</p>
    pub fn task_execution_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.task_execution_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the task that is being executed.</p>
    pub fn set_task_execution_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.task_execution_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeTaskExecutionInput`](crate::operation::describe_task_execution::DescribeTaskExecutionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_task_execution::DescribeTaskExecutionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_task_execution::DescribeTaskExecutionInput {
                task_execution_arn: self.task_execution_arn,
            },
        )
    }
}
