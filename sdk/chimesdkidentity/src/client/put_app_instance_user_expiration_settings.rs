// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutAppInstanceUserExpirationSettings`](crate::operation::put_app_instance_user_expiration_settings::builders::PutAppInstanceUserExpirationSettingsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_instance_user_arn(impl ::std::convert::Into<String>)`](crate::operation::put_app_instance_user_expiration_settings::builders::PutAppInstanceUserExpirationSettingsFluentBuilder::app_instance_user_arn) / [`set_app_instance_user_arn(Option<String>)`](crate::operation::put_app_instance_user_expiration_settings::builders::PutAppInstanceUserExpirationSettingsFluentBuilder::set_app_instance_user_arn): <p>The ARN of the <code>AppInstanceUser</code>.</p>
    ///   - [`expiration_settings(ExpirationSettings)`](crate::operation::put_app_instance_user_expiration_settings::builders::PutAppInstanceUserExpirationSettingsFluentBuilder::expiration_settings) / [`set_expiration_settings(Option<ExpirationSettings>)`](crate::operation::put_app_instance_user_expiration_settings::builders::PutAppInstanceUserExpirationSettingsFluentBuilder::set_expiration_settings): <p>Settings that control the interval after which an <code>AppInstanceUser</code> is automatically deleted.</p>
    /// - On success, responds with [`PutAppInstanceUserExpirationSettingsOutput`](crate::operation::put_app_instance_user_expiration_settings::PutAppInstanceUserExpirationSettingsOutput) with field(s):
    ///   - [`app_instance_user_arn(Option<String>)`](crate::operation::put_app_instance_user_expiration_settings::PutAppInstanceUserExpirationSettingsOutput::app_instance_user_arn): <p>The ARN of the <code>AppInstanceUser</code>.</p>
    ///   - [`expiration_settings(Option<ExpirationSettings>)`](crate::operation::put_app_instance_user_expiration_settings::PutAppInstanceUserExpirationSettingsOutput::expiration_settings): <p>Settings that control the interval after which an <code>AppInstanceUser</code> is automatically deleted.</p>
    /// - On failure, responds with [`SdkError<PutAppInstanceUserExpirationSettingsError>`](crate::operation::put_app_instance_user_expiration_settings::PutAppInstanceUserExpirationSettingsError)
    pub fn put_app_instance_user_expiration_settings(&self) -> crate::operation::put_app_instance_user_expiration_settings::builders::PutAppInstanceUserExpirationSettingsFluentBuilder{
        crate::operation::put_app_instance_user_expiration_settings::builders::PutAppInstanceUserExpirationSettingsFluentBuilder::new(self.handle.clone())
    }
}
