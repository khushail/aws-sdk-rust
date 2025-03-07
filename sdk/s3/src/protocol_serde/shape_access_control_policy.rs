// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_access_control_policy(
    input: &crate::types::AccessControlPolicy,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.grants {
        let mut inner_writer = scope.start_el("AccessControlList").finish();
        for list_item_2 in var_1 {
            {
                let inner_writer = inner_writer.start_el("Grant");
                crate::protocol_serde::shape_grant::ser_grant(list_item_2, inner_writer)?
            }
        }
    }
    if let Some(var_3) = &input.owner {
        let inner_writer = scope.start_el("Owner");
        crate::protocol_serde::shape_owner::ser_owner(var_3, inner_writer)?
    }
    scope.finish();
    Ok(())
}
