// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelTaskExecution`](crate::operation::cancel_task_execution::builders::CancelTaskExecutionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`task_execution_arn(impl ::std::convert::Into<String>)`](crate::operation::cancel_task_execution::builders::CancelTaskExecutionFluentBuilder::task_execution_arn) / [`set_task_execution_arn(Option<String>)`](crate::operation::cancel_task_execution::builders::CancelTaskExecutionFluentBuilder::set_task_execution_arn): <p>The Amazon Resource Name (ARN) of the task execution to stop.</p>
    /// - On success, responds with [`CancelTaskExecutionOutput`](crate::operation::cancel_task_execution::CancelTaskExecutionOutput)
    /// - On failure, responds with [`SdkError<CancelTaskExecutionError>`](crate::operation::cancel_task_execution::CancelTaskExecutionError)
    pub fn cancel_task_execution(
        &self,
    ) -> crate::operation::cancel_task_execution::builders::CancelTaskExecutionFluentBuilder {
        crate::operation::cancel_task_execution::builders::CancelTaskExecutionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
