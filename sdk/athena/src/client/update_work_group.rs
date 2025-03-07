// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateWorkGroup`](crate::operation::update_work_group::builders::UpdateWorkGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`work_group(impl ::std::convert::Into<String>)`](crate::operation::update_work_group::builders::UpdateWorkGroupFluentBuilder::work_group) / [`set_work_group(Option<String>)`](crate::operation::update_work_group::builders::UpdateWorkGroupFluentBuilder::set_work_group): <p>The specified workgroup that will be updated.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_work_group::builders::UpdateWorkGroupFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_work_group::builders::UpdateWorkGroupFluentBuilder::set_description): <p>The workgroup description.</p>
    ///   - [`configuration_updates(WorkGroupConfigurationUpdates)`](crate::operation::update_work_group::builders::UpdateWorkGroupFluentBuilder::configuration_updates) / [`set_configuration_updates(Option<WorkGroupConfigurationUpdates>)`](crate::operation::update_work_group::builders::UpdateWorkGroupFluentBuilder::set_configuration_updates): <p>Contains configuration updates for an Athena SQL workgroup.</p>
    ///   - [`state(WorkGroupState)`](crate::operation::update_work_group::builders::UpdateWorkGroupFluentBuilder::state) / [`set_state(Option<WorkGroupState>)`](crate::operation::update_work_group::builders::UpdateWorkGroupFluentBuilder::set_state): <p>The workgroup state that will be updated for the given workgroup.</p>
    /// - On success, responds with [`UpdateWorkGroupOutput`](crate::operation::update_work_group::UpdateWorkGroupOutput)
    /// - On failure, responds with [`SdkError<UpdateWorkGroupError>`](crate::operation::update_work_group::UpdateWorkGroupError)
    pub fn update_work_group(
        &self,
    ) -> crate::operation::update_work_group::builders::UpdateWorkGroupFluentBuilder {
        crate::operation::update_work_group::builders::UpdateWorkGroupFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
