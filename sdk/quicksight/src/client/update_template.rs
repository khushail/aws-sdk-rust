// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateTemplate`](crate::operation::update_template::builders::UpdateTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::update_template::builders::UpdateTemplateFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::update_template::builders::UpdateTemplateFluentBuilder::set_aws_account_id): <p>The ID of the Amazon Web Services account that contains the template that you're updating.</p>
    ///   - [`template_id(impl ::std::convert::Into<String>)`](crate::operation::update_template::builders::UpdateTemplateFluentBuilder::template_id) / [`set_template_id(Option<String>)`](crate::operation::update_template::builders::UpdateTemplateFluentBuilder::set_template_id): <p>The ID for the template.</p>
    ///   - [`source_entity(TemplateSourceEntity)`](crate::operation::update_template::builders::UpdateTemplateFluentBuilder::source_entity) / [`set_source_entity(Option<TemplateSourceEntity>)`](crate::operation::update_template::builders::UpdateTemplateFluentBuilder::set_source_entity): <p>The entity that you are using as a source when you update the template. In <code>SourceEntity</code>, you specify the type of object you're using as source: <code>SourceTemplate</code> for a template or <code>SourceAnalysis</code> for an analysis. Both of these require an Amazon Resource Name (ARN). For <code>SourceTemplate</code>, specify the ARN of the source template. For <code>SourceAnalysis</code>, specify the ARN of the source analysis. The <code>SourceTemplate</code> ARN can contain any Amazon Web Services account and any Amazon QuickSight-supported Amazon Web Services Region;. </p>  <p>Use the <code>DataSetReferences</code> entity within <code>SourceTemplate</code> or <code>SourceAnalysis</code> to list the replacement datasets for the placeholders listed in the original. The schema in each dataset must match its placeholder. </p>
    ///   - [`version_description(impl ::std::convert::Into<String>)`](crate::operation::update_template::builders::UpdateTemplateFluentBuilder::version_description) / [`set_version_description(Option<String>)`](crate::operation::update_template::builders::UpdateTemplateFluentBuilder::set_version_description): <p>A description of the current template version that is being updated. Every time you call <code>UpdateTemplate</code>, you create a new version of the template. Each version of the template maintains a description of the version in the <code>VersionDescription</code> field.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_template::builders::UpdateTemplateFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_template::builders::UpdateTemplateFluentBuilder::set_name): <p>The name for the template.</p>
    ///   - [`definition(TemplateVersionDefinition)`](crate::operation::update_template::builders::UpdateTemplateFluentBuilder::definition) / [`set_definition(Option<TemplateVersionDefinition>)`](crate::operation::update_template::builders::UpdateTemplateFluentBuilder::set_definition): <p>The definition of a template.</p>  <p>A definition is the data model of all features in a Dashboard, Template, or Analysis.</p>
    /// - On success, responds with [`UpdateTemplateOutput`](crate::operation::update_template::UpdateTemplateOutput) with field(s):
    ///   - [`template_id(Option<String>)`](crate::operation::update_template::UpdateTemplateOutput::template_id): <p>The ID for the template.</p>
    ///   - [`arn(Option<String>)`](crate::operation::update_template::UpdateTemplateOutput::arn): <p>The Amazon Resource Name (ARN) for the template.</p>
    ///   - [`version_arn(Option<String>)`](crate::operation::update_template::UpdateTemplateOutput::version_arn): <p>The ARN for the template, including the version information of the first version.</p>
    ///   - [`creation_status(Option<ResourceStatus>)`](crate::operation::update_template::UpdateTemplateOutput::creation_status): <p>The creation status of the template.</p>
    ///   - [`status(i32)`](crate::operation::update_template::UpdateTemplateOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::update_template::UpdateTemplateOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    /// - On failure, responds with [`SdkError<UpdateTemplateError>`](crate::operation::update_template::UpdateTemplateError)
    pub fn update_template(
        &self,
    ) -> crate::operation::update_template::builders::UpdateTemplateFluentBuilder {
        crate::operation::update_template::builders::UpdateTemplateFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
