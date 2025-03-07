// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateServiceSettings`](crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`s3_bucket_arn(impl ::std::convert::Into<String>)`](crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder::s3_bucket_arn) / [`set_s3_bucket_arn(Option<String>)`](crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder::set_s3_bucket_arn): <p>Amazon Resource Name (ARN) of the Amazon S3 bucket where the License Manager information is stored.</p>
    ///   - [`sns_topic_arn(impl ::std::convert::Into<String>)`](crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder::sns_topic_arn) / [`set_sns_topic_arn(Option<String>)`](crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder::set_sns_topic_arn): <p>Amazon Resource Name (ARN) of the Amazon SNS topic used for License Manager alerts.</p>
    ///   - [`organization_configuration(OrganizationConfiguration)`](crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder::organization_configuration) / [`set_organization_configuration(Option<OrganizationConfiguration>)`](crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder::set_organization_configuration): <p>Enables integration with Organizations for cross-account discovery.</p>
    ///   - [`enable_cross_accounts_discovery(bool)`](crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder::enable_cross_accounts_discovery) / [`set_enable_cross_accounts_discovery(Option<bool>)`](crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder::set_enable_cross_accounts_discovery): <p>Activates cross-account discovery.</p>
    /// - On success, responds with [`UpdateServiceSettingsOutput`](crate::operation::update_service_settings::UpdateServiceSettingsOutput)
    /// - On failure, responds with [`SdkError<UpdateServiceSettingsError>`](crate::operation::update_service_settings::UpdateServiceSettingsError)
    pub fn update_service_settings(
        &self,
    ) -> crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder
    {
        crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
