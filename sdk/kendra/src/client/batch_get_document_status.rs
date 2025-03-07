// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchGetDocumentStatus`](crate::operation::batch_get_document_status::builders::BatchGetDocumentStatusFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`index_id(impl ::std::convert::Into<String>)`](crate::operation::batch_get_document_status::builders::BatchGetDocumentStatusFluentBuilder::index_id) / [`set_index_id(Option<String>)`](crate::operation::batch_get_document_status::builders::BatchGetDocumentStatusFluentBuilder::set_index_id): <p>The identifier of the index to add documents to. The index ID is returned by the <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_CreateIndex.html">CreateIndex </a> API.</p>
    ///   - [`document_info_list(Vec<DocumentInfo>)`](crate::operation::batch_get_document_status::builders::BatchGetDocumentStatusFluentBuilder::document_info_list) / [`set_document_info_list(Option<Vec<DocumentInfo>>)`](crate::operation::batch_get_document_status::builders::BatchGetDocumentStatusFluentBuilder::set_document_info_list): <p>A list of <code>DocumentInfo</code> objects that identify the documents for which to get the status. You identify the documents by their document ID and optional attributes.</p>
    /// - On success, responds with [`BatchGetDocumentStatusOutput`](crate::operation::batch_get_document_status::BatchGetDocumentStatusOutput) with field(s):
    ///   - [`errors(Option<Vec<BatchGetDocumentStatusResponseError>>)`](crate::operation::batch_get_document_status::BatchGetDocumentStatusOutput::errors): <p>A list of documents that Amazon Kendra couldn't get the status for. The list includes the ID of the document and the reason that the status couldn't be found.</p>
    ///   - [`document_status_list(Option<Vec<Status>>)`](crate::operation::batch_get_document_status::BatchGetDocumentStatusOutput::document_status_list): <p>The status of documents. The status indicates if the document is waiting to be indexed, is in the process of indexing, has completed indexing, or failed indexing. If a document failed indexing, the status provides the reason why.</p>
    /// - On failure, responds with [`SdkError<BatchGetDocumentStatusError>`](crate::operation::batch_get_document_status::BatchGetDocumentStatusError)
    pub fn batch_get_document_status(
        &self,
    ) -> crate::operation::batch_get_document_status::builders::BatchGetDocumentStatusFluentBuilder
    {
        crate::operation::batch_get_document_status::builders::BatchGetDocumentStatusFluentBuilder::new(self.handle.clone())
    }
}
