// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_batch_describe_type_configurations_errors(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<
    ::std::vec::Vec<crate::types::BatchDescribeTypeConfigurationsError>,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("member") /* member com.amazonaws.cloudformation#BatchDescribeTypeConfigurationsErrors$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_batch_describe_type_configurations_error::de_batch_describe_type_configurations_error(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
