// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_dataset_org_attributes(
    object_4: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DatasetOrgAttributes,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::types::DatasetOrgAttributes::Vsam(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_4.key("vsam").start_object();
            crate::protocol_serde::shape_vsam_attributes::ser_vsam_attributes(
                &mut object_1,
                inner,
            )?;
            object_1.finish();
        }
        crate::types::DatasetOrgAttributes::Gdg(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_4.key("gdg").start_object();
            crate::protocol_serde::shape_gdg_attributes::ser_gdg_attributes(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::DatasetOrgAttributes::Unknown => {
            return Err(
                ::aws_smithy_http::operation::error::SerializationError::unknown_variant(
                    "DatasetOrgAttributes",
                ),
            )
        }
    }
    Ok(())
}
