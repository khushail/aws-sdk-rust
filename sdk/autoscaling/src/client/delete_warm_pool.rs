// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteWarmPool`](crate::operation::delete_warm_pool::builders::DeleteWarmPoolFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`auto_scaling_group_name(impl ::std::convert::Into<String>)`](crate::operation::delete_warm_pool::builders::DeleteWarmPoolFluentBuilder::auto_scaling_group_name) / [`set_auto_scaling_group_name(Option<String>)`](crate::operation::delete_warm_pool::builders::DeleteWarmPoolFluentBuilder::set_auto_scaling_group_name): <p>The name of the Auto Scaling group.</p>
    ///   - [`force_delete(bool)`](crate::operation::delete_warm_pool::builders::DeleteWarmPoolFluentBuilder::force_delete) / [`set_force_delete(Option<bool>)`](crate::operation::delete_warm_pool::builders::DeleteWarmPoolFluentBuilder::set_force_delete): <p>Specifies that the warm pool is to be deleted along with all of its associated instances, without waiting for all instances to be terminated. This parameter also deletes any outstanding lifecycle actions associated with the warm pool instances.</p>
    /// - On success, responds with [`DeleteWarmPoolOutput`](crate::operation::delete_warm_pool::DeleteWarmPoolOutput)
    /// - On failure, responds with [`SdkError<DeleteWarmPoolError>`](crate::operation::delete_warm_pool::DeleteWarmPoolError)
    pub fn delete_warm_pool(
        &self,
    ) -> crate::operation::delete_warm_pool::builders::DeleteWarmPoolFluentBuilder {
        crate::operation::delete_warm_pool::builders::DeleteWarmPoolFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
