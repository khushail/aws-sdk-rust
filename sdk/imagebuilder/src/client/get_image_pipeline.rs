// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetImagePipeline`](crate::operation::get_image_pipeline::builders::GetImagePipelineFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`image_pipeline_arn(impl ::std::convert::Into<String>)`](crate::operation::get_image_pipeline::builders::GetImagePipelineFluentBuilder::image_pipeline_arn) / [`set_image_pipeline_arn(Option<String>)`](crate::operation::get_image_pipeline::builders::GetImagePipelineFluentBuilder::set_image_pipeline_arn): <p>The Amazon Resource Name (ARN) of the image pipeline that you want to retrieve.</p>
    /// - On success, responds with [`GetImagePipelineOutput`](crate::operation::get_image_pipeline::GetImagePipelineOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::get_image_pipeline::GetImagePipelineOutput::request_id): <p>The request ID that uniquely identifies this request.</p>
    ///   - [`image_pipeline(Option<ImagePipeline>)`](crate::operation::get_image_pipeline::GetImagePipelineOutput::image_pipeline): <p>The image pipeline object.</p>
    /// - On failure, responds with [`SdkError<GetImagePipelineError>`](crate::operation::get_image_pipeline::GetImagePipelineError)
    pub fn get_image_pipeline(
        &self,
    ) -> crate::operation::get_image_pipeline::builders::GetImagePipelineFluentBuilder {
        crate::operation::get_image_pipeline::builders::GetImagePipelineFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
