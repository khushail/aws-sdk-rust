// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateConfigurationSetTrackingOptions`](crate::operation::update_configuration_set_tracking_options::builders::UpdateConfigurationSetTrackingOptionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`configuration_set_name(impl ::std::convert::Into<String>)`](crate::operation::update_configuration_set_tracking_options::builders::UpdateConfigurationSetTrackingOptionsFluentBuilder::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::operation::update_configuration_set_tracking_options::builders::UpdateConfigurationSetTrackingOptionsFluentBuilder::set_configuration_set_name): <p>The name of the configuration set for which you want to update the custom tracking domain.</p>
    ///   - [`tracking_options(TrackingOptions)`](crate::operation::update_configuration_set_tracking_options::builders::UpdateConfigurationSetTrackingOptionsFluentBuilder::tracking_options) / [`set_tracking_options(Option<TrackingOptions>)`](crate::operation::update_configuration_set_tracking_options::builders::UpdateConfigurationSetTrackingOptionsFluentBuilder::set_tracking_options): <p>A domain that is used to redirect email recipients to an Amazon SES-operated domain. This domain captures open and click events generated by Amazon SES emails.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Configuring Custom Domains to Handle Open and Click Tracking</a> in the <i>Amazon SES Developer Guide</i>.</p>
    /// - On success, responds with [`UpdateConfigurationSetTrackingOptionsOutput`](crate::operation::update_configuration_set_tracking_options::UpdateConfigurationSetTrackingOptionsOutput)
    /// - On failure, responds with [`SdkError<UpdateConfigurationSetTrackingOptionsError>`](crate::operation::update_configuration_set_tracking_options::UpdateConfigurationSetTrackingOptionsError)
    pub fn update_configuration_set_tracking_options(&self) -> crate::operation::update_configuration_set_tracking_options::builders::UpdateConfigurationSetTrackingOptionsFluentBuilder{
        crate::operation::update_configuration_set_tracking_options::builders::UpdateConfigurationSetTrackingOptionsFluentBuilder::new(self.handle.clone())
    }
}
