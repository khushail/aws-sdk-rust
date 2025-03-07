// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeEventSourcesConfigInput {}
impl DescribeEventSourcesConfigInput {
    /// Creates a new builder-style object to manufacture [`DescribeEventSourcesConfigInput`](crate::operation::describe_event_sources_config::DescribeEventSourcesConfigInput).
    pub fn builder() -> crate::operation::describe_event_sources_config::builders::DescribeEventSourcesConfigInputBuilder{
        crate::operation::describe_event_sources_config::builders::DescribeEventSourcesConfigInputBuilder::default()
    }
}

/// A builder for [`DescribeEventSourcesConfigInput`](crate::operation::describe_event_sources_config::DescribeEventSourcesConfigInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeEventSourcesConfigInputBuilder {}
impl DescribeEventSourcesConfigInputBuilder {
    /// Consumes the builder and constructs a [`DescribeEventSourcesConfigInput`](crate::operation::describe_event_sources_config::DescribeEventSourcesConfigInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_event_sources_config::DescribeEventSourcesConfigInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_event_sources_config::DescribeEventSourcesConfigInput {},
        )
    }
}
