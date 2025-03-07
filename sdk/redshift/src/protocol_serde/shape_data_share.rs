// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_data_share(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::DataShare, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::DataShare::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("DataShareArn") /* DataShareArn com.amazonaws.redshift#DataShare$DataShareArn */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_data_share_arn(var_1);
            }
            ,
            s if s.matches("ProducerArn") /* ProducerArn com.amazonaws.redshift#DataShare$ProducerArn */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_producer_arn(var_2);
            }
            ,
            s if s.matches("AllowPubliclyAccessibleConsumers") /* AllowPubliclyAccessibleConsumers com.amazonaws.redshift#DataShare$AllowPubliclyAccessibleConsumers */ =>  {
                let var_3 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.redshift#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_allow_publicly_accessible_consumers(var_3);
            }
            ,
            s if s.matches("DataShareAssociations") /* DataShareAssociations com.amazonaws.redshift#DataShare$DataShareAssociations */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_data_share_association_list::de_data_share_association_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_data_share_associations(var_4);
            }
            ,
            s if s.matches("ManagedBy") /* ManagedBy com.amazonaws.redshift#DataShare$ManagedBy */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_managed_by(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
