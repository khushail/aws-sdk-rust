// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetUserAccessLoggingSettings`](crate::operation::get_user_access_logging_settings::builders::GetUserAccessLoggingSettingsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_access_logging_settings_arn(impl ::std::convert::Into<String>)`](crate::operation::get_user_access_logging_settings::builders::GetUserAccessLoggingSettingsFluentBuilder::user_access_logging_settings_arn) / [`set_user_access_logging_settings_arn(Option<String>)`](crate::operation::get_user_access_logging_settings::builders::GetUserAccessLoggingSettingsFluentBuilder::set_user_access_logging_settings_arn): <p>The ARN of the user access logging settings.</p>
    /// - On success, responds with [`GetUserAccessLoggingSettingsOutput`](crate::operation::get_user_access_logging_settings::GetUserAccessLoggingSettingsOutput) with field(s):
    ///   - [`user_access_logging_settings(Option<UserAccessLoggingSettings>)`](crate::operation::get_user_access_logging_settings::GetUserAccessLoggingSettingsOutput::user_access_logging_settings): <p>The user access logging settings.</p>
    /// - On failure, responds with [`SdkError<GetUserAccessLoggingSettingsError>`](crate::operation::get_user_access_logging_settings::GetUserAccessLoggingSettingsError)
    pub fn get_user_access_logging_settings(&self) -> crate::operation::get_user_access_logging_settings::builders::GetUserAccessLoggingSettingsFluentBuilder{
        crate::operation::get_user_access_logging_settings::builders::GetUserAccessLoggingSettingsFluentBuilder::new(self.handle.clone())
    }
}
