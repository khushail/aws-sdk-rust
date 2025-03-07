// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListEventDataStores`](crate::operation::list_event_data_stores::builders::ListEventDataStoresFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_event_data_stores::builders::ListEventDataStoresFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_event_data_stores::builders::ListEventDataStoresFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_event_data_stores::builders::ListEventDataStoresFluentBuilder::set_next_token): <p>A token you can use to get the next page of event data store results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_event_data_stores::builders::ListEventDataStoresFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_event_data_stores::builders::ListEventDataStoresFluentBuilder::set_max_results): <p>The maximum number of event data stores to display on a single page.</p>
    /// - On success, responds with [`ListEventDataStoresOutput`](crate::operation::list_event_data_stores::ListEventDataStoresOutput) with field(s):
    ///   - [`event_data_stores(Option<Vec<EventDataStore>>)`](crate::operation::list_event_data_stores::ListEventDataStoresOutput::event_data_stores): <p>Contains information about event data stores in the account, in the current region.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_event_data_stores::ListEventDataStoresOutput::next_token): <p>A token you can use to get the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListEventDataStoresError>`](crate::operation::list_event_data_stores::ListEventDataStoresError)
    pub fn list_event_data_stores(
        &self,
    ) -> crate::operation::list_event_data_stores::builders::ListEventDataStoresFluentBuilder {
        crate::operation::list_event_data_stores::builders::ListEventDataStoresFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
