// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeResourcePolicies`](crate::operation::describe_resource_policies::builders::DescribeResourcePoliciesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_resource_policies::builders::DescribeResourcePoliciesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_resource_policies::builders::DescribeResourcePoliciesFluentBuilder::set_next_token): <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    ///   - [`limit(i32)`](crate::operation::describe_resource_policies::builders::DescribeResourcePoliciesFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::describe_resource_policies::builders::DescribeResourcePoliciesFluentBuilder::set_limit): <p>The maximum number of resource policies to be displayed with one call of this API.</p>
    /// - On success, responds with [`DescribeResourcePoliciesOutput`](crate::operation::describe_resource_policies::DescribeResourcePoliciesOutput) with field(s):
    ///   - [`resource_policies(Option<Vec<ResourcePolicy>>)`](crate::operation::describe_resource_policies::DescribeResourcePoliciesOutput::resource_policies): <p>The resource policies that exist in this account.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_resource_policies::DescribeResourcePoliciesOutput::next_token): <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    /// - On failure, responds with [`SdkError<DescribeResourcePoliciesError>`](crate::operation::describe_resource_policies::DescribeResourcePoliciesError)
    pub fn describe_resource_policies(
        &self,
    ) -> crate::operation::describe_resource_policies::builders::DescribeResourcePoliciesFluentBuilder
    {
        crate::operation::describe_resource_policies::builders::DescribeResourcePoliciesFluentBuilder::new(self.handle.clone())
    }
}
