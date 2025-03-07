// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_receipt_rule_input_input(
    input: &crate::operation::update_receipt_rule::UpdateReceiptRuleInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        ::aws_smithy_query::QueryWriter::new(&mut out, "UpdateReceiptRule", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("RuleSetName");
    if let Some(var_2) = &input.rule_set_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Rule");
    if let Some(var_4) = &input.rule {
        crate::protocol_serde::shape_receipt_rule::ser_receipt_rule(scope_3, var_4)?;
    }
    writer.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}
