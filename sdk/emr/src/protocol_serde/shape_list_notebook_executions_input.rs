// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_notebook_executions_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_notebook_executions::ListNotebookExecutionsInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.editor_id {
        object.key("EditorId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.status {
        object.key("Status").string(var_2.as_str());
    }
    if let Some(var_3) = &input.from {
        object
            .key("From")
            .date_time(var_3, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_4) = &input.to {
        object
            .key("To")
            .date_time(var_4, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_5) = &input.marker {
        object.key("Marker").string(var_5.as_str());
    }
    if let Some(var_6) = &input.execution_engine_id {
        object.key("ExecutionEngineId").string(var_6.as_str());
    }
    Ok(())
}
