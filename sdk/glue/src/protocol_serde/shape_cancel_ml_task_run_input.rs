// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cancel_ml_task_run_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::cancel_ml_task_run::CancelMlTaskRunInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.transform_id {
        object.key("TransformId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.task_run_id {
        object.key("TaskRunId").string(var_2.as_str());
    }
    Ok(())
}
