// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_rule_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_rule::UpdateRuleInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resource_tags {
        let mut array_3 = object.key("ResourceTags").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_resource_tag::ser_resource_tag(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.resource_type {
        object.key("ResourceType").string(var_6.as_str());
    }
    if let Some(var_7) = &input.retention_period {
        #[allow(unused_mut)]
        let mut object_8 = object.key("RetentionPeriod").start_object();
        crate::protocol_serde::shape_retention_period::ser_retention_period(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}
