// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct DetectSentimentInput {
    /// <p>A UTF-8 text string. The maximum string size is 5 KB.</p>
    #[doc(hidden)]
    pub text: ::std::option::Option<::std::string::String>,
    /// <p>The language of the input documents. You can specify any of the primary languages supported by Amazon Comprehend. All documents must be in the same language.</p>
    #[doc(hidden)]
    pub language_code: ::std::option::Option<crate::types::LanguageCode>,
}
impl DetectSentimentInput {
    /// <p>A UTF-8 text string. The maximum string size is 5 KB.</p>
    pub fn text(&self) -> ::std::option::Option<&str> {
        self.text.as_deref()
    }
    /// <p>The language of the input documents. You can specify any of the primary languages supported by Amazon Comprehend. All documents must be in the same language.</p>
    pub fn language_code(&self) -> ::std::option::Option<&crate::types::LanguageCode> {
        self.language_code.as_ref()
    }
}
impl ::std::fmt::Debug for DetectSentimentInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DetectSentimentInput");
        formatter.field("text", &"*** Sensitive Data Redacted ***");
        formatter.field("language_code", &self.language_code);
        formatter.finish()
    }
}
impl DetectSentimentInput {
    /// Creates a new builder-style object to manufacture [`DetectSentimentInput`](crate::operation::detect_sentiment::DetectSentimentInput).
    pub fn builder() -> crate::operation::detect_sentiment::builders::DetectSentimentInputBuilder {
        crate::operation::detect_sentiment::builders::DetectSentimentInputBuilder::default()
    }
}

/// A builder for [`DetectSentimentInput`](crate::operation::detect_sentiment::DetectSentimentInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct DetectSentimentInputBuilder {
    pub(crate) text: ::std::option::Option<::std::string::String>,
    pub(crate) language_code: ::std::option::Option<crate::types::LanguageCode>,
}
impl DetectSentimentInputBuilder {
    /// <p>A UTF-8 text string. The maximum string size is 5 KB.</p>
    pub fn text(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.text = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A UTF-8 text string. The maximum string size is 5 KB.</p>
    pub fn set_text(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.text = input;
        self
    }
    /// <p>The language of the input documents. You can specify any of the primary languages supported by Amazon Comprehend. All documents must be in the same language.</p>
    pub fn language_code(mut self, input: crate::types::LanguageCode) -> Self {
        self.language_code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The language of the input documents. You can specify any of the primary languages supported by Amazon Comprehend. All documents must be in the same language.</p>
    pub fn set_language_code(
        mut self,
        input: ::std::option::Option<crate::types::LanguageCode>,
    ) -> Self {
        self.language_code = input;
        self
    }
    /// Consumes the builder and constructs a [`DetectSentimentInput`](crate::operation::detect_sentiment::DetectSentimentInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::detect_sentiment::DetectSentimentInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::detect_sentiment::DetectSentimentInput {
            text: self.text,
            language_code: self.language_code,
        })
    }
}
impl ::std::fmt::Debug for DetectSentimentInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DetectSentimentInputBuilder");
        formatter.field("text", &"*** Sensitive Data Redacted ***");
        formatter.field("language_code", &self.language_code);
        formatter.finish()
    }
}
