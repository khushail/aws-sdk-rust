// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateKeyspace`](crate::operation::create_keyspace::builders::CreateKeyspaceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`keyspace_name(impl ::std::convert::Into<String>)`](crate::operation::create_keyspace::builders::CreateKeyspaceFluentBuilder::keyspace_name) / [`set_keyspace_name(Option<String>)`](crate::operation::create_keyspace::builders::CreateKeyspaceFluentBuilder::set_keyspace_name): <p>The name of the keyspace to be created.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_keyspace::builders::CreateKeyspaceFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_keyspace::builders::CreateKeyspaceFluentBuilder::set_tags): <p>A list of key-value pair tags to be attached to the keyspace.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/tagging-keyspaces.html">Adding tags and labels to Amazon Keyspaces resources</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    /// - On success, responds with [`CreateKeyspaceOutput`](crate::operation::create_keyspace::CreateKeyspaceOutput) with field(s):
    ///   - [`resource_arn(Option<String>)`](crate::operation::create_keyspace::CreateKeyspaceOutput::resource_arn): <p>The unique identifier of the keyspace in the format of an Amazon Resource Name (ARN).</p>
    /// - On failure, responds with [`SdkError<CreateKeyspaceError>`](crate::operation::create_keyspace::CreateKeyspaceError)
    pub fn create_keyspace(
        &self,
    ) -> crate::operation::create_keyspace::builders::CreateKeyspaceFluentBuilder {
        crate::operation::create_keyspace::builders::CreateKeyspaceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
