// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides the bot parameters required for importing a bot.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BotImportSpecification {
    /// <p>The name that Amazon Lex should use for the bot.</p>
    #[doc(hidden)]
    pub bot_name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role used to build and run the bot.</p>
    #[doc(hidden)]
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>By default, data stored by Amazon Lex is encrypted. The <code>DataPrivacy</code> structure provides settings that determine how Amazon Lex handles special cases of securing the data for your bot. </p>
    #[doc(hidden)]
    pub data_privacy: ::std::option::Option<crate::types::DataPrivacy>,
    /// <p>The time, in seconds, that Amazon Lex should keep information about a user's conversation with the bot. </p>
    /// <p>A user interaction remains active for the amount of time specified. If no conversation occurs during this time, the session expires and Amazon Lex deletes any data provided before the timeout.</p>
    /// <p>You can specify between 60 (1 minute) and 86,400 (24 hours) seconds.</p>
    #[doc(hidden)]
    pub idle_session_ttl_in_seconds: ::std::option::Option<i32>,
    /// <p>A list of tags to add to the bot. You can only add tags when you import a bot. You can't use the <code>UpdateBot</code> operation to update tags. To update tags, use the <code>TagResource</code> operation.</p>
    #[doc(hidden)]
    pub bot_tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p>A list of tags to add to the test alias for a bot. You can only add tags when you import a bot. You can't use the <code>UpdateAlias</code> operation to update tags. To update tags on the test alias, use the <code>TagResource</code> operation.</p>
    #[doc(hidden)]
    pub test_bot_alias_tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl BotImportSpecification {
    /// <p>The name that Amazon Lex should use for the bot.</p>
    pub fn bot_name(&self) -> ::std::option::Option<&str> {
        self.bot_name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role used to build and run the bot.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>By default, data stored by Amazon Lex is encrypted. The <code>DataPrivacy</code> structure provides settings that determine how Amazon Lex handles special cases of securing the data for your bot. </p>
    pub fn data_privacy(&self) -> ::std::option::Option<&crate::types::DataPrivacy> {
        self.data_privacy.as_ref()
    }
    /// <p>The time, in seconds, that Amazon Lex should keep information about a user's conversation with the bot. </p>
    /// <p>A user interaction remains active for the amount of time specified. If no conversation occurs during this time, the session expires and Amazon Lex deletes any data provided before the timeout.</p>
    /// <p>You can specify between 60 (1 minute) and 86,400 (24 hours) seconds.</p>
    pub fn idle_session_ttl_in_seconds(&self) -> ::std::option::Option<i32> {
        self.idle_session_ttl_in_seconds
    }
    /// <p>A list of tags to add to the bot. You can only add tags when you import a bot. You can't use the <code>UpdateBot</code> operation to update tags. To update tags, use the <code>TagResource</code> operation.</p>
    pub fn bot_tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.bot_tags.as_ref()
    }
    /// <p>A list of tags to add to the test alias for a bot. You can only add tags when you import a bot. You can't use the <code>UpdateAlias</code> operation to update tags. To update tags on the test alias, use the <code>TagResource</code> operation.</p>
    pub fn test_bot_alias_tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.test_bot_alias_tags.as_ref()
    }
}
impl BotImportSpecification {
    /// Creates a new builder-style object to manufacture [`BotImportSpecification`](crate::types::BotImportSpecification).
    pub fn builder() -> crate::types::builders::BotImportSpecificationBuilder {
        crate::types::builders::BotImportSpecificationBuilder::default()
    }
}

