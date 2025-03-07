// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteQualificationType`](crate::operation::delete_qualification_type::builders::DeleteQualificationTypeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`qualification_type_id(impl ::std::convert::Into<String>)`](crate::operation::delete_qualification_type::builders::DeleteQualificationTypeFluentBuilder::qualification_type_id) / [`set_qualification_type_id(Option<String>)`](crate::operation::delete_qualification_type::builders::DeleteQualificationTypeFluentBuilder::set_qualification_type_id): <p>The ID of the QualificationType to dispose.</p>
    /// - On success, responds with [`DeleteQualificationTypeOutput`](crate::operation::delete_qualification_type::DeleteQualificationTypeOutput)
    /// - On failure, responds with [`SdkError<DeleteQualificationTypeError>`](crate::operation::delete_qualification_type::DeleteQualificationTypeError)
    pub fn delete_qualification_type(
        &self,
    ) -> crate::operation::delete_qualification_type::builders::DeleteQualificationTypeFluentBuilder
    {
        crate::operation::delete_qualification_type::builders::DeleteQualificationTypeFluentBuilder::new(self.handle.clone())
    }
}
