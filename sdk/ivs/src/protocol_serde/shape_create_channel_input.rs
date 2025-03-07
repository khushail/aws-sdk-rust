// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_channel_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_channel::CreateChannelInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if input.authorized {
        object.key("authorized").boolean(input.authorized);
    }
    if input.insecure_ingest {
        object.key("insecureIngest").boolean(input.insecure_ingest);
    }
    if let Some(var_1) = &input.latency_mode {
        object.key("latencyMode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.recording_configuration_arn {
        object
            .key("recordingConfigurationArn")
            .string(var_3.as_str());
    }
    if let Some(var_4) = &input.tags {
        #[allow(unused_mut)]
        let mut object_5 = object.key("tags").start_object();
        for (key_6, value_7) in var_4 {
            {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.r#type {
        object.key("type").string(var_8.as_str());
    }
    Ok(())
}
