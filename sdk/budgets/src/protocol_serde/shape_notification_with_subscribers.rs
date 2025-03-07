// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_notification_with_subscribers(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::NotificationWithSubscribers,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.notification {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Notification").start_object();
        crate::protocol_serde::shape_notification::ser_notification(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.subscribers {
        let mut array_4 = object.key("Subscribers").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_subscriber::ser_subscriber(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    Ok(())
}
