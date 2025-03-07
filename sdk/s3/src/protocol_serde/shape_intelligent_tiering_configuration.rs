// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_intelligent_tiering_configuration(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::IntelligentTieringConfiguration, ::aws_smithy_xml::decode::XmlDecodeError>
{
    #[allow(unused_mut)]
    let mut builder = crate::types::IntelligentTieringConfiguration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Id") /* Id com.amazonaws.s3#IntelligentTieringConfiguration$Id */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_1);
            }
            ,
            s if s.matches("Filter") /* Filter com.amazonaws.s3#IntelligentTieringConfiguration$Filter */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_intelligent_tiering_filter::de_intelligent_tiering_filter(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_filter(var_2);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.s3#IntelligentTieringConfiguration$Status */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::types::IntelligentTieringStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::IntelligentTieringStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_3);
            }
            ,
            s if s.matches("Tiering") /* Tierings com.amazonaws.s3#IntelligentTieringConfiguration$Tierings */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::vec::Vec<crate::types::Tiering>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_5 = builder.tierings.take().unwrap_or_default();
                            list_5.push(
                                crate::protocol_serde::shape_tiering::de_tiering(&mut tag)
                                ?
                            );
                            list_5
                        })
                        ?
                    )
                ;
                builder = builder.set_tierings(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

pub fn ser_intelligent_tiering_configuration(
    input: &crate::types::IntelligentTieringConfiguration,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_6) = &input.id {
        let mut inner_writer = scope.start_el("Id").finish();
        inner_writer.data(var_6.as_str());
    }
    if let Some(var_7) = &input.filter {
        let inner_writer = scope.start_el("Filter");
        crate::protocol_serde::shape_intelligent_tiering_filter::ser_intelligent_tiering_filter(
            var_7,
            inner_writer,
        )?
    }
    if let Some(var_8) = &input.status {
        let mut inner_writer = scope.start_el("Status").finish();
        inner_writer.data(var_8.as_str());
    }
    if let Some(var_9) = &input.tierings {
        for list_item_10 in var_9 {
            {
                let inner_writer = scope.start_el("Tiering");
                crate::protocol_serde::shape_tiering::ser_tiering(list_item_10, inner_writer)?
            }
        }
    }
    scope.finish();
    Ok(())
}
