// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeVpcConnectionInput {
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies a MSK VPC connection.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
}
impl DescribeVpcConnectionInput {
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies a MSK VPC connection.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
}
impl DescribeVpcConnectionInput {
    /// Creates a new builder-style object to manufacture [`DescribeVpcConnectionInput`](crate::operation::describe_vpc_connection::DescribeVpcConnectionInput).
    pub fn builder(
    ) -> crate::operation::describe_vpc_connection::builders::DescribeVpcConnectionInputBuilder
    {
        crate::operation::describe_vpc_connection::builders::DescribeVpcConnectionInputBuilder::default()
    }
}

/// A builder for [`DescribeVpcConnectionInput`](crate::operation::describe_vpc_connection::DescribeVpcConnectionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeVpcConnectionInputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
}
impl DescribeVpcConnectionInputBuilder {
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies a MSK VPC connection.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies a MSK VPC connection.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeVpcConnectionInput`](crate::operation::describe_vpc_connection::DescribeVpcConnectionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_vpc_connection::DescribeVpcConnectionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_vpc_connection::DescribeVpcConnectionInput { arn: self.arn },
        )
    }
}
