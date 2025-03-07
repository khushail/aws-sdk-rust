// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAppsList`](crate::operation::delete_apps_list::builders::DeleteAppsListFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`list_id(impl ::std::convert::Into<String>)`](crate::operation::delete_apps_list::builders::DeleteAppsListFluentBuilder::list_id) / [`set_list_id(Option<String>)`](crate::operation::delete_apps_list::builders::DeleteAppsListFluentBuilder::set_list_id): <p>The ID of the applications list that you want to delete. You can retrieve this ID from <code>PutAppsList</code>, <code>ListAppsLists</code>, and <code>GetAppsList</code>.</p>
    /// - On success, responds with [`DeleteAppsListOutput`](crate::operation::delete_apps_list::DeleteAppsListOutput)
    /// - On failure, responds with [`SdkError<DeleteAppsListError>`](crate::operation::delete_apps_list::DeleteAppsListError)
    pub fn delete_apps_list(
        &self,
    ) -> crate::operation::delete_apps_list::builders::DeleteAppsListFluentBuilder {
        crate::operation::delete_apps_list::builders::DeleteAppsListFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
