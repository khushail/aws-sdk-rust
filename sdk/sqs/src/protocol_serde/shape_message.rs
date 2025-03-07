// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_message(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::Message, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Message::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("MessageId") /* MessageId com.amazonaws.sqs#Message$MessageId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_message_id(var_1);
            }
            ,
            s if s.matches("ReceiptHandle") /* ReceiptHandle com.amazonaws.sqs#Message$ReceiptHandle */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_receipt_handle(var_2);
            }
            ,
            s if s.matches("MD5OfBody") /* MD5OfBody com.amazonaws.sqs#Message$MD5OfBody */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_md5_of_body(var_3);
            }
            ,
            s if s.matches("Body") /* Body com.amazonaws.sqs#Message$Body */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_body(var_4);
            }
            ,
            s if s.matches("Attribute") /* Attributes com.amazonaws.sqs#Message$Attributes */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::collections::HashMap<crate::types::MessageSystemAttributeName, ::std::string::String>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut map_6 = builder.attributes.take().unwrap_or_default();
                                            crate::protocol_serde::shape_message_system_attribute_map::de_message_system_attribute_map_entry(&mut tag, &mut map_6)?;
                                            map_6
                        })
                        ?
                    )
                ;
                builder = builder.set_attributes(var_5);
            }
            ,
            s if s.matches("MD5OfMessageAttributes") /* MD5OfMessageAttributes com.amazonaws.sqs#Message$MD5OfMessageAttributes */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_md5_of_message_attributes(var_7);
            }
            ,
            s if s.matches("MessageAttribute") /* MessageAttributes com.amazonaws.sqs#Message$MessageAttributes */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::collections::HashMap<::std::string::String, crate::types::MessageAttributeValue>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut map_9 = builder.message_attributes.take().unwrap_or_default();
                                            crate::protocol_serde::shape_message_body_attribute_map::de_message_body_attribute_map_entry(&mut tag, &mut map_9)?;
                                            map_9
                        })
                        ?
                    )
                ;
                builder = builder.set_message_attributes(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
