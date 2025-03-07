// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides information about documents that could not be removed from an index by the <code>BatchDeleteDocument</code> API.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchDeleteDocumentResponseFailedDocument {
    /// <p>The identifier of the document that couldn't be removed from the index.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The error code for why the document couldn't be removed from the index.</p>
    #[doc(hidden)]
    pub error_code: ::std::option::Option<crate::types::ErrorCode>,
    /// <p>An explanation for why the document couldn't be removed from the index.</p>
    #[doc(hidden)]
    pub error_message: ::std::option::Option<::std::string::String>,
}
impl BatchDeleteDocumentResponseFailedDocument {
    /// <p>The identifier of the document that couldn't be removed from the index.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The error code for why the document couldn't be removed from the index.</p>
    pub fn error_code(&self) -> ::std::option::Option<&crate::types::ErrorCode> {
        self.error_code.as_ref()
    }
    /// <p>An explanation for why the document couldn't be removed from the index.</p>
    pub fn error_message(&self) -> ::std::option::Option<&str> {
        self.error_message.as_deref()
    }
}
impl BatchDeleteDocumentResponseFailedDocument {
    /// Creates a new builder-style object to manufacture [`BatchDeleteDocumentResponseFailedDocument`](crate::types::BatchDeleteDocumentResponseFailedDocument).
    pub fn builder() -> crate::types::builders::BatchDeleteDocumentResponseFailedDocumentBuilder {
        crate::types::builders::BatchDeleteDocumentResponseFailedDocumentBuilder::default()
    }
}

/// A builder for [`BatchDeleteDocumentResponseFailedDocument`](crate::types::BatchDeleteDocumentResponseFailedDocument).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchDeleteDocumentResponseFailedDocumentBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) error_code: ::std::option::Option<crate::types::ErrorCode>,
    pub(crate) error_message: ::std::option::Option<::std::string::String>,
}
impl BatchDeleteDocumentResponseFailedDocumentBuilder {
    /// <p>The identifier of the document that couldn't be removed from the index.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the document that couldn't be removed from the index.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The error code for why the document couldn't be removed from the index.</p>
    pub fn error_code(mut self, input: crate::types::ErrorCode) -> Self {
        self.error_code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The error code for why the document couldn't be removed from the index.</p>
    pub fn set_error_code(mut self, input: ::std::option::Option<crate::types::ErrorCode>) -> Self {
        self.error_code = input;
        self
    }
    /// <p>An explanation for why the document couldn't be removed from the index.</p>
    pub fn error_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.error_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An explanation for why the document couldn't be removed from the index.</p>
    pub fn set_error_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.error_message = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchDeleteDocumentResponseFailedDocument`](crate::types::BatchDeleteDocumentResponseFailedDocument).
    pub fn build(self) -> crate::types::BatchDeleteDocumentResponseFailedDocument {
        crate::types::BatchDeleteDocumentResponseFailedDocument {
            id: self.id,
            error_code: self.error_code,
            error_message: self.error_message,
        }
    }
}
