// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_vpc_cidr_block_input_input(
    input: &crate::operation::disassociate_vpc_cidr_block::DisassociateVpcCidrBlockInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        ::aws_smithy_query::QueryWriter::new(&mut out, "DisassociateVpcCidrBlock", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AssociationId");
    if let Some(var_2) = &input.association_id {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}
