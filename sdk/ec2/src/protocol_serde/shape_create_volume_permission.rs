// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_create_volume_permission(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::CreateVolumePermission,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Group");
    if let Some(var_2) = &input.group {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("UserId");
    if let Some(var_4) = &input.user_id {
        scope_3.string(var_4);
    }
    Ok(())
}

pub fn de_create_volume_permission(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::CreateVolumePermission, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::CreateVolumePermission::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("group") /* Group com.amazonaws.ec2#CreateVolumePermission$Group */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::types::PermissionGroup, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::PermissionGroup::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_group(var_5);
            }
            ,
            s if s.matches("userId") /* UserId com.amazonaws.ec2#CreateVolumePermission$UserId */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_user_id(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
