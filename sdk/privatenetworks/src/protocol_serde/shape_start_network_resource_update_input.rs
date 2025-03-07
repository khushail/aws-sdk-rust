// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_network_resource_update_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_network_resource_update::StartNetworkResourceUpdateInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.network_resource_arn {
        object.key("networkResourceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.return_reason {
        object.key("returnReason").string(var_2.as_str());
    }
    if let Some(var_3) = &input.shipping_address {
        #[allow(unused_mut)]
        let mut object_4 = object.key("shippingAddress").start_object();
        crate::protocol_serde::shape_address::ser_address(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.update_type {
        object.key("updateType").string(var_5.as_str());
    }
    Ok(())
}
