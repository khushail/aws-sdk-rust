// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetComplianceDetail`](crate::operation::get_compliance_detail::builders::GetComplianceDetailFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`policy_id(impl ::std::convert::Into<String>)`](crate::operation::get_compliance_detail::builders::GetComplianceDetailFluentBuilder::policy_id) / [`set_policy_id(Option<String>)`](crate::operation::get_compliance_detail::builders::GetComplianceDetailFluentBuilder::set_policy_id): <p>The ID of the policy that you want to get the details for. <code>PolicyId</code> is returned by <code>PutPolicy</code> and by <code>ListPolicies</code>.</p>
    ///   - [`member_account(impl ::std::convert::Into<String>)`](crate::operation::get_compliance_detail::builders::GetComplianceDetailFluentBuilder::member_account) / [`set_member_account(Option<String>)`](crate::operation::get_compliance_detail::builders::GetComplianceDetailFluentBuilder::set_member_account): <p>The Amazon Web Services account that owns the resources that you want to get the details for.</p>
    /// - On success, responds with [`GetComplianceDetailOutput`](crate::operation::get_compliance_detail::GetComplianceDetailOutput) with field(s):
    ///   - [`policy_compliance_detail(Option<PolicyComplianceDetail>)`](crate::operation::get_compliance_detail::GetComplianceDetailOutput::policy_compliance_detail): <p>Information about the resources and the policy that you specified in the <code>GetComplianceDetail</code> request.</p>
    /// - On failure, responds with [`SdkError<GetComplianceDetailError>`](crate::operation::get_compliance_detail::GetComplianceDetailError)
    pub fn get_compliance_detail(
        &self,
    ) -> crate::operation::get_compliance_detail::builders::GetComplianceDetailFluentBuilder {
        crate::operation::get_compliance_detail::builders::GetComplianceDetailFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
