// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateResource`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`role_arn(impl ::std::convert::Into<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::set_role_arn): <p>The new role to use for the given resource registered in Lake Formation.</p>
    ///   - [`resource_arn(impl ::std::convert::Into<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::set_resource_arn): <p>The resource ARN.</p>
    ///   - [`with_federation(bool)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::with_federation) / [`set_with_federation(Option<bool>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::set_with_federation): <p>Whether or not the resource is a federated resource.</p>
    /// - On success, responds with [`UpdateResourceOutput`](crate::operation::update_resource::UpdateResourceOutput)
    /// - On failure, responds with [`SdkError<UpdateResourceError>`](crate::operation::update_resource::UpdateResourceError)
    pub fn update_resource(
        &self,
    ) -> crate::operation::update_resource::builders::UpdateResourceFluentBuilder {
        crate::operation::update_resource::builders::UpdateResourceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
