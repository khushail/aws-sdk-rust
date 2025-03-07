// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateTrustAnchorInput {
    /// <p>The name of the trust anchor.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The trust anchor type and its related certificate data.</p>
    #[doc(hidden)]
    pub source: ::std::option::Option<crate::types::Source>,
    /// <p>Specifies whether the trust anchor is enabled.</p>
    #[doc(hidden)]
    pub enabled: ::std::option::Option<bool>,
    /// <p>The tags to attach to the trust anchor.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>A list of notification settings to be associated to the trust anchor.</p>
    #[doc(hidden)]
    pub notification_settings:
        ::std::option::Option<::std::vec::Vec<crate::types::NotificationSetting>>,
}
impl CreateTrustAnchorInput {
    /// <p>The name of the trust anchor.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The trust anchor type and its related certificate data.</p>
    pub fn source(&self) -> ::std::option::Option<&crate::types::Source> {
        self.source.as_ref()
    }
    /// <p>Specifies whether the trust anchor is enabled.</p>
    pub fn enabled(&self) -> ::std::option::Option<bool> {
        self.enabled
    }
    /// <p>The tags to attach to the trust anchor.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
    /// <p>A list of notification settings to be associated to the trust anchor.</p>
    pub fn notification_settings(
        &self,
    ) -> ::std::option::Option<&[crate::types::NotificationSetting]> {
        self.notification_settings.as_deref()
    }
}
impl CreateTrustAnchorInput {
    /// Creates a new builder-style object to manufacture [`CreateTrustAnchorInput`](crate::operation::create_trust_anchor::CreateTrustAnchorInput).
    pub fn builder(
    ) -> crate::operation::create_trust_anchor::builders::CreateTrustAnchorInputBuilder {
        crate::operation::create_trust_anchor::builders::CreateTrustAnchorInputBuilder::default()
    }
}

/// A builder for [`CreateTrustAnchorInput`](crate::operation::create_trust_anchor::CreateTrustAnchorInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateTrustAnchorInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) source: ::std::option::Option<crate::types::Source>,
    pub(crate) enabled: ::std::option::Option<bool>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) notification_settings:
        ::std::option::Option<::std::vec::Vec<crate::types::NotificationSetting>>,
}
impl CreateTrustAnchorInputBuilder {
    /// <p>The name of the trust anchor.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the trust anchor.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The trust anchor type and its related certificate data.</p>
    pub fn source(mut self, input: crate::types::Source) -> Self {
        self.source = ::std::option::Option::Some(input);
        self
    }
    /// <p>The trust anchor type and its related certificate data.</p>
    pub fn set_source(mut self, input: ::std::option::Option<crate::types::Source>) -> Self {
        self.source = input;
        self
    }
    /// <p>Specifies whether the trust anchor is enabled.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether the trust anchor is enabled.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to attach to the trust anchor.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to attach to the trust anchor.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Appends an item to `notification_settings`.
    ///
    /// To override the contents of this collection use [`set_notification_settings`](Self::set_notification_settings).
    ///
    /// <p>A list of notification settings to be associated to the trust anchor.</p>
    pub fn notification_settings(mut self, input: crate::types::NotificationSetting) -> Self {
        let mut v = self.notification_settings.unwrap_or_default();
        v.push(input);
        self.notification_settings = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of notification settings to be associated to the trust anchor.</p>
    pub fn set_notification_settings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::NotificationSetting>>,
    ) -> Self {
        self.notification_settings = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateTrustAnchorInput`](crate::operation::create_trust_anchor::CreateTrustAnchorInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_trust_anchor::CreateTrustAnchorInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_trust_anchor::CreateTrustAnchorInput {
                name: self.name,
                source: self.source,
                enabled: self.enabled,
                tags: self.tags,
                notification_settings: self.notification_settings,
            },
        )
    }
}
