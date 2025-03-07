// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCustomPlugin`](crate::operation::delete_custom_plugin::builders::DeleteCustomPluginFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`custom_plugin_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_custom_plugin::builders::DeleteCustomPluginFluentBuilder::custom_plugin_arn) / [`set_custom_plugin_arn(Option<String>)`](crate::operation::delete_custom_plugin::builders::DeleteCustomPluginFluentBuilder::set_custom_plugin_arn): <p>The Amazon Resource Name (ARN) of the custom plugin that you want to delete.</p>
    /// - On success, responds with [`DeleteCustomPluginOutput`](crate::operation::delete_custom_plugin::DeleteCustomPluginOutput) with field(s):
    ///   - [`custom_plugin_arn(Option<String>)`](crate::operation::delete_custom_plugin::DeleteCustomPluginOutput::custom_plugin_arn): <p>The Amazon Resource Name (ARN) of the custom plugin that you requested to delete.</p>
    ///   - [`custom_plugin_state(Option<CustomPluginState>)`](crate::operation::delete_custom_plugin::DeleteCustomPluginOutput::custom_plugin_state): <p>The state of the custom plugin.</p>
    /// - On failure, responds with [`SdkError<DeleteCustomPluginError>`](crate::operation::delete_custom_plugin::DeleteCustomPluginError)
    pub fn delete_custom_plugin(
        &self,
    ) -> crate::operation::delete_custom_plugin::builders::DeleteCustomPluginFluentBuilder {
        crate::operation::delete_custom_plugin::builders::DeleteCustomPluginFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
