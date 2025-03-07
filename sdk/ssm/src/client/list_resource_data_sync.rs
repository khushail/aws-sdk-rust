// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListResourceDataSync`](crate::operation::list_resource_data_sync::builders::ListResourceDataSyncFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_resource_data_sync::builders::ListResourceDataSyncFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`sync_type(impl ::std::convert::Into<String>)`](crate::operation::list_resource_data_sync::builders::ListResourceDataSyncFluentBuilder::sync_type) / [`set_sync_type(Option<String>)`](crate::operation::list_resource_data_sync::builders::ListResourceDataSyncFluentBuilder::set_sync_type): <p>View a list of resource data syncs according to the sync type. Specify <code>SyncToDestination</code> to view resource data syncs that synchronize data to an Amazon S3 bucket. Specify <code>SyncFromSource</code> to view resource data syncs from Organizations or from multiple Amazon Web Services Regions.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_resource_data_sync::builders::ListResourceDataSyncFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_resource_data_sync::builders::ListResourceDataSyncFluentBuilder::set_next_token): <p>A token to start the list. Use this token to get the next set of results. </p>
    ///   - [`max_results(i32)`](crate::operation::list_resource_data_sync::builders::ListResourceDataSyncFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_resource_data_sync::builders::ListResourceDataSyncFluentBuilder::set_max_results): <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    /// - On success, responds with [`ListResourceDataSyncOutput`](crate::operation::list_resource_data_sync::ListResourceDataSyncOutput) with field(s):
    ///   - [`resource_data_sync_items(Option<Vec<ResourceDataSyncItem>>)`](crate::operation::list_resource_data_sync::ListResourceDataSyncOutput::resource_data_sync_items): <p>A list of your current resource data sync configurations and their statuses.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_resource_data_sync::ListResourceDataSyncOutput::next_token): <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    /// - On failure, responds with [`SdkError<ListResourceDataSyncError>`](crate::operation::list_resource_data_sync::ListResourceDataSyncError)
    pub fn list_resource_data_sync(
        &self,
    ) -> crate::operation::list_resource_data_sync::builders::ListResourceDataSyncFluentBuilder
    {
        crate::operation::list_resource_data_sync::builders::ListResourceDataSyncFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
