// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_subscription_notification_configuration_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_subscription_notification_configuration::CreateSubscriptionNotificationConfigurationInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.create_sqs {
        object.key("createSqs").boolean(*var_1);
    }
    if let Some(var_2) = &input.https_api_key_name {
        object.key("httpsApiKeyName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.https_api_key_value {
        object.key("httpsApiKeyValue").string(var_3.as_str());
    }
    if let Some(var_4) = &input.https_method {
        object.key("httpsMethod").string(var_4.as_str());
    }
    if let Some(var_5) = &input.role_arn {
        object.key("roleArn").string(var_5.as_str());
    }
    if let Some(var_6) = &input.subscription_endpoint {
        object.key("subscriptionEndpoint").string(var_6.as_str());
    }
    Ok(())
}
