// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The error information.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ErrorDocument {
    /// <p>The object key name to use when a 4XX class error occurs.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    #[doc(hidden)]
    pub key: ::std::option::Option<::std::string::String>,
}
impl ErrorDocument {
    /// <p>The object key name to use when a 4XX class error occurs.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
}
impl ErrorDocument {
    /// Creates a new builder-style object to manufacture [`ErrorDocument`](crate::types::ErrorDocument).
    pub fn builder() -> crate::types::builders::ErrorDocumentBuilder {
        crate::types::builders::ErrorDocumentBuilder::default()
    }
}

/// A builder for [`ErrorDocument`](crate::types::ErrorDocument).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ErrorDocumentBuilder {
    pub(crate) key: ::std::option::Option<::std::string::String>,
}
impl ErrorDocumentBuilder {
    /// <p>The object key name to use when a 4XX class error occurs.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The object key name to use when a 4XX class error occurs.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// Consumes the builder and constructs a [`ErrorDocument`](crate::types::ErrorDocument).
    pub fn build(self) -> crate::types::ErrorDocument {
        crate::types::ErrorDocument { key: self.key }
    }
}
