// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stop_training_job_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::stop_training_job::StopTrainingJobInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.training_job_name {
        object.key("TrainingJobName").string(var_1.as_str());
    }
    Ok(())
}
