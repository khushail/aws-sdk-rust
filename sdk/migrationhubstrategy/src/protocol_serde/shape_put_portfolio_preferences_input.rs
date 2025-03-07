// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_portfolio_preferences_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_portfolio_preferences::PutPortfolioPreferencesInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.application_mode {
        object.key("applicationMode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.application_preferences {
        #[allow(unused_mut)]
        let mut object_3 = object.key("applicationPreferences").start_object();
        crate::protocol_serde::shape_application_preferences::ser_application_preferences(
            &mut object_3,
            var_2,
        )?;
        object_3.finish();
    }
    if let Some(var_4) = &input.database_preferences {
        #[allow(unused_mut)]
        let mut object_5 = object.key("databasePreferences").start_object();
        crate::protocol_serde::shape_database_preferences::ser_database_preferences(
            &mut object_5,
            var_4,
        )?;
        object_5.finish();
    }
    if let Some(var_6) = &input.prioritize_business_goals {
        #[allow(unused_mut)]
        let mut object_7 = object.key("prioritizeBusinessGoals").start_object();
        crate::protocol_serde::shape_prioritize_business_goals::ser_prioritize_business_goals(
            &mut object_7,
            var_6,
        )?;
        object_7.finish();
    }
    Ok(())
}
