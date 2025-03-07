// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_approval_rule_template_with_repository_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::associate_approval_rule_template_with_repository::AssociateApprovalRuleTemplateWithRepositoryInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.approval_rule_template_name {
        object
            .key("approvalRuleTemplateName")
            .string(var_1.as_str());
    }
    if let Some(var_2) = &input.repository_name {
        object.key("repositoryName").string(var_2.as_str());
    }
    Ok(())
}
