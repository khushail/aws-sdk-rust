// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteRevision`](crate::operation::delete_revision::builders::DeleteRevisionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`data_set_id(impl ::std::convert::Into<String>)`](crate::operation::delete_revision::builders::DeleteRevisionFluentBuilder::data_set_id) / [`set_data_set_id(Option<String>)`](crate::operation::delete_revision::builders::DeleteRevisionFluentBuilder::set_data_set_id): <p>The unique identifier for a data set.</p>
    ///   - [`revision_id(impl ::std::convert::Into<String>)`](crate::operation::delete_revision::builders::DeleteRevisionFluentBuilder::revision_id) / [`set_revision_id(Option<String>)`](crate::operation::delete_revision::builders::DeleteRevisionFluentBuilder::set_revision_id): <p>The unique identifier for a revision.</p>
    /// - On success, responds with [`DeleteRevisionOutput`](crate::operation::delete_revision::DeleteRevisionOutput)
    /// - On failure, responds with [`SdkError<DeleteRevisionError>`](crate::operation::delete_revision::DeleteRevisionError)
    pub fn delete_revision(
        &self,
    ) -> crate::operation::delete_revision::builders::DeleteRevisionFluentBuilder {
        crate::operation::delete_revision::builders::DeleteRevisionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
