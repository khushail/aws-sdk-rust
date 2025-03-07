// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAccessPreview`](crate::operation::get_access_preview::builders::GetAccessPreviewFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`access_preview_id(impl ::std::convert::Into<String>)`](crate::operation::get_access_preview::builders::GetAccessPreviewFluentBuilder::access_preview_id) / [`set_access_preview_id(Option<String>)`](crate::operation::get_access_preview::builders::GetAccessPreviewFluentBuilder::set_access_preview_id): <p>The unique ID for the access preview.</p>
    ///   - [`analyzer_arn(impl ::std::convert::Into<String>)`](crate::operation::get_access_preview::builders::GetAccessPreviewFluentBuilder::analyzer_arn) / [`set_analyzer_arn(Option<String>)`](crate::operation::get_access_preview::builders::GetAccessPreviewFluentBuilder::set_analyzer_arn): <p>The <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-getting-started.html#permission-resources">ARN of the analyzer</a> used to generate the access preview.</p>
    /// - On success, responds with [`GetAccessPreviewOutput`](crate::operation::get_access_preview::GetAccessPreviewOutput) with field(s):
    ///   - [`access_preview(Option<AccessPreview>)`](crate::operation::get_access_preview::GetAccessPreviewOutput::access_preview): <p>An object that contains information about the access preview.</p>
    /// - On failure, responds with [`SdkError<GetAccessPreviewError>`](crate::operation::get_access_preview::GetAccessPreviewError)
    pub fn get_access_preview(
        &self,
    ) -> crate::operation::get_access_preview::builders::GetAccessPreviewFluentBuilder {
        crate::operation::get_access_preview::builders::GetAccessPreviewFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
