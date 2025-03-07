// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateWorldTemplate`](crate::operation::update_world_template::builders::UpdateWorldTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`template(impl ::std::convert::Into<String>)`](crate::operation::update_world_template::builders::UpdateWorldTemplateFluentBuilder::template) / [`set_template(Option<String>)`](crate::operation::update_world_template::builders::UpdateWorldTemplateFluentBuilder::set_template): <p>The Amazon Resource Name (arn) of the world template to update.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_world_template::builders::UpdateWorldTemplateFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_world_template::builders::UpdateWorldTemplateFluentBuilder::set_name): <p>The name of the template.</p>
    ///   - [`template_body(impl ::std::convert::Into<String>)`](crate::operation::update_world_template::builders::UpdateWorldTemplateFluentBuilder::template_body) / [`set_template_body(Option<String>)`](crate::operation::update_world_template::builders::UpdateWorldTemplateFluentBuilder::set_template_body): <p>The world template body.</p>
    ///   - [`template_location(TemplateLocation)`](crate::operation::update_world_template::builders::UpdateWorldTemplateFluentBuilder::template_location) / [`set_template_location(Option<TemplateLocation>)`](crate::operation::update_world_template::builders::UpdateWorldTemplateFluentBuilder::set_template_location): <p>The location of the world template.</p>
    /// - On success, responds with [`UpdateWorldTemplateOutput`](crate::operation::update_world_template::UpdateWorldTemplateOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::update_world_template::UpdateWorldTemplateOutput::arn): <p>The Amazon Resource Name (arn) of the world template.</p>
    ///   - [`name(Option<String>)`](crate::operation::update_world_template::UpdateWorldTemplateOutput::name): <p>The name of the world template.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::operation::update_world_template::UpdateWorldTemplateOutput::created_at): <p>The time, in milliseconds since the epoch, when the world template was created.</p>
    ///   - [`last_updated_at(Option<DateTime>)`](crate::operation::update_world_template::UpdateWorldTemplateOutput::last_updated_at): <p>The time, in milliseconds since the epoch, when the world template was last updated.</p>
    /// - On failure, responds with [`SdkError<UpdateWorldTemplateError>`](crate::operation::update_world_template::UpdateWorldTemplateError)
    pub fn update_world_template(
        &self,
    ) -> crate::operation::update_world_template::builders::UpdateWorldTemplateFluentBuilder {
        crate::operation::update_world_template::builders::UpdateWorldTemplateFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
