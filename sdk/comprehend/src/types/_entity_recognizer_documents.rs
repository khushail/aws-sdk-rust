// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the training documents submitted with an entity recognizer.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EntityRecognizerDocuments {
    /// <p> Specifies the Amazon S3 location where the training documents for an entity recognizer are located. The URI must be in the same Region as the API endpoint that you are calling.</p>
    #[doc(hidden)]
    pub s3_uri: ::std::option::Option<::std::string::String>,
    /// <p> Specifies the Amazon S3 location where the test documents for an entity recognizer are located. The URI must be in the same Amazon Web Services Region as the API endpoint that you are calling.</p>
    #[doc(hidden)]
    pub test_s3_uri: ::std::option::Option<::std::string::String>,
    /// <p> Specifies how the text in an input file should be processed. This is optional, and the default is ONE_DOC_PER_LINE. ONE_DOC_PER_FILE - Each file is considered a separate document. Use this option when you are processing large documents, such as newspaper articles or scientific papers. ONE_DOC_PER_LINE - Each line in a file is considered a separate document. Use this option when you are processing many short documents, such as text messages.</p>
    #[doc(hidden)]
    pub input_format: ::std::option::Option<crate::types::InputFormat>,
}
impl EntityRecognizerDocuments {
    /// <p> Specifies the Amazon S3 location where the training documents for an entity recognizer are located. The URI must be in the same Region as the API endpoint that you are calling.</p>
    pub fn s3_uri(&self) -> ::std::option::Option<&str> {
        self.s3_uri.as_deref()
    }
    /// <p> Specifies the Amazon S3 location where the test documents for an entity recognizer are located. The URI must be in the same Amazon Web Services Region as the API endpoint that you are calling.</p>
    pub fn test_s3_uri(&self) -> ::std::option::Option<&str> {
        self.test_s3_uri.as_deref()
    }
    /// <p> Specifies how the text in an input file should be processed. This is optional, and the default is ONE_DOC_PER_LINE. ONE_DOC_PER_FILE - Each file is considered a separate document. Use this option when you are processing large documents, such as newspaper articles or scientific papers. ONE_DOC_PER_LINE - Each line in a file is considered a separate document. Use this option when you are processing many short documents, such as text messages.</p>
    pub fn input_format(&self) -> ::std::option::Option<&crate::types::InputFormat> {
        self.input_format.as_ref()
    }
}
impl EntityRecognizerDocuments {
    /// Creates a new builder-style object to manufacture [`EntityRecognizerDocuments`](crate::types::EntityRecognizerDocuments).
    pub fn builder() -> crate::types::builders::EntityRecognizerDocumentsBuilder {
        crate::types::builders::EntityRecognizerDocumentsBuilder::default()
    }
}

/// A builder for [`EntityRecognizerDocuments`](crate::types::EntityRecognizerDocuments).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EntityRecognizerDocumentsBuilder {
    pub(crate) s3_uri: ::std::option::Option<::std::string::String>,
    pub(crate) test_s3_uri: ::std::option::Option<::std::string::String>,
    pub(crate) input_format: ::std::option::Option<crate::types::InputFormat>,
}
impl EntityRecognizerDocumentsBuilder {
    /// <p> Specifies the Amazon S3 location where the training documents for an entity recognizer are located. The URI must be in the same Region as the API endpoint that you are calling.</p>
    pub fn s3_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.s3_uri = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> Specifies the Amazon S3 location where the training documents for an entity recognizer are located. The URI must be in the same Region as the API endpoint that you are calling.</p>
    pub fn set_s3_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.s3_uri = input;
        self
    }
    /// <p> Specifies the Amazon S3 location where the test documents for an entity recognizer are located. The URI must be in the same Amazon Web Services Region as the API endpoint that you are calling.</p>
    pub fn test_s3_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.test_s3_uri = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> Specifies the Amazon S3 location where the test documents for an entity recognizer are located. The URI must be in the same Amazon Web Services Region as the API endpoint that you are calling.</p>
    pub fn set_test_s3_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.test_s3_uri = input;
        self
    }
    /// <p> Specifies how the text in an input file should be processed. This is optional, and the default is ONE_DOC_PER_LINE. ONE_DOC_PER_FILE - Each file is considered a separate document. Use this option when you are processing large documents, such as newspaper articles or scientific papers. ONE_DOC_PER_LINE - Each line in a file is considered a separate document. Use this option when you are processing many short documents, such as text messages.</p>
    pub fn input_format(mut self, input: crate::types::InputFormat) -> Self {
        self.input_format = ::std::option::Option::Some(input);
        self
    }
    /// <p> Specifies how the text in an input file should be processed. This is optional, and the default is ONE_DOC_PER_LINE. ONE_DOC_PER_FILE - Each file is considered a separate document. Use this option when you are processing large documents, such as newspaper articles or scientific papers. ONE_DOC_PER_LINE - Each line in a file is considered a separate document. Use this option when you are processing many short documents, such as text messages.</p>
    pub fn set_input_format(
        mut self,
        input: ::std::option::Option<crate::types::InputFormat>,
    ) -> Self {
        self.input_format = input;
        self
    }
    /// Consumes the builder and constructs a [`EntityRecognizerDocuments`](crate::types::EntityRecognizerDocuments).
    pub fn build(self) -> crate::types::EntityRecognizerDocuments {
        crate::types::EntityRecognizerDocuments {
            s3_uri: self.s3_uri,
            test_s3_uri: self.test_s3_uri,
            input_format: self.input_format,
        }
    }
}
