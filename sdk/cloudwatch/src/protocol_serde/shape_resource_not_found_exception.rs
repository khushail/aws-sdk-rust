// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn de_resource_not_found_exception_xml_err(
    inp: &[u8],
    mut builder: crate::types::error::builders::ResourceNotFoundExceptionBuilder,
) -> Result<
    crate::types::error::builders::ResourceNotFoundExceptionBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    if inp.is_empty() {
        return Ok(builder);
    }
    let mut document = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    #[allow(unused_mut)]
    let mut error_decoder = crate::rest_xml_wrapped_errors::error_scope(&mut document)?;
    while let Some(mut tag) = error_decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Message") /* Message com.amazonaws.cloudwatch#ResourceNotFoundException$Message */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_message(var_1);
            }
            ,
            s if s.matches("ResourceId") /* ResourceId com.amazonaws.cloudwatch#ResourceNotFoundException$ResourceId */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_id(var_2);
            }
            ,
            s if s.matches("ResourceType") /* ResourceType com.amazonaws.cloudwatch#ResourceNotFoundException$ResourceType */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_type(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
