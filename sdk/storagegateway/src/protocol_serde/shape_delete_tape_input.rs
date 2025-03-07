// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_tape_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_tape::DeleteTapeInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.gateway_arn {
        object.key("GatewayARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tape_arn {
        object.key("TapeARN").string(var_2.as_str());
    }
    if input.bypass_governance_retention {
        object
            .key("BypassGovernanceRetention")
            .boolean(input.bypass_governance_retention);
    }
    Ok(())
}
