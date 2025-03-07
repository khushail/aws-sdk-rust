// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteLaunchConfigurationTemplate`](crate::operation::delete_launch_configuration_template::builders::DeleteLaunchConfigurationTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`launch_configuration_template_id(impl ::std::convert::Into<String>)`](crate::operation::delete_launch_configuration_template::builders::DeleteLaunchConfigurationTemplateFluentBuilder::launch_configuration_template_id) / [`set_launch_configuration_template_id(Option<String>)`](crate::operation::delete_launch_configuration_template::builders::DeleteLaunchConfigurationTemplateFluentBuilder::set_launch_configuration_template_id): <p>The ID of the Launch Configuration Template to be deleted.</p>
    /// - On success, responds with [`DeleteLaunchConfigurationTemplateOutput`](crate::operation::delete_launch_configuration_template::DeleteLaunchConfigurationTemplateOutput)
    /// - On failure, responds with [`SdkError<DeleteLaunchConfigurationTemplateError>`](crate::operation::delete_launch_configuration_template::DeleteLaunchConfigurationTemplateError)
    pub fn delete_launch_configuration_template(&self) -> crate::operation::delete_launch_configuration_template::builders::DeleteLaunchConfigurationTemplateFluentBuilder{
        crate::operation::delete_launch_configuration_template::builders::DeleteLaunchConfigurationTemplateFluentBuilder::new(self.handle.clone())
    }
}
