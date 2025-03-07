// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListChangesets`](crate::operation::list_changesets::builders::ListChangesetsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_changesets::builders::ListChangesetsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`dataset_id(impl ::std::convert::Into<String>)`](crate::operation::list_changesets::builders::ListChangesetsFluentBuilder::dataset_id) / [`set_dataset_id(Option<String>)`](crate::operation::list_changesets::builders::ListChangesetsFluentBuilder::set_dataset_id): <p>The unique identifier for the FinSpace Dataset to which the Changeset belongs.</p>
    ///   - [`max_results(i32)`](crate::operation::list_changesets::builders::ListChangesetsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_changesets::builders::ListChangesetsFluentBuilder::set_max_results): <p>The maximum number of results per page.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_changesets::builders::ListChangesetsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_changesets::builders::ListChangesetsFluentBuilder::set_next_token): <p>A token that indicates where a results page should begin.</p>
    /// - On success, responds with [`ListChangesetsOutput`](crate::operation::list_changesets::ListChangesetsOutput) with field(s):
    ///   - [`changesets(Option<Vec<ChangesetSummary>>)`](crate::operation::list_changesets::ListChangesetsOutput::changesets): <p>List of Changesets found.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_changesets::ListChangesetsOutput::next_token): <p>A token that indicates where a results page should begin.</p>
    /// - On failure, responds with [`SdkError<ListChangesetsError>`](crate::operation::list_changesets::ListChangesetsError)
    pub fn list_changesets(
        &self,
    ) -> crate::operation::list_changesets::builders::ListChangesetsFluentBuilder {
        crate::operation::list_changesets::builders::ListChangesetsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
