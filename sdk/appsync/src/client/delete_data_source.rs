// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteDataSource`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`api_id(impl ::std::convert::Into<String>)`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::api_id) / [`set_api_id(Option<String>)`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::set_api_id): <p>The API ID.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::set_name): <p>The name of the data source.</p>
    /// - On success, responds with [`DeleteDataSourceOutput`](crate::operation::delete_data_source::DeleteDataSourceOutput)
    /// - On failure, responds with [`SdkError<DeleteDataSourceError>`](crate::operation::delete_data_source::DeleteDataSourceError)
    pub fn delete_data_source(
        &self,
    ) -> crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder {
        crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
