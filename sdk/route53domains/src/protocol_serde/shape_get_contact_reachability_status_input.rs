// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_contact_reachability_status_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_contact_reachability_status::GetContactReachabilityStatusInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.domain_name {
        object.key("domainName").string(var_1.as_str());
    }
    Ok(())
}
