// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchCreateCustomVocabularyItemInput {
    /// <p>The identifier of the bot associated with this custom vocabulary.</p>
    #[doc(hidden)]
    pub bot_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the version of the bot associated with this custom vocabulary.</p>
    #[doc(hidden)]
    pub bot_version: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the language and locale where this custom vocabulary is used. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html"> Supported Languages </a>.</p>
    #[doc(hidden)]
    pub locale_id: ::std::option::Option<::std::string::String>,
    /// <p>A list of new custom vocabulary items. Each entry must contain a phrase and can optionally contain a displayAs and/or a weight.</p>
    #[doc(hidden)]
    pub custom_vocabulary_item_list:
        ::std::option::Option<::std::vec::Vec<crate::types::NewCustomVocabularyItem>>,
}
impl BatchCreateCustomVocabularyItemInput {
    /// <p>The identifier of the bot associated with this custom vocabulary.</p>
    pub fn bot_id(&self) -> ::std::option::Option<&str> {
        self.bot_id.as_deref()
    }
    /// <p>The identifier of the version of the bot associated with this custom vocabulary.</p>
    pub fn bot_version(&self) -> ::std::option::Option<&str> {
        self.bot_version.as_deref()
    }
    /// <p>The identifier of the language and locale where this custom vocabulary is used. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html"> Supported Languages </a>.</p>
    pub fn locale_id(&self) -> ::std::option::Option<&str> {
        self.locale_id.as_deref()
    }
    /// <p>A list of new custom vocabulary items. Each entry must contain a phrase and can optionally contain a displayAs and/or a weight.</p>
    pub fn custom_vocabulary_item_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::NewCustomVocabularyItem]> {
        self.custom_vocabulary_item_list.as_deref()
    }
}
impl BatchCreateCustomVocabularyItemInput {
    /// Creates a new builder-style object to manufacture [`BatchCreateCustomVocabularyItemInput`](crate::operation::batch_create_custom_vocabulary_item::BatchCreateCustomVocabularyItemInput).
    pub fn builder() -> crate::operation::batch_create_custom_vocabulary_item::builders::BatchCreateCustomVocabularyItemInputBuilder{
        crate::operation::batch_create_custom_vocabulary_item::builders::BatchCreateCustomVocabularyItemInputBuilder::default()
    }
}

/// A builder for [`BatchCreateCustomVocabularyItemInput`](crate::operation::batch_create_custom_vocabulary_item::BatchCreateCustomVocabularyItemInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchCreateCustomVocabularyItemInputBuilder {
    pub(crate) bot_id: ::std::option::Option<::std::string::String>,
    pub(crate) bot_version: ::std::option::Option<::std::string::String>,
    pub(crate) locale_id: ::std::option::Option<::std::string::String>,
    pub(crate) custom_vocabulary_item_list:
        ::std::option::Option<::std::vec::Vec<crate::types::NewCustomVocabularyItem>>,
}
impl BatchCreateCustomVocabularyItemInputBuilder {
    /// <p>The identifier of the bot associated with this custom vocabulary.</p>
    pub fn bot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bot_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the bot associated with this custom vocabulary.</p>
    pub fn set_bot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bot_id = input;
        self
    }
    /// <p>The identifier of the version of the bot associated with this custom vocabulary.</p>
    pub fn bot_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bot_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the version of the bot associated with this custom vocabulary.</p>
    pub fn set_bot_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bot_version = input;
        self
    }
    /// <p>The identifier of the language and locale where this custom vocabulary is used. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html"> Supported Languages </a>.</p>
    pub fn locale_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.locale_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the language and locale where this custom vocabulary is used. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html"> Supported Languages </a>.</p>
    pub fn set_locale_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.locale_id = input;
        self
    }
    /// Appends an item to `custom_vocabulary_item_list`.
    ///
    /// To override the contents of this collection use [`set_custom_vocabulary_item_list`](Self::set_custom_vocabulary_item_list).
    ///
    /// <p>A list of new custom vocabulary items. Each entry must contain a phrase and can optionally contain a displayAs and/or a weight.</p>
    pub fn custom_vocabulary_item_list(
        mut self,
        input: crate::types::NewCustomVocabularyItem,
    ) -> Self {
        let mut v = self.custom_vocabulary_item_list.unwrap_or_default();
        v.push(input);
        self.custom_vocabulary_item_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of new custom vocabulary items. Each entry must contain a phrase and can optionally contain a displayAs and/or a weight.</p>
    pub fn set_custom_vocabulary_item_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::NewCustomVocabularyItem>>,
    ) -> Self {
        self.custom_vocabulary_item_list = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchCreateCustomVocabularyItemInput`](crate::operation::batch_create_custom_vocabulary_item::BatchCreateCustomVocabularyItemInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_create_custom_vocabulary_item::BatchCreateCustomVocabularyItemInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::batch_create_custom_vocabulary_item::BatchCreateCustomVocabularyItemInput {
                bot_id: self.bot_id
                ,
                bot_version: self.bot_version
                ,
                locale_id: self.locale_id
                ,
                custom_vocabulary_item_list: self.custom_vocabulary_item_list
                ,
            }
        )
    }
}
