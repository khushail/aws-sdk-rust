// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ClearQuerySuggestions`](crate::operation::clear_query_suggestions::builders::ClearQuerySuggestionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`index_id(impl ::std::convert::Into<String>)`](crate::operation::clear_query_suggestions::builders::ClearQuerySuggestionsFluentBuilder::index_id) / [`set_index_id(Option<String>)`](crate::operation::clear_query_suggestions::builders::ClearQuerySuggestionsFluentBuilder::set_index_id): <p>The identifier of the index you want to clear query suggestions from.</p>
    /// - On success, responds with [`ClearQuerySuggestionsOutput`](crate::operation::clear_query_suggestions::ClearQuerySuggestionsOutput)
    /// - On failure, responds with [`SdkError<ClearQuerySuggestionsError>`](crate::operation::clear_query_suggestions::ClearQuerySuggestionsError)
    pub fn clear_query_suggestions(
        &self,
    ) -> crate::operation::clear_query_suggestions::builders::ClearQuerySuggestionsFluentBuilder
    {
        crate::operation::clear_query_suggestions::builders::ClearQuerySuggestionsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
