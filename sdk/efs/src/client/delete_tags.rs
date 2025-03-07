// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteTags`](crate::operation::delete_tags::builders::DeleteTagsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`file_system_id(impl ::std::convert::Into<String>)`](crate::operation::delete_tags::builders::DeleteTagsFluentBuilder::file_system_id) / [`set_file_system_id(Option<String>)`](crate::operation::delete_tags::builders::DeleteTagsFluentBuilder::set_file_system_id): <p>The ID of the file system whose tags you want to delete (String).</p>
    ///   - [`tag_keys(Vec<String>)`](crate::operation::delete_tags::builders::DeleteTagsFluentBuilder::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::operation::delete_tags::builders::DeleteTagsFluentBuilder::set_tag_keys): <p>A list of tag keys to delete.</p>
    /// - On success, responds with [`DeleteTagsOutput`](crate::operation::delete_tags::DeleteTagsOutput)
    /// - On failure, responds with [`SdkError<DeleteTagsError>`](crate::operation::delete_tags::DeleteTagsError)
    #[deprecated(note = "Use UntagResource.")]
    pub fn delete_tags(&self) -> crate::operation::delete_tags::builders::DeleteTagsFluentBuilder {
        crate::operation::delete_tags::builders::DeleteTagsFluentBuilder::new(self.handle.clone())
    }
}
