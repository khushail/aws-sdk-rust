// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_filter(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Filter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.key {
        object.key("key").string(var_1.as_str());
    }
    if let Some(var_2) = &input.condition {
        #[allow(unused_mut)]
        let mut object_3 = object.key("condition").start_object();
        crate::protocol_serde::shape_condition::ser_condition(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}
