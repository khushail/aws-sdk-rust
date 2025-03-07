// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTagsForResources`](crate::operation::list_tags_for_resources::builders::ListTagsForResourcesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl ::std::convert::Into<String>)`](crate::operation::list_tags_for_resources::builders::ListTagsForResourcesFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::list_tags_for_resources::builders::ListTagsForResourcesFluentBuilder::set_resource_arn): <p>The Amazon Resource Name (ARN) for a resource.</p>
    /// - On success, responds with [`ListTagsForResourcesOutput`](crate::operation::list_tags_for_resources::ListTagsForResourcesOutput) with field(s):
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::list_tags_for_resources::ListTagsForResourcesOutput::tags): <p></p>
    /// - On failure, responds with [`SdkError<ListTagsForResourcesError>`](crate::operation::list_tags_for_resources::ListTagsForResourcesError)
    pub fn list_tags_for_resources(
        &self,
    ) -> crate::operation::list_tags_for_resources::builders::ListTagsForResourcesFluentBuilder
    {
        crate::operation::list_tags_for_resources::builders::ListTagsForResourcesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
