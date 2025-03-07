// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListBatchJobDefinitions`](crate::operation::list_batch_job_definitions::builders::ListBatchJobDefinitionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_batch_job_definitions::builders::ListBatchJobDefinitionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_batch_job_definitions::builders::ListBatchJobDefinitionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_batch_job_definitions::builders::ListBatchJobDefinitionsFluentBuilder::set_next_token): <p>A pagination token returned from a previous call to this operation. This specifies the next item to return. To return to the beginning of the list, exclude this parameter.</p>
    ///   - [`max_results(i32)`](crate::operation::list_batch_job_definitions::builders::ListBatchJobDefinitionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_batch_job_definitions::builders::ListBatchJobDefinitionsFluentBuilder::set_max_results): <p>The maximum number of batch job definitions to return.</p>
    ///   - [`application_id(impl ::std::convert::Into<String>)`](crate::operation::list_batch_job_definitions::builders::ListBatchJobDefinitionsFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::list_batch_job_definitions::builders::ListBatchJobDefinitionsFluentBuilder::set_application_id): <p>The identifier of the application.</p>
    ///   - [`prefix(impl ::std::convert::Into<String>)`](crate::operation::list_batch_job_definitions::builders::ListBatchJobDefinitionsFluentBuilder::prefix) / [`set_prefix(Option<String>)`](crate::operation::list_batch_job_definitions::builders::ListBatchJobDefinitionsFluentBuilder::set_prefix): <p>If the batch job definition is a FileBatchJobDefinition, the prefix allows you to search on the file names of FileBatchJobDefinitions.</p>
    /// - On success, responds with [`ListBatchJobDefinitionsOutput`](crate::operation::list_batch_job_definitions::ListBatchJobDefinitionsOutput) with field(s):
    ///   - [`batch_job_definitions(Option<Vec<BatchJobDefinition>>)`](crate::operation::list_batch_job_definitions::ListBatchJobDefinitionsOutput::batch_job_definitions): <p>The list of batch job definitions.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_batch_job_definitions::ListBatchJobDefinitionsOutput::next_token): <p>If there are more items to return, this contains a token that is passed to a subsequent call to this operation to retrieve the next set of items.</p>
    /// - On failure, responds with [`SdkError<ListBatchJobDefinitionsError>`](crate::operation::list_batch_job_definitions::ListBatchJobDefinitionsError)
    pub fn list_batch_job_definitions(
        &self,
    ) -> crate::operation::list_batch_job_definitions::builders::ListBatchJobDefinitionsFluentBuilder
    {
        crate::operation::list_batch_job_definitions::builders::ListBatchJobDefinitionsFluentBuilder::new(self.handle.clone())
    }
}
