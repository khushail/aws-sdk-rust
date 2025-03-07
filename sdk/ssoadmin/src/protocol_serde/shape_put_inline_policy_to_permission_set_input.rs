// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_inline_policy_to_permission_set_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_inline_policy_to_permission_set::PutInlinePolicyToPermissionSetInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.instance_arn {
        object.key("InstanceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.permission_set_arn {
        object.key("PermissionSetArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.inline_policy {
        object.key("InlinePolicy").string(var_3.as_str());
    }
    Ok(())
}
