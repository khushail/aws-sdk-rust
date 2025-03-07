// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteTask`](crate::operation::delete_task::builders::DeleteTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`task_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_task::builders::DeleteTaskFluentBuilder::task_arn) / [`set_task_arn(Option<String>)`](crate::operation::delete_task::builders::DeleteTaskFluentBuilder::set_task_arn): <p>Specifies the Amazon Resource Name (ARN) of the task that you want to delete.</p>
    /// - On success, responds with [`DeleteTaskOutput`](crate::operation::delete_task::DeleteTaskOutput)
    /// - On failure, responds with [`SdkError<DeleteTaskError>`](crate::operation::delete_task::DeleteTaskError)
    pub fn delete_task(&self) -> crate::operation::delete_task::builders::DeleteTaskFluentBuilder {
        crate::operation::delete_task::builders::DeleteTaskFluentBuilder::new(self.handle.clone())
    }
}
