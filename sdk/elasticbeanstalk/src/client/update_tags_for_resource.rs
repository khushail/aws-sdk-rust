// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateTagsForResource`](crate::operation::update_tags_for_resource::builders::UpdateTagsForResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl ::std::convert::Into<String>)`](crate::operation::update_tags_for_resource::builders::UpdateTagsForResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::update_tags_for_resource::builders::UpdateTagsForResourceFluentBuilder::set_resource_arn): <p>The Amazon Resource Name (ARN) of the resouce to be updated.</p>  <p>Must be the ARN of an Elastic Beanstalk resource.</p>
    ///   - [`tags_to_add(Vec<Tag>)`](crate::operation::update_tags_for_resource::builders::UpdateTagsForResourceFluentBuilder::tags_to_add) / [`set_tags_to_add(Option<Vec<Tag>>)`](crate::operation::update_tags_for_resource::builders::UpdateTagsForResourceFluentBuilder::set_tags_to_add): <p>A list of tags to add or update. If a key of an existing tag is added, the tag's value is updated.</p>  <p>Specify at least one of these parameters: <code>TagsToAdd</code>, <code>TagsToRemove</code>.</p>
    ///   - [`tags_to_remove(Vec<String>)`](crate::operation::update_tags_for_resource::builders::UpdateTagsForResourceFluentBuilder::tags_to_remove) / [`set_tags_to_remove(Option<Vec<String>>)`](crate::operation::update_tags_for_resource::builders::UpdateTagsForResourceFluentBuilder::set_tags_to_remove): <p>A list of tag keys to remove. If a tag key doesn't exist, it is silently ignored.</p>  <p>Specify at least one of these parameters: <code>TagsToAdd</code>, <code>TagsToRemove</code>.</p>
    /// - On success, responds with [`UpdateTagsForResourceOutput`](crate::operation::update_tags_for_resource::UpdateTagsForResourceOutput)
    /// - On failure, responds with [`SdkError<UpdateTagsForResourceError>`](crate::operation::update_tags_for_resource::UpdateTagsForResourceError)
    pub fn update_tags_for_resource(
        &self,
    ) -> crate::operation::update_tags_for_resource::builders::UpdateTagsForResourceFluentBuilder
    {
        crate::operation::update_tags_for_resource::builders::UpdateTagsForResourceFluentBuilder::new(self.handle.clone())
    }
}
