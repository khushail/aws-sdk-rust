// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteLifecyclePolicy`](crate::operation::delete_lifecycle_policy::builders::DeleteLifecyclePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`container_name(impl ::std::convert::Into<String>)`](crate::operation::delete_lifecycle_policy::builders::DeleteLifecyclePolicyFluentBuilder::container_name) / [`set_container_name(Option<String>)`](crate::operation::delete_lifecycle_policy::builders::DeleteLifecyclePolicyFluentBuilder::set_container_name): <p>The name of the container that holds the object lifecycle policy.</p>
    /// - On success, responds with [`DeleteLifecyclePolicyOutput`](crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyOutput)
    /// - On failure, responds with [`SdkError<DeleteLifecyclePolicyError>`](crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError)
    pub fn delete_lifecycle_policy(
        &self,
    ) -> crate::operation::delete_lifecycle_policy::builders::DeleteLifecyclePolicyFluentBuilder
    {
        crate::operation::delete_lifecycle_policy::builders::DeleteLifecyclePolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
