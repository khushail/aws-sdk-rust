// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutCapacityAssignmentConfiguration`](crate::operation::put_capacity_assignment_configuration::builders::PutCapacityAssignmentConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`capacity_reservation_name(impl ::std::convert::Into<String>)`](crate::operation::put_capacity_assignment_configuration::builders::PutCapacityAssignmentConfigurationFluentBuilder::capacity_reservation_name) / [`set_capacity_reservation_name(Option<String>)`](crate::operation::put_capacity_assignment_configuration::builders::PutCapacityAssignmentConfigurationFluentBuilder::set_capacity_reservation_name): <p>The name of the capacity reservation to put a capacity assignment configuration for.</p>
    ///   - [`capacity_assignments(Vec<CapacityAssignment>)`](crate::operation::put_capacity_assignment_configuration::builders::PutCapacityAssignmentConfigurationFluentBuilder::capacity_assignments) / [`set_capacity_assignments(Option<Vec<CapacityAssignment>>)`](crate::operation::put_capacity_assignment_configuration::builders::PutCapacityAssignmentConfigurationFluentBuilder::set_capacity_assignments): <p>The list of assignments for the capacity assignment configuration.</p>
    /// - On success, responds with [`PutCapacityAssignmentConfigurationOutput`](crate::operation::put_capacity_assignment_configuration::PutCapacityAssignmentConfigurationOutput)
    /// - On failure, responds with [`SdkError<PutCapacityAssignmentConfigurationError>`](crate::operation::put_capacity_assignment_configuration::PutCapacityAssignmentConfigurationError)
    pub fn put_capacity_assignment_configuration(&self) -> crate::operation::put_capacity_assignment_configuration::builders::PutCapacityAssignmentConfigurationFluentBuilder{
        crate::operation::put_capacity_assignment_configuration::builders::PutCapacityAssignmentConfigurationFluentBuilder::new(self.handle.clone())
    }
}
