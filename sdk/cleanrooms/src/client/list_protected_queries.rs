// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListProtectedQueries`](crate::operation::list_protected_queries::builders::ListProtectedQueriesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_protected_queries::builders::ListProtectedQueriesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`membership_identifier(impl ::std::convert::Into<String>)`](crate::operation::list_protected_queries::builders::ListProtectedQueriesFluentBuilder::membership_identifier) / [`set_membership_identifier(Option<String>)`](crate::operation::list_protected_queries::builders::ListProtectedQueriesFluentBuilder::set_membership_identifier): <p>The identifier for the membership in the collaboration.</p>
    ///   - [`status(ProtectedQueryStatus)`](crate::operation::list_protected_queries::builders::ListProtectedQueriesFluentBuilder::status) / [`set_status(Option<ProtectedQueryStatus>)`](crate::operation::list_protected_queries::builders::ListProtectedQueriesFluentBuilder::set_status): <p>A filter on the status of the protected query.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_protected_queries::builders::ListProtectedQueriesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_protected_queries::builders::ListProtectedQueriesFluentBuilder::set_next_token): <p>The token value retrieved from a previous call to access the next page of results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_protected_queries::builders::ListProtectedQueriesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_protected_queries::builders::ListProtectedQueriesFluentBuilder::set_max_results): <p>The maximum size of the results that is returned per call. Service chooses a default if it has not been set. Service can return a nextToken even if the maximum results has not been met. </p>
    /// - On success, responds with [`ListProtectedQueriesOutput`](crate::operation::list_protected_queries::ListProtectedQueriesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_protected_queries::ListProtectedQueriesOutput::next_token): <p>The token value retrieved from a previous call to access the next page of results.</p>
    ///   - [`protected_queries(Option<Vec<ProtectedQuerySummary>>)`](crate::operation::list_protected_queries::ListProtectedQueriesOutput::protected_queries): <p>A list of protected queries.</p>
    /// - On failure, responds with [`SdkError<ListProtectedQueriesError>`](crate::operation::list_protected_queries::ListProtectedQueriesError)
    pub fn list_protected_queries(
        &self,
    ) -> crate::operation::list_protected_queries::builders::ListProtectedQueriesFluentBuilder {
        crate::operation::list_protected_queries::builders::ListProtectedQueriesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