/// A builder for [`BotImportSpecification`](crate::types::BotImportSpecification).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BotImportSpecificationBuilder {
    pub(crate) bot_name: ::std::option::Option<::std::string::String>,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) data_privacy: ::std::option::Option<crate::types::DataPrivacy>,
    pub(crate) idle_session_ttl_in_seconds: ::std::option::Option<i32>,
    pub(crate) bot_tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) test_bot_alias_tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl BotImportSpecificationBuilder {
    /// <p>The name that Amazon Lex should use for the bot.</p>
    pub fn bot_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bot_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name that Amazon Lex should use for the bot.</p>
    pub fn set_bot_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bot_name = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role used to build and run the bot.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role used to build and run the bot.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>By default, data stored by Amazon Lex is encrypted. The <code>DataPrivacy</code> structure provides settings that determine how Amazon Lex handles special cases of securing the data for your bot. </p>
    pub fn data_privacy(mut self, input: crate::types::DataPrivacy) -> Self {
        self.data_privacy = ::std::option::Option::Some(input);
        self
    }
    /// <p>By default, data stored by Amazon Lex is encrypted. The <code>DataPrivacy</code> structure provides settings that determine how Amazon Lex handles special cases of securing the data for your bot. </p>
    pub fn set_data_privacy(
        mut self,
        input: ::std::option::Option<crate::types::DataPrivacy>,
    ) -> Self {
        self.data_privacy = input;
        self
    }
    /// <p>The time, in seconds, that Amazon Lex should keep information about a user's conversation with the bot. </p>
    /// <p>A user interaction remains active for the amount of time specified. If no conversation occurs during this time, the session expires and Amazon Lex deletes any data provided before the timeout.</p>
    /// <p>You can specify between 60 (1 minute) and 86,400 (24 hours) seconds.</p>
    pub fn idle_session_ttl_in_seconds(mut self, input: i32) -> Self {
        self.idle_session_ttl_in_seconds = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time, in seconds, that Amazon Lex should keep information about a user's conversation with the bot. </p>
    /// <p>A user interaction remains active for the amount of time specified. If no conversation occurs during this time, the session expires and Amazon Lex deletes any data provided before the timeout.</p>
    /// <p>You can specify between 60 (1 minute) and 86,400 (24 hours) seconds.</p>
    pub fn set_idle_session_ttl_in_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.idle_session_ttl_in_seconds = input;
        self
    }
    /// Adds a key-value pair to `bot_tags`.
    ///
    /// To override the contents of this collection use [`set_bot_tags`](Self::set_bot_tags).
    ///
    /// <p>A list of tags to add to the bot. You can only add tags when you import a bot. You can't use the <code>UpdateBot</code> operation to update tags. To update tags, use the <code>TagResource</code> operation.</p>
    pub fn bot_tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.bot_tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.bot_tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A list of tags to add to the bot. You can only add tags when you import a bot. You can't use the <code>UpdateBot</code> operation to update tags. To update tags, use the <code>TagResource</code> operation.</p>
    pub fn set_bot_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.bot_tags = input;
        self
    }
    /// Adds a key-value pair to `test_bot_alias_tags`.
    ///
    /// To override the contents of this collection use [`set_test_bot_alias_tags`](Self::set_test_bot_alias_tags).
    ///
    /// <p>A list of tags to add to the test alias for a bot. You can only add tags when you import a bot. You can't use the <code>UpdateAlias</code> operation to update tags. To update tags on the test alias, use the <code>TagResource</code> operation.</p>
    pub fn test_bot_alias_tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.test_bot_alias_tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.test_bot_alias_tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A list of tags to add to the test alias for a bot. You can only add tags when you import a bot. You can't use the <code>UpdateAlias</code> operation to update tags. To update tags on the test alias, use the <code>TagResource</code> operation.</p>
    pub fn set_test_bot_alias_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.test_bot_alias_tags = input;
        self
    }
    /// Consumes the builder and constructs a [`BotImportSpecification`](crate::types::BotImportSpecification).
    pub fn build(self) -> crate::types::BotImportSpecification {
        crate::types::BotImportSpecification {
            bot_name: self.bot_name,
            role_arn: self.role_arn,
            data_privacy: self.data_privacy,
            idle_session_ttl_in_seconds: self.idle_session_ttl_in_seconds,
            bot_tags: self.bot_tags,
            test_bot_alias_tags: self.test_bot_alias_tags,
        }
    }
}
