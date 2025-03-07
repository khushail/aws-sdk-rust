// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListOperations`](crate::operation::list_operations::builders::ListOperationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl ::std::convert::Into<String>)`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::set_application_id): <p>The ID of the application.</p>
    ///   - [`max_results(i32)`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned nextToken value. If you do not specify a value for MaxResults, the request returns 50 items per page by default.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::set_next_token): <p>The token for the next page of results. </p>
    ///   - [`filters(Vec<Filter>)`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::set_filters): <p>The filters of an operation.</p>
    /// - On success, responds with [`ListOperationsOutput`](crate::operation::list_operations::ListOperationsOutput) with field(s):
    ///   - [`operations(Option<Vec<Operation>>)`](crate::operation::list_operations::ListOperationsOutput::operations): <p>List of operations performed by AWS Systems Manager for SAP.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_operations::ListOperationsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<ListOperationsError>`](crate::operation::list_operations::ListOperationsError)
    pub fn list_operations(
        &self,
    ) -> crate::operation::list_operations::builders::ListOperationsFluentBuilder {
        crate::operation::list_operations::builders::ListOperationsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
