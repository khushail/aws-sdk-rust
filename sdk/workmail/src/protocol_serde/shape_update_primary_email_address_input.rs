// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_primary_email_address_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_primary_email_address::UpdatePrimaryEmailAddressInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.organization_id {
        object.key("OrganizationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.entity_id {
        object.key("EntityId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.email {
        object.key("Email").string(var_3.as_str());
    }
    Ok(())
}
