// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_query_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_query::StartQueryInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.log_group_name {
        object.key("logGroupName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.log_group_names {
        let mut array_3 = object.key("logGroupNames").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.log_group_identifiers {
        let mut array_6 = object.key("logGroupIdentifiers").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.start_time {
        object.key("startTime").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    if let Some(var_9) = &input.end_time {
        object.key("endTime").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_9).into()),
        );
    }
    if let Some(var_10) = &input.query_string {
        object.key("queryString").string(var_10.as_str());
    }
    if let Some(var_11) = &input.limit {
        object.key("limit").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    Ok(())
}
