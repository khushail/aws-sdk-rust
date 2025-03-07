// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeSourceServers`](crate::operation::describe_source_servers::builders::DescribeSourceServersFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_source_servers::builders::DescribeSourceServersFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(DescribeSourceServersRequestFilters)`](crate::operation::describe_source_servers::builders::DescribeSourceServersFluentBuilder::filters) / [`set_filters(Option<DescribeSourceServersRequestFilters>)`](crate::operation::describe_source_servers::builders::DescribeSourceServersFluentBuilder::set_filters): <p>A set of filters by which to return Source Servers.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_source_servers::builders::DescribeSourceServersFluentBuilder::max_results) / [`set_max_results(i32)`](crate::operation::describe_source_servers::builders::DescribeSourceServersFluentBuilder::set_max_results): <p>Maximum number of Source Servers to retrieve.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_source_servers::builders::DescribeSourceServersFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_source_servers::builders::DescribeSourceServersFluentBuilder::set_next_token): <p>The token of the next Source Server to retrieve.</p>
    /// - On success, responds with [`DescribeSourceServersOutput`](crate::operation::describe_source_servers::DescribeSourceServersOutput) with field(s):
    ///   - [`items(Option<Vec<SourceServer>>)`](crate::operation::describe_source_servers::DescribeSourceServersOutput::items): <p>An array of Source Servers.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_source_servers::DescribeSourceServersOutput::next_token): <p>The token of the next Source Server to retrieve.</p>
    /// - On failure, responds with [`SdkError<DescribeSourceServersError>`](crate::operation::describe_source_servers::DescribeSourceServersError)
    pub fn describe_source_servers(
        &self,
    ) -> crate::operation::describe_source_servers::builders::DescribeSourceServersFluentBuilder
    {
        crate::operation::describe_source_servers::builders::DescribeSourceServersFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
