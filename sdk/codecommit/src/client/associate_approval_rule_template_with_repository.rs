// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateApprovalRuleTemplateWithRepository`](crate::operation::associate_approval_rule_template_with_repository::builders::AssociateApprovalRuleTemplateWithRepositoryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`approval_rule_template_name(impl ::std::convert::Into<String>)`](crate::operation::associate_approval_rule_template_with_repository::builders::AssociateApprovalRuleTemplateWithRepositoryFluentBuilder::approval_rule_template_name) / [`set_approval_rule_template_name(Option<String>)`](crate::operation::associate_approval_rule_template_with_repository::builders::AssociateApprovalRuleTemplateWithRepositoryFluentBuilder::set_approval_rule_template_name): <p>The name for the approval rule template. </p>
    ///   - [`repository_name(impl ::std::convert::Into<String>)`](crate::operation::associate_approval_rule_template_with_repository::builders::AssociateApprovalRuleTemplateWithRepositoryFluentBuilder::repository_name) / [`set_repository_name(Option<String>)`](crate::operation::associate_approval_rule_template_with_repository::builders::AssociateApprovalRuleTemplateWithRepositoryFluentBuilder::set_repository_name): <p>The name of the repository that you want to associate with the template.</p>
    /// - On success, responds with [`AssociateApprovalRuleTemplateWithRepositoryOutput`](crate::operation::associate_approval_rule_template_with_repository::AssociateApprovalRuleTemplateWithRepositoryOutput)
    /// - On failure, responds with [`SdkError<AssociateApprovalRuleTemplateWithRepositoryError>`](crate::operation::associate_approval_rule_template_with_repository::AssociateApprovalRuleTemplateWithRepositoryError)
    pub fn associate_approval_rule_template_with_repository(&self) -> crate::operation::associate_approval_rule_template_with_repository::builders::AssociateApprovalRuleTemplateWithRepositoryFluentBuilder{
        crate::operation::associate_approval_rule_template_with_repository::builders::AssociateApprovalRuleTemplateWithRepositoryFluentBuilder::new(self.handle.clone())
    }
}
