// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_notify_workers_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::notify_workers::NotifyWorkersInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.subject {
        object.key("Subject").string(var_1.as_str());
    }
    if let Some(var_2) = &input.message_text {
        object.key("MessageText").string(var_2.as_str());
    }
    if let Some(var_3) = &input.worker_ids {
        let mut array_4 = object.key("WorkerIds").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}
