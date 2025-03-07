// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutResourcePolicy`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`policy_document(impl ::std::convert::Into<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::policy_document) / [`set_policy_document(Option<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::set_policy_document): <p>The JSON resource policy document.</p>
    ///   - [`resource_arn(impl ::std::convert::Into<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::set_resource_arn): <p>The ARN of the resource policy. </p>
    /// - On success, responds with [`PutResourcePolicyOutput`](crate::operation::put_resource_policy::PutResourcePolicyOutput)
    /// - On failure, responds with [`SdkError<PutResourcePolicyError>`](crate::operation::put_resource_policy::PutResourcePolicyError)
    pub fn put_resource_policy(
        &self,
    ) -> crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder {
        crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
