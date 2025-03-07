// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_run_configuration_update(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RunConfigurationUpdate,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.flink_run_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("FlinkRunConfiguration").start_object();
        crate::protocol_serde::shape_flink_run_configuration::ser_flink_run_configuration(
            &mut object_2,
            var_1,
        )?;
        object_2.finish();
    }
    if let Some(var_3) = &input.application_restore_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ApplicationRestoreConfiguration").start_object();
        crate::protocol_serde::shape_application_restore_configuration::ser_application_restore_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}
