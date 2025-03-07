// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAuditSuppressions`](crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`check_name(impl ::std::convert::Into<String>)`](crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsFluentBuilder::check_name) / [`set_check_name(Option<String>)`](crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsFluentBuilder::set_check_name): <p>An audit check name. Checks must be enabled for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> to select which checks are enabled.)</p>
    ///   - [`resource_identifier(ResourceIdentifier)`](crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsFluentBuilder::resource_identifier) / [`set_resource_identifier(Option<ResourceIdentifier>)`](crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsFluentBuilder::set_resource_identifier): <p>Information that identifies the noncompliant resource.</p>
    ///   - [`ascending_order(bool)`](crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsFluentBuilder::ascending_order) / [`set_ascending_order(Option<bool>)`](crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsFluentBuilder::set_ascending_order): <p> Determines whether suppressions are listed in ascending order by expiration date or not. If parameter isn't provided, <code>ascendingOrder=true</code>. </p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsFluentBuilder::set_next_token): <p> The token for the next set of results. </p>
    ///   - [`max_results(i32)`](crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsFluentBuilder::set_max_results): <p> The maximum number of results to return at one time. The default is 25. </p>
    /// - On success, responds with [`ListAuditSuppressionsOutput`](crate::operation::list_audit_suppressions::ListAuditSuppressionsOutput) with field(s):
    ///   - [`suppressions(Option<Vec<AuditSuppression>>)`](crate::operation::list_audit_suppressions::ListAuditSuppressionsOutput::suppressions): <p> List of audit suppressions. </p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_audit_suppressions::ListAuditSuppressionsOutput::next_token): <p> A token that can be used to retrieve the next set of results, or <code>null</code> if there are no additional results. </p>
    /// - On failure, responds with [`SdkError<ListAuditSuppressionsError>`](crate::operation::list_audit_suppressions::ListAuditSuppressionsError)
    pub fn list_audit_suppressions(
        &self,
    ) -> crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsFluentBuilder
    {
        crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
