// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_contacts_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_contacts::ListContactsInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.filter {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Filter").start_object();
        crate::protocol_serde::shape_list_contacts_filter::ser_list_contacts_filter(
            &mut object_2,
            var_1,
        )?;
        object_2.finish();
    }
    Ok(())
}
