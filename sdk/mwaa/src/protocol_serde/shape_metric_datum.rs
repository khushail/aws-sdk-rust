// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_metric_datum(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MetricDatum,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.metric_name {
        object.key("MetricName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.timestamp {
        object
            .key("Timestamp")
            .date_time(var_2, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_3) = &input.dimensions {
        let mut array_4 = object.key("Dimensions").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_dimension::ser_dimension(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.value {
        object.key("Value").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.unit {
        object.key("Unit").string(var_8.as_str());
    }
    if let Some(var_9) = &input.statistic_values {
        #[allow(unused_mut)]
        let mut object_10 = object.key("StatisticValues").start_object();
        crate::protocol_serde::shape_statistic_set::ser_statistic_set(&mut object_10, var_9)?;
        object_10.finish();
    }
    Ok(())
}
