// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UntagResource`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl ::std::convert::Into<String>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::set_resource_arn): <p>The Amazon Resource Name (ARN) of the resource from which to remove tags.</p>
    ///   - [`tag_keys(Vec<String>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::set_tag_keys): <p>The list of tag keys specifying which tags to remove.</p>
    /// - On success, responds with [`UntagResourceOutput`](crate::operation::untag_resource::UntagResourceOutput) with field(s):
    ///   - [`resource_arn(Option<String>)`](crate::operation::untag_resource::UntagResourceOutput::resource_arn): <p>The Amazon Resource Name (ARN) of the resource from which you removed tags.</p>
    /// - On failure, responds with [`SdkError<UntagResourceError>`](crate::operation::untag_resource::UntagResourceError)
    pub fn untag_resource(
        &self,
    ) -> crate::operation::untag_resource::builders::UntagResourceFluentBuilder {
        crate::operation::untag_resource::builders::UntagResourceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
