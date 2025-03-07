// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides the bot locale parameters required for exporting a bot locale.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BotLocaleExportSpecification {
    /// <p>The identifier of the bot to create the locale for.</p>
    #[doc(hidden)]
    pub bot_id: ::std::option::Option<::std::string::String>,
    /// <p>The version of the bot to export.</p>
    #[doc(hidden)]
    pub bot_version: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the language and locale to export. The string must match one of the locales in the bot.</p>
    #[doc(hidden)]
    pub locale_id: ::std::option::Option<::std::string::String>,
}
impl BotLocaleExportSpecification {
    /// <p>The identifier of the bot to create the locale for.</p>
    pub fn bot_id(&self) -> ::std::option::Option<&str> {
        self.bot_id.as_deref()
    }
    /// <p>The version of the bot to export.</p>
    pub fn bot_version(&self) -> ::std::option::Option<&str> {
        self.bot_version.as_deref()
    }
    /// <p>The identifier of the language and locale to export. The string must match one of the locales in the bot.</p>
    pub fn locale_id(&self) -> ::std::option::Option<&str> {
        self.locale_id.as_deref()
    }
}
impl BotLocaleExportSpecification {
    /// Creates a new builder-style object to manufacture [`BotLocaleExportSpecification`](crate::types::BotLocaleExportSpecification).
    pub fn builder() -> crate::types::builders::BotLocaleExportSpecificationBuilder {
        crate::types::builders::BotLocaleExportSpecificationBuilder::default()
    }
}

/// A builder for [`BotLocaleExportSpecification`](crate::types::BotLocaleExportSpecification).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BotLocaleExportSpecificationBuilder {
    pub(crate) bot_id: ::std::option::Option<::std::string::String>,
    pub(crate) bot_version: ::std::option::Option<::std::string::String>,
    pub(crate) locale_id: ::std::option::Option<::std::string::String>,
}
impl BotLocaleExportSpecificationBuilder {
    /// <p>The identifier of the bot to create the locale for.</p>
    pub fn bot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bot_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the bot to create the locale for.</p>
    pub fn set_bot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bot_id = input;
        self
    }
    /// <p>The version of the bot to export.</p>
    pub fn bot_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bot_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the bot to export.</p>
    pub fn set_bot_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bot_version = input;
        self
    }
    /// <p>The identifier of the language and locale to export. The string must match one of the locales in the bot.</p>
    pub fn locale_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.locale_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the language and locale to export. The string must match one of the locales in the bot.</p>
    pub fn set_locale_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.locale_id = input;
        self
    }
    /// Consumes the builder and constructs a [`BotLocaleExportSpecification`](crate::types::BotLocaleExportSpecification).
    pub fn build(self) -> crate::types::BotLocaleExportSpecification {
        crate::types::BotLocaleExportSpecification {
            bot_id: self.bot_id,
            bot_version: self.bot_version,
            locale_id: self.locale_id,
        }
    }
}
