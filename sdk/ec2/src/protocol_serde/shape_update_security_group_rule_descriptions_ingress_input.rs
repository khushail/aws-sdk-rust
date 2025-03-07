// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_security_group_rule_descriptions_ingress_input_input(
    input: &crate::operation::update_security_group_rule_descriptions_ingress::UpdateSecurityGroupRuleDescriptionsIngressInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(
        &mut out,
        "UpdateSecurityGroupRuleDescriptionsIngress",
        "2016-11-15",
    );
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("GroupId");
    if let Some(var_4) = &input.group_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("GroupName");
    if let Some(var_6) = &input.group_name {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("IpPermissions");
    if let Some(var_8) = &input.ip_permissions {
        let mut list_10 = scope_7.start_list(true, Some("item"));
        for item_9 in var_8 {
            #[allow(unused_mut)]
            let mut entry_11 = list_10.entry();
            crate::protocol_serde::shape_ip_permission::ser_ip_permission(entry_11, item_9)?;
        }
        list_10.finish();
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("SecurityGroupRuleDescription");
    if let Some(var_13) = &input.security_group_rule_descriptions {
        let mut list_15 = scope_12.start_list(true, Some("item"));
        for item_14 in var_13 {
            #[allow(unused_mut)]
            let mut entry_16 = list_15.entry();
            crate::protocol_serde::shape_security_group_rule_description::ser_security_group_rule_description(entry_16, item_14)?;
        }
        list_15.finish();
    }
    writer.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}
