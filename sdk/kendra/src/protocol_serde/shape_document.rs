// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_document(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Document,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.id {
        object.key("Id").string(var_1.as_str());
    }
    if let Some(var_2) = &input.title {
        object.key("Title").string(var_2.as_str());
    }
    if let Some(var_3) = &input.blob {
        object
            .key("Blob")
            .string_unchecked(&::aws_smithy_types::base64::encode(var_3));
    }
    if let Some(var_4) = &input.s3_path {
        #[allow(unused_mut)]
        let mut object_5 = object.key("S3Path").start_object();
        crate::protocol_serde::shape_s3_path::ser_s3_path(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.attributes {
        let mut array_7 = object.key("Attributes").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_document_attribute::ser_document_attribute(
                    &mut object_9,
                    item_8,
                )?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.access_control_list {
        let mut array_11 = object.key("AccessControlList").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_principal::ser_principal(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.hierarchical_access_control_list {
        let mut array_15 = object.key("HierarchicalAccessControlList").start_array();
        for item_16 in var_14 {
            {
                #[allow(unused_mut)]
                let mut object_17 = array_15.value().start_object();
                crate::protocol_serde::shape_hierarchical_principal::ser_hierarchical_principal(
                    &mut object_17,
                    item_16,
                )?;
                object_17.finish();
            }
        }
        array_15.finish();
    }
    if let Some(var_18) = &input.content_type {
        object.key("ContentType").string(var_18.as_str());
    }
    if let Some(var_19) = &input.access_control_configuration_id {
        object
            .key("AccessControlConfigurationId")
            .string(var_19.as_str());
    }
    Ok(())
}
