// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_can_dbc_definition(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CanDbcDefinition,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.network_interface {
        object.key("networkInterface").string(var_1.as_str());
    }
    if let Some(var_2) = &input.can_dbc_files {
        let mut array_3 = object.key("canDbcFiles").start_array();
        for item_4 in var_2 {
            {
                array_3
                    .value()
                    .string_unchecked(&::aws_smithy_types::base64::encode(item_4));
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.signals_map {
        #[allow(unused_mut)]
        let mut object_6 = object.key("signalsMap").start_object();
        for (key_7, value_8) in var_5 {
            {
                object_6.key(key_7.as_str()).string(value_8.as_str());
            }
        }
        object_6.finish();
    }
    Ok(())
}
