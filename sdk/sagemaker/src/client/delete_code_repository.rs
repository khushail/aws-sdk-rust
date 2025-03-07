// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCodeRepository`](crate::operation::delete_code_repository::builders::DeleteCodeRepositoryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`code_repository_name(impl ::std::convert::Into<String>)`](crate::operation::delete_code_repository::builders::DeleteCodeRepositoryFluentBuilder::code_repository_name) / [`set_code_repository_name(Option<String>)`](crate::operation::delete_code_repository::builders::DeleteCodeRepositoryFluentBuilder::set_code_repository_name): <p>The name of the Git repository to delete.</p>
    /// - On success, responds with [`DeleteCodeRepositoryOutput`](crate::operation::delete_code_repository::DeleteCodeRepositoryOutput)
    /// - On failure, responds with [`SdkError<DeleteCodeRepositoryError>`](crate::operation::delete_code_repository::DeleteCodeRepositoryError)
    pub fn delete_code_repository(
        &self,
    ) -> crate::operation::delete_code_repository::builders::DeleteCodeRepositoryFluentBuilder {
        crate::operation::delete_code_repository::builders::DeleteCodeRepositoryFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
