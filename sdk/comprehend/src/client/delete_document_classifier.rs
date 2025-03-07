// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteDocumentClassifier`](crate::operation::delete_document_classifier::builders::DeleteDocumentClassifierFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`document_classifier_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_document_classifier::builders::DeleteDocumentClassifierFluentBuilder::document_classifier_arn) / [`set_document_classifier_arn(Option<String>)`](crate::operation::delete_document_classifier::builders::DeleteDocumentClassifierFluentBuilder::set_document_classifier_arn): <p>The Amazon Resource Name (ARN) that identifies the document classifier. </p>
    /// - On success, responds with [`DeleteDocumentClassifierOutput`](crate::operation::delete_document_classifier::DeleteDocumentClassifierOutput)
    /// - On failure, responds with [`SdkError<DeleteDocumentClassifierError>`](crate::operation::delete_document_classifier::DeleteDocumentClassifierError)
    pub fn delete_document_classifier(
        &self,
    ) -> crate::operation::delete_document_classifier::builders::DeleteDocumentClassifierFluentBuilder
    {
        crate::operation::delete_document_classifier::builders::DeleteDocumentClassifierFluentBuilder::new(self.handle.clone())
    }
}
