// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_job_run_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_job_run::StartJobRunInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.configuration_overrides {
        #[allow(unused_mut)]
        let mut object_3 = object.key("configurationOverrides").start_object();
        crate::protocol_serde::shape_configuration_overrides::ser_configuration_overrides(
            &mut object_3,
            var_2,
        )?;
        object_3.finish();
    }
    if let Some(var_4) = &input.execution_role_arn {
        object.key("executionRoleArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.execution_timeout_minutes {
        object.key("executionTimeoutMinutes").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.job_driver {
        #[allow(unused_mut)]
        let mut object_7 = object.key("jobDriver").start_object();
        crate::protocol_serde::shape_job_driver::ser_job_driver(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.name {
        object.key("name").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tags {
        #[allow(unused_mut)]
        let mut object_10 = object.key("tags").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    Ok(())
}
