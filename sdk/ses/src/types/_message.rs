// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the message to be sent, composed of a subject and a body.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Message {
    /// <p>The subject of the message: A short summary of the content, which will appear in the recipient's inbox.</p>
    #[doc(hidden)]
    pub subject: ::std::option::Option<crate::types::Content>,
    /// <p>The message body.</p>
    #[doc(hidden)]
    pub body: ::std::option::Option<crate::types::Body>,
}
impl Message {
    /// <p>The subject of the message: A short summary of the content, which will appear in the recipient's inbox.</p>
    pub fn subject(&self) -> ::std::option::Option<&crate::types::Content> {
        self.subject.as_ref()
    }
    /// <p>The message body.</p>
    pub fn body(&self) -> ::std::option::Option<&crate::types::Body> {
        self.body.as_ref()
    }
}
impl Message {
    /// Creates a new builder-style object to manufacture [`Message`](crate::types::Message).
    pub fn builder() -> crate::types::builders::MessageBuilder {
        crate::types::builders::MessageBuilder::default()
    }
}

/// A builder for [`Message`](crate::types::Message).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MessageBuilder {
    pub(crate) subject: ::std::option::Option<crate::types::Content>,
    pub(crate) body: ::std::option::Option<crate::types::Body>,
}
impl MessageBuilder {
    /// <p>The subject of the message: A short summary of the content, which will appear in the recipient's inbox.</p>
    pub fn subject(mut self, input: crate::types::Content) -> Self {
        self.subject = ::std::option::Option::Some(input);
        self
    }
    /// <p>The subject of the message: A short summary of the content, which will appear in the recipient's inbox.</p>
    pub fn set_subject(mut self, input: ::std::option::Option<crate::types::Content>) -> Self {
        self.subject = input;
        self
    }
    /// <p>The message body.</p>
    pub fn body(mut self, input: crate::types::Body) -> Self {
        self.body = ::std::option::Option::Some(input);
        self
    }
    /// <p>The message body.</p>
    pub fn set_body(mut self, input: ::std::option::Option<crate::types::Body>) -> Self {
        self.body = input;
        self
    }
    /// Consumes the builder and constructs a [`Message`](crate::types::Message).
    pub fn build(self) -> crate::types::Message {
        crate::types::Message {
            subject: self.subject,
            body: self.body,
        }
    }
}
