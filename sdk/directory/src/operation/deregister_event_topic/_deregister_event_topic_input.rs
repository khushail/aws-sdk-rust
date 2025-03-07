// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Removes the specified directory as a publisher to the specified Amazon SNS topic.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeregisterEventTopicInput {
    /// <p>The Directory ID to remove as a publisher. This directory will no longer send messages to the specified Amazon SNS topic.</p>
    #[doc(hidden)]
    pub directory_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the Amazon SNS topic from which to remove the directory as a publisher.</p>
    #[doc(hidden)]
    pub topic_name: ::std::option::Option<::std::string::String>,
}
impl DeregisterEventTopicInput {
    /// <p>The Directory ID to remove as a publisher. This directory will no longer send messages to the specified Amazon SNS topic.</p>
    pub fn directory_id(&self) -> ::std::option::Option<&str> {
        self.directory_id.as_deref()
    }
    /// <p>The name of the Amazon SNS topic from which to remove the directory as a publisher.</p>
    pub fn topic_name(&self) -> ::std::option::Option<&str> {
        self.topic_name.as_deref()
    }
}
impl DeregisterEventTopicInput {
    /// Creates a new builder-style object to manufacture [`DeregisterEventTopicInput`](crate::operation::deregister_event_topic::DeregisterEventTopicInput).
    pub fn builder(
    ) -> crate::operation::deregister_event_topic::builders::DeregisterEventTopicInputBuilder {
        crate::operation::deregister_event_topic::builders::DeregisterEventTopicInputBuilder::default()
    }
}

/// A builder for [`DeregisterEventTopicInput`](crate::operation::deregister_event_topic::DeregisterEventTopicInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeregisterEventTopicInputBuilder {
    pub(crate) directory_id: ::std::option::Option<::std::string::String>,
    pub(crate) topic_name: ::std::option::Option<::std::string::String>,
}
impl DeregisterEventTopicInputBuilder {
    /// <p>The Directory ID to remove as a publisher. This directory will no longer send messages to the specified Amazon SNS topic.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.directory_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Directory ID to remove as a publisher. This directory will no longer send messages to the specified Amazon SNS topic.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.directory_id = input;
        self
    }
    /// <p>The name of the Amazon SNS topic from which to remove the directory as a publisher.</p>
    pub fn topic_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.topic_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon SNS topic from which to remove the directory as a publisher.</p>
    pub fn set_topic_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.topic_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeregisterEventTopicInput`](crate::operation::deregister_event_topic::DeregisterEventTopicInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::deregister_event_topic::DeregisterEventTopicInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::deregister_event_topic::DeregisterEventTopicInput {
                directory_id: self.directory_id,
                topic_name: self.topic_name,
            },
        )
    }
}
