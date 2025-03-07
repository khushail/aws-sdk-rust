// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreatePipeline`](crate::operation::create_pipeline::builders::CreatePipelineFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`pipeline_name(impl ::std::convert::Into<String>)`](crate::operation::create_pipeline::builders::CreatePipelineFluentBuilder::pipeline_name) / [`set_pipeline_name(Option<String>)`](crate::operation::create_pipeline::builders::CreatePipelineFluentBuilder::set_pipeline_name): <p>The name of the pipeline.</p>
    ///   - [`pipeline_activities(Vec<PipelineActivity>)`](crate::operation::create_pipeline::builders::CreatePipelineFluentBuilder::pipeline_activities) / [`set_pipeline_activities(Option<Vec<PipelineActivity>>)`](crate::operation::create_pipeline::builders::CreatePipelineFluentBuilder::set_pipeline_activities): <p>A list of <code>PipelineActivity</code> objects. Activities perform transformations on your messages, such as removing, renaming or adding message attributes; filtering messages based on attribute values; invoking your Lambda unctions on messages for advanced processing; or performing mathematical transformations to normalize device data.</p>  <p>The list can be 2-25 <code>PipelineActivity</code> objects and must contain both a <code>channel</code> and a <code>datastore</code> activity. Each entry in the list must contain only one activity. For example:</p>  <p> <code>pipelineActivities = [ { "channel": { ... } }, { "lambda": { ... } }, ... ]</code> </p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_pipeline::builders::CreatePipelineFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_pipeline::builders::CreatePipelineFluentBuilder::set_tags): <p>Metadata which can be used to manage the pipeline.</p>
    /// - On success, responds with [`CreatePipelineOutput`](crate::operation::create_pipeline::CreatePipelineOutput) with field(s):
    ///   - [`pipeline_name(Option<String>)`](crate::operation::create_pipeline::CreatePipelineOutput::pipeline_name): <p>The name of the pipeline.</p>
    ///   - [`pipeline_arn(Option<String>)`](crate::operation::create_pipeline::CreatePipelineOutput::pipeline_arn): <p>The ARN of the pipeline.</p>
    /// - On failure, responds with [`SdkError<CreatePipelineError>`](crate::operation::create_pipeline::CreatePipelineError)
    pub fn create_pipeline(
        &self,
    ) -> crate::operation::create_pipeline::builders::CreatePipelineFluentBuilder {
        crate::operation::create_pipeline::builders::CreatePipelineFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
