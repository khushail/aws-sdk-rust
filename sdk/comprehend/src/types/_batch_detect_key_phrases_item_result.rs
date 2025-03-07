// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The result of calling the operation. The operation returns one object for each document that is successfully processed by the operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchDetectKeyPhrasesItemResult {
    /// <p>The zero-based index of the document in the input list.</p>
    #[doc(hidden)]
    pub index: ::std::option::Option<i32>,
    /// <p>One or more <code>KeyPhrase</code> objects, one for each key phrase detected in the document.</p>
    #[doc(hidden)]
    pub key_phrases: ::std::option::Option<::std::vec::Vec<crate::types::KeyPhrase>>,
}
impl BatchDetectKeyPhrasesItemResult {
    /// <p>The zero-based index of the document in the input list.</p>
    pub fn index(&self) -> ::std::option::Option<i32> {
        self.index
    }
    /// <p>One or more <code>KeyPhrase</code> objects, one for each key phrase detected in the document.</p>
    pub fn key_phrases(&self) -> ::std::option::Option<&[crate::types::KeyPhrase]> {
        self.key_phrases.as_deref()
    }
}
impl BatchDetectKeyPhrasesItemResult {
    /// Creates a new builder-style object to manufacture [`BatchDetectKeyPhrasesItemResult`](crate::types::BatchDetectKeyPhrasesItemResult).
    pub fn builder() -> crate::types::builders::BatchDetectKeyPhrasesItemResultBuilder {
        crate::types::builders::BatchDetectKeyPhrasesItemResultBuilder::default()
    }
}

/// A builder for [`BatchDetectKeyPhrasesItemResult`](crate::types::BatchDetectKeyPhrasesItemResult).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchDetectKeyPhrasesItemResultBuilder {
    pub(crate) index: ::std::option::Option<i32>,
    pub(crate) key_phrases: ::std::option::Option<::std::vec::Vec<crate::types::KeyPhrase>>,
}
impl BatchDetectKeyPhrasesItemResultBuilder {
    /// <p>The zero-based index of the document in the input list.</p>
    pub fn index(mut self, input: i32) -> Self {
        self.index = ::std::option::Option::Some(input);
        self
    }
    /// <p>The zero-based index of the document in the input list.</p>
    pub fn set_index(mut self, input: ::std::option::Option<i32>) -> Self {
        self.index = input;
        self
    }
    /// Appends an item to `key_phrases`.
    ///
    /// To override the contents of this collection use [`set_key_phrases`](Self::set_key_phrases).
    ///
    /// <p>One or more <code>KeyPhrase</code> objects, one for each key phrase detected in the document.</p>
    pub fn key_phrases(mut self, input: crate::types::KeyPhrase) -> Self {
        let mut v = self.key_phrases.unwrap_or_default();
        v.push(input);
        self.key_phrases = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more <code>KeyPhrase</code> objects, one for each key phrase detected in the document.</p>
    pub fn set_key_phrases(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::KeyPhrase>>,
    ) -> Self {
        self.key_phrases = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchDetectKeyPhrasesItemResult`](crate::types::BatchDetectKeyPhrasesItemResult).
    pub fn build(self) -> crate::types::BatchDetectKeyPhrasesItemResult {
        crate::types::BatchDetectKeyPhrasesItemResult {
            index: self.index,
            key_phrases: self.key_phrases,
        }
    }
}
