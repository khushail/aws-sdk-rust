// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_ip_address(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UpdateIpAddress,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.ip_id {
        object.key("IpId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.ipv6 {
        object.key("Ipv6").string(var_2.as_str());
    }
    Ok(())
}
