// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeregisterOnPremisesInstance`](crate::operation::deregister_on_premises_instance::builders::DeregisterOnPremisesInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_name(impl ::std::convert::Into<String>)`](crate::operation::deregister_on_premises_instance::builders::DeregisterOnPremisesInstanceFluentBuilder::instance_name) / [`set_instance_name(Option<String>)`](crate::operation::deregister_on_premises_instance::builders::DeregisterOnPremisesInstanceFluentBuilder::set_instance_name): <p>The name of the on-premises instance to deregister.</p>
    /// - On success, responds with [`DeregisterOnPremisesInstanceOutput`](crate::operation::deregister_on_premises_instance::DeregisterOnPremisesInstanceOutput)
    /// - On failure, responds with [`SdkError<DeregisterOnPremisesInstanceError>`](crate::operation::deregister_on_premises_instance::DeregisterOnPremisesInstanceError)
    pub fn deregister_on_premises_instance(&self) -> crate::operation::deregister_on_premises_instance::builders::DeregisterOnPremisesInstanceFluentBuilder{
        crate::operation::deregister_on_premises_instance::builders::DeregisterOnPremisesInstanceFluentBuilder::new(self.handle.clone())
    }
}
