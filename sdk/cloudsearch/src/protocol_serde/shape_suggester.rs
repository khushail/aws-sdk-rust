// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_suggester(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::Suggester,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("SuggesterName");
    if let Some(var_2) = &input.suggester_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("DocumentSuggesterOptions");
    if let Some(var_4) = &input.document_suggester_options {
        crate::protocol_serde::shape_document_suggester_options::ser_document_suggester_options(
            scope_3, var_4,
        )?;
    }
    Ok(())
}

pub fn de_suggester(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::Suggester, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Suggester::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("SuggesterName") /* SuggesterName com.amazonaws.cloudsearch#Suggester$SuggesterName */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_suggester_name(var_5);
            }
            ,
            s if s.matches("DocumentSuggesterOptions") /* DocumentSuggesterOptions com.amazonaws.cloudsearch#Suggester$DocumentSuggesterOptions */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_document_suggester_options::de_document_suggester_options(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_document_suggester_options(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
