// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_instance_refresh_input_input(
    input: &crate::operation::start_instance_refresh::StartInstanceRefreshInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        ::aws_smithy_query::QueryWriter::new(&mut out, "StartInstanceRefresh", "2011-01-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AutoScalingGroupName");
    if let Some(var_2) = &input.auto_scaling_group_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Strategy");
    if let Some(var_4) = &input.strategy {
        scope_3.string(var_4.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DesiredConfiguration");
    if let Some(var_6) = &input.desired_configuration {
        crate::protocol_serde::shape_desired_configuration::ser_desired_configuration(
            scope_5, var_6,
        )?;
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Preferences");
    if let Some(var_8) = &input.preferences {
        crate::protocol_serde::shape_refresh_preferences::ser_refresh_preferences(scope_7, var_8)?;
    }
    writer.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}
