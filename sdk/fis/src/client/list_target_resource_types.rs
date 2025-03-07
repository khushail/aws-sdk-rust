// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTargetResourceTypes`](crate::operation::list_target_resource_types::builders::ListTargetResourceTypesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_target_resource_types::builders::ListTargetResourceTypesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_target_resource_types::builders::ListTargetResourceTypesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_target_resource_types::builders::ListTargetResourceTypesFluentBuilder::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_target_resource_types::builders::ListTargetResourceTypesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_target_resource_types::builders::ListTargetResourceTypesFluentBuilder::set_next_token): <p>The token for the next page of results.</p>
    /// - On success, responds with [`ListTargetResourceTypesOutput`](crate::operation::list_target_resource_types::ListTargetResourceTypesOutput) with field(s):
    ///   - [`target_resource_types(Option<Vec<TargetResourceTypeSummary>>)`](crate::operation::list_target_resource_types::ListTargetResourceTypesOutput::target_resource_types): <p>The target resource types.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_target_resource_types::ListTargetResourceTypesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<ListTargetResourceTypesError>`](crate::operation::list_target_resource_types::ListTargetResourceTypesError)
    pub fn list_target_resource_types(
        &self,
    ) -> crate::operation::list_target_resource_types::builders::ListTargetResourceTypesFluentBuilder
    {
        crate::operation::list_target_resource_types::builders::ListTargetResourceTypesFluentBuilder::new(self.handle.clone())
    }
}
