// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_experiment_template_stop_condition_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CreateExperimentTemplateStopConditionInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.source {
        object.key("source").string(var_1.as_str());
    }
    if let Some(var_2) = &input.value {
        object.key("value").string(var_2.as_str());
    }
    Ok(())
}
