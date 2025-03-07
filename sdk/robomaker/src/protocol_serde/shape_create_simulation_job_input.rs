// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_simulation_job_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_simulation_job::CreateSimulationJobInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("clientRequestToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.compute {
        #[allow(unused_mut)]
        let mut object_3 = object.key("compute").start_object();
        crate::protocol_serde::shape_compute::ser_compute(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.data_sources {
        let mut array_5 = object.key("dataSources").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_data_source_config::ser_data_source_config(
                    &mut object_7,
                    item_6,
                )?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.failure_behavior {
        object.key("failureBehavior").string(var_8.as_str());
    }
    if let Some(var_9) = &input.iam_role {
        object.key("iamRole").string(var_9.as_str());
    }
    if let Some(var_10) = &input.logging_config {
        #[allow(unused_mut)]
        let mut object_11 = object.key("loggingConfig").start_object();
        crate::protocol_serde::shape_logging_config::ser_logging_config(&mut object_11, var_10)?;
        object_11.finish();
    }
    {
        object.key("maxJobDurationInSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.max_job_duration_in_seconds).into()),
        );
    }
    if let Some(var_12) = &input.output_location {
        #[allow(unused_mut)]
        let mut object_13 = object.key("outputLocation").start_object();
        crate::protocol_serde::shape_output_location::ser_output_location(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.robot_applications {
        let mut array_15 = object.key("robotApplications").start_array();
        for item_16 in var_14 {
            {
                #[allow(unused_mut)]
                let mut object_17 = array_15.value().start_object();
                crate::protocol_serde::shape_robot_application_config::ser_robot_application_config(&mut object_17, item_16)?;
                object_17.finish();
            }
        }
        array_15.finish();
    }
    if let Some(var_18) = &input.simulation_applications {
        let mut array_19 = object.key("simulationApplications").start_array();
        for item_20 in var_18 {
            {
                #[allow(unused_mut)]
                let mut object_21 = array_19.value().start_object();
                crate::protocol_serde::shape_simulation_application_config::ser_simulation_application_config(&mut object_21, item_20)?;
                object_21.finish();
            }
        }
        array_19.finish();
    }
    if let Some(var_22) = &input.tags {
        #[allow(unused_mut)]
        let mut object_23 = object.key("tags").start_object();
        for (key_24, value_25) in var_22 {
            {
                object_23.key(key_24.as_str()).string(value_25.as_str());
            }
        }
        object_23.finish();
    }
    if let Some(var_26) = &input.vpc_config {
        #[allow(unused_mut)]
        let mut object_27 = object.key("vpcConfig").start_object();
        crate::protocol_serde::shape_vpc_config::ser_vpc_config(&mut object_27, var_26)?;
        object_27.finish();
    }
    Ok(())
}
