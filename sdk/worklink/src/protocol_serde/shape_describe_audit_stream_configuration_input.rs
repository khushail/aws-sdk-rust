// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_audit_stream_configuration_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_audit_stream_configuration::DescribeAuditStreamConfigurationInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.fleet_arn {
        object.key("FleetArn").string(var_1.as_str());
    }
    Ok(())
}
