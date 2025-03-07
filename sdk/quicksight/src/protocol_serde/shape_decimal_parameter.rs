// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_decimal_parameter(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DecimalParameter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.values {
        let mut array_3 = object.key("Values").start_array();
        for item_4 in var_2 {
            {
                array_3.value().number(
                    #[allow(clippy::useless_conversion)]
                    ::aws_smithy_types::Number::Float((*item_4).into()),
                );
            }
        }
        array_3.finish();
    }
    Ok(())
}
