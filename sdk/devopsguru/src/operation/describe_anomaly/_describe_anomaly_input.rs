// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeAnomalyInput {
    /// <p> The ID of the anomaly. </p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the member account.</p>
    #[doc(hidden)]
    pub account_id: ::std::option::Option<::std::string::String>,
}
impl DescribeAnomalyInput {
    /// <p> The ID of the anomaly. </p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The ID of the member account.</p>
    pub fn account_id(&self) -> ::std::option::Option<&str> {
        self.account_id.as_deref()
    }
}
impl DescribeAnomalyInput {
    /// Creates a new builder-style object to manufacture [`DescribeAnomalyInput`](crate::operation::describe_anomaly::DescribeAnomalyInput).
    pub fn builder() -> crate::operation::describe_anomaly::builders::DescribeAnomalyInputBuilder {
        crate::operation::describe_anomaly::builders::DescribeAnomalyInputBuilder::default()
    }
}

/// A builder for [`DescribeAnomalyInput`](crate::operation::describe_anomaly::DescribeAnomalyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeAnomalyInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) account_id: ::std::option::Option<::std::string::String>,
}
impl DescribeAnomalyInputBuilder {
    /// <p> The ID of the anomaly. </p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The ID of the anomaly. </p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The ID of the member account.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the member account.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeAnomalyInput`](crate::operation::describe_anomaly::DescribeAnomalyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_anomaly::DescribeAnomalyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_anomaly::DescribeAnomalyInput {
            id: self.id,
            account_id: self.account_id,
        })
    }
}
