// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_lifecycle_policy_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_lifecycle_policy::PutLifecyclePolicyInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.registry_id {
        object.key("registryId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.repository_name {
        object.key("repositoryName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.lifecycle_policy_text {
        object.key("lifecyclePolicyText").string(var_3.as_str());
    }
    Ok(())
}
