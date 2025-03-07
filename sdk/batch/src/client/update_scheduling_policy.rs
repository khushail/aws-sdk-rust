// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateSchedulingPolicy`](crate::operation::update_scheduling_policy::builders::UpdateSchedulingPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl ::std::convert::Into<String>)`](crate::operation::update_scheduling_policy::builders::UpdateSchedulingPolicyFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::update_scheduling_policy::builders::UpdateSchedulingPolicyFluentBuilder::set_arn): <p>The Amazon Resource Name (ARN) of the scheduling policy to update.</p>
    ///   - [`fairshare_policy(FairsharePolicy)`](crate::operation::update_scheduling_policy::builders::UpdateSchedulingPolicyFluentBuilder::fairshare_policy) / [`set_fairshare_policy(Option<FairsharePolicy>)`](crate::operation::update_scheduling_policy::builders::UpdateSchedulingPolicyFluentBuilder::set_fairshare_policy): <p>The fair share policy.</p>
    /// - On success, responds with [`UpdateSchedulingPolicyOutput`](crate::operation::update_scheduling_policy::UpdateSchedulingPolicyOutput)
    /// - On failure, responds with [`SdkError<UpdateSchedulingPolicyError>`](crate::operation::update_scheduling_policy::UpdateSchedulingPolicyError)
    pub fn update_scheduling_policy(
        &self,
    ) -> crate::operation::update_scheduling_policy::builders::UpdateSchedulingPolicyFluentBuilder
    {
        crate::operation::update_scheduling_policy::builders::UpdateSchedulingPolicyFluentBuilder::new(self.handle.clone())
    }
}
