// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListEvents`](crate::operation::list_events::builders::ListEventsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_events::builders::ListEventsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(ListEventsFilters)`](crate::operation::list_events::builders::ListEventsFluentBuilder::filters) / [`set_filters(Option<ListEventsFilters>)`](crate::operation::list_events::builders::ListEventsFluentBuilder::set_filters): <p> A <code>ListEventsFilters</code> object used to specify which events to return. </p>
    ///   - [`max_results(i32)`](crate::operation::list_events::builders::ListEventsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_events::builders::ListEventsFluentBuilder::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_events::builders::ListEventsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_events::builders::ListEventsFluentBuilder::set_next_token): <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::list_events::builders::ListEventsFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::list_events::builders::ListEventsFluentBuilder::set_account_id): <p>The ID of the Amazon Web Services account. </p>
    /// - On success, responds with [`ListEventsOutput`](crate::operation::list_events::ListEventsOutput) with field(s):
    ///   - [`events(Option<Vec<Event>>)`](crate::operation::list_events::ListEventsOutput::events): <p> A list of the requested events. </p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_events::ListEventsOutput::next_token): <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    /// - On failure, responds with [`SdkError<ListEventsError>`](crate::operation::list_events::ListEventsError)
    pub fn list_events(&self) -> crate::operation::list_events::builders::ListEventsFluentBuilder {
        crate::operation::list_events::builders::ListEventsFluentBuilder::new(self.handle.clone())
    }
}
