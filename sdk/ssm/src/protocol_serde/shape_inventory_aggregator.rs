// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_inventory_aggregator(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::InventoryAggregator,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.expression {
        object.key("Expression").string(var_1.as_str());
    }
    if let Some(var_2) = &input.aggregators {
        let mut array_3 = object.key("Aggregators").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_inventory_aggregator::ser_inventory_aggregator(
                    &mut object_5,
                    item_4,
                )?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.groups {
        let mut array_7 = object.key("Groups").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_inventory_group::ser_inventory_group(
                    &mut object_9,
                    item_8,
                )?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    Ok(())
}
