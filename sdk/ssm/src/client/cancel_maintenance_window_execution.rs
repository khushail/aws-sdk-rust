// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelMaintenanceWindowExecution`](crate::operation::cancel_maintenance_window_execution::builders::CancelMaintenanceWindowExecutionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`window_execution_id(impl ::std::convert::Into<String>)`](crate::operation::cancel_maintenance_window_execution::builders::CancelMaintenanceWindowExecutionFluentBuilder::window_execution_id) / [`set_window_execution_id(Option<String>)`](crate::operation::cancel_maintenance_window_execution::builders::CancelMaintenanceWindowExecutionFluentBuilder::set_window_execution_id): <p>The ID of the maintenance window execution to stop.</p>
    /// - On success, responds with [`CancelMaintenanceWindowExecutionOutput`](crate::operation::cancel_maintenance_window_execution::CancelMaintenanceWindowExecutionOutput) with field(s):
    ///   - [`window_execution_id(Option<String>)`](crate::operation::cancel_maintenance_window_execution::CancelMaintenanceWindowExecutionOutput::window_execution_id): <p>The ID of the maintenance window execution that has been stopped.</p>
    /// - On failure, responds with [`SdkError<CancelMaintenanceWindowExecutionError>`](crate::operation::cancel_maintenance_window_execution::CancelMaintenanceWindowExecutionError)
    pub fn cancel_maintenance_window_execution(&self) -> crate::operation::cancel_maintenance_window_execution::builders::CancelMaintenanceWindowExecutionFluentBuilder{
        crate::operation::cancel_maintenance_window_execution::builders::CancelMaintenanceWindowExecutionFluentBuilder::new(self.handle.clone())
    }
}
