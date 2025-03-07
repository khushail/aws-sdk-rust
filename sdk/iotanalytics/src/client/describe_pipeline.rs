// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribePipeline`](crate::operation::describe_pipeline::builders::DescribePipelineFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`pipeline_name(impl ::std::convert::Into<String>)`](crate::operation::describe_pipeline::builders::DescribePipelineFluentBuilder::pipeline_name) / [`set_pipeline_name(Option<String>)`](crate::operation::describe_pipeline::builders::DescribePipelineFluentBuilder::set_pipeline_name): <p>The name of the pipeline whose information is retrieved.</p>
    /// - On success, responds with [`DescribePipelineOutput`](crate::operation::describe_pipeline::DescribePipelineOutput) with field(s):
    ///   - [`pipeline(Option<Pipeline>)`](crate::operation::describe_pipeline::DescribePipelineOutput::pipeline): <p>A <code>Pipeline</code> object that contains information about the pipeline.</p>
    /// - On failure, responds with [`SdkError<DescribePipelineError>`](crate::operation::describe_pipeline::DescribePipelineError)
    pub fn describe_pipeline(
        &self,
    ) -> crate::operation::describe_pipeline::builders::DescribePipelineFluentBuilder {
        crate::operation::describe_pipeline::builders::DescribePipelineFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
