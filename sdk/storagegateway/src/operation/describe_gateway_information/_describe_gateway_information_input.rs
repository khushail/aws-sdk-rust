// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A JSON object containing the ID of the gateway.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeGatewayInformationInput {
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    #[doc(hidden)]
    pub gateway_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeGatewayInformationInput {
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn gateway_arn(&self) -> ::std::option::Option<&str> {
        self.gateway_arn.as_deref()
    }
}
impl DescribeGatewayInformationInput {
    /// Creates a new builder-style object to manufacture [`DescribeGatewayInformationInput`](crate::operation::describe_gateway_information::DescribeGatewayInformationInput).
    pub fn builder() -> crate::operation::describe_gateway_information::builders::DescribeGatewayInformationInputBuilder{
        crate::operation::describe_gateway_information::builders::DescribeGatewayInformationInputBuilder::default()
    }
}

/// A builder for [`DescribeGatewayInformationInput`](crate::operation::describe_gateway_information::DescribeGatewayInformationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeGatewayInformationInputBuilder {
    pub(crate) gateway_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeGatewayInformationInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn gateway_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.gateway_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn set_gateway_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.gateway_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeGatewayInformationInput`](crate::operation::describe_gateway_information::DescribeGatewayInformationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_gateway_information::DescribeGatewayInformationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_gateway_information::DescribeGatewayInformationInput {
                gateway_arn: self.gateway_arn,
            },
        )
    }
}
