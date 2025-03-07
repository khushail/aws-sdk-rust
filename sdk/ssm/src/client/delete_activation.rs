// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteActivation`](crate::operation::delete_activation::builders::DeleteActivationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`activation_id(impl ::std::convert::Into<String>)`](crate::operation::delete_activation::builders::DeleteActivationFluentBuilder::activation_id) / [`set_activation_id(Option<String>)`](crate::operation::delete_activation::builders::DeleteActivationFluentBuilder::set_activation_id): <p>The ID of the activation that you want to delete.</p>
    /// - On success, responds with [`DeleteActivationOutput`](crate::operation::delete_activation::DeleteActivationOutput)
    /// - On failure, responds with [`SdkError<DeleteActivationError>`](crate::operation::delete_activation::DeleteActivationError)
    pub fn delete_activation(
        &self,
    ) -> crate::operation::delete_activation::builders::DeleteActivationFluentBuilder {
        crate::operation::delete_activation::builders::DeleteActivationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
