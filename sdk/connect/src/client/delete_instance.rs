// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteInstance`](crate::operation::delete_instance::builders::DeleteInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::delete_instance::builders::DeleteInstanceFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::delete_instance::builders::DeleteInstanceFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    /// - On success, responds with [`DeleteInstanceOutput`](crate::operation::delete_instance::DeleteInstanceOutput)
    /// - On failure, responds with [`SdkError<DeleteInstanceError>`](crate::operation::delete_instance::DeleteInstanceError)
    pub fn delete_instance(
        &self,
    ) -> crate::operation::delete_instance::builders::DeleteInstanceFluentBuilder {
        crate::operation::delete_instance::builders::DeleteInstanceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
