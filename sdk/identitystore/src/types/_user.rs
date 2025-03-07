// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A user object that contains the metadata and attributes for a specified user.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct User {
    /// <p>A unique string used to identify the user. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store.</p>
    #[doc(hidden)]
    pub user_name: ::std::option::Option<::std::string::String>,
    /// <p>The identifier for a user in the identity store.</p>
    #[doc(hidden)]
    pub user_id: ::std::option::Option<::std::string::String>,
    /// <p>A list of <code>ExternalId</code> objects that contains the identifiers issued to this resource by an external identity provider.</p>
    #[doc(hidden)]
    pub external_ids: ::std::option::Option<::std::vec::Vec<crate::types::ExternalId>>,
    /// <p>An object containing the name of the user.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<crate::types::Name>,
    /// <p>A string containing the name of the user that is formatted for display when the user is referenced. For example, "John Doe."</p>
    #[doc(hidden)]
    pub display_name: ::std::option::Option<::std::string::String>,
    /// <p>A string containing an alternate name for the user.</p>
    #[doc(hidden)]
    pub nick_name: ::std::option::Option<::std::string::String>,
    /// <p>A string containing a URL that might be associated with the user.</p>
    #[doc(hidden)]
    pub profile_url: ::std::option::Option<::std::string::String>,
    /// <p>A list of <code>Email</code> objects containing email addresses associated with the user.</p>
    #[doc(hidden)]
    pub emails: ::std::option::Option<::std::vec::Vec<crate::types::Email>>,
    /// <p>A list of <code>Address</code> objects containing addresses associated with the user.</p>
    #[doc(hidden)]
    pub addresses: ::std::option::Option<::std::vec::Vec<crate::types::Address>>,
    /// <p>A list of <code>PhoneNumber</code> objects containing phone numbers associated with the user.</p>
    #[doc(hidden)]
    pub phone_numbers: ::std::option::Option<::std::vec::Vec<crate::types::PhoneNumber>>,
    /// <p>A string indicating the type of user. Possible values are left unspecified. The value can vary based on your specific use case.</p>
    #[doc(hidden)]
    pub user_type: ::std::option::Option<::std::string::String>,
    /// <p>A string containing the title of the user. Possible values are left unspecified. The value can vary based on your specific use case.</p>
    #[doc(hidden)]
    pub title: ::std::option::Option<::std::string::String>,
    /// <p>A string containing the preferred language of the user. For example, "American English" or "en-us."</p>
    #[doc(hidden)]
    pub preferred_language: ::std::option::Option<::std::string::String>,
    /// <p>A string containing the geographical region or location of the user.</p>
    #[doc(hidden)]
    pub locale: ::std::option::Option<::std::string::String>,
    /// <p>A string containing the time zone of the user.</p>
    #[doc(hidden)]
    pub timezone: ::std::option::Option<::std::string::String>,
    /// <p>The globally unique identifier for the identity store.</p>
    #[doc(hidden)]
    pub identity_store_id: ::std::option::Option<::std::string::String>,
}
impl User {
    /// <p>A unique string used to identify the user. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store.</p>
    pub fn user_name(&self) -> ::std::option::Option<&str> {
        self.user_name.as_deref()
    }
    /// <p>The identifier for a user in the identity store.</p>
    pub fn user_id(&self) -> ::std::option::Option<&str> {
        self.user_id.as_deref()
    }
    /// <p>A list of <code>ExternalId</code> objects that contains the identifiers issued to this resource by an external identity provider.</p>
    pub fn external_ids(&self) -> ::std::option::Option<&[crate::types::ExternalId]> {
        self.external_ids.as_deref()
    }
    /// <p>An object containing the name of the user.</p>
    pub fn name(&self) -> ::std::option::Option<&crate::types::Name> {
        self.name.as_ref()
    }
    /// <p>A string containing the name of the user that is formatted for display when the user is referenced. For example, "John Doe."</p>
    pub fn display_name(&self) -> ::std::option::Option<&str> {
        self.display_name.as_deref()
    }
    /// <p>A string containing an alternate name for the user.</p>
    pub fn nick_name(&self) -> ::std::option::Option<&str> {
        self.nick_name.as_deref()
    }
    /// <p>A string containing a URL that might be associated with the user.</p>
    pub fn profile_url(&self) -> ::std::option::Option<&str> {
        self.profile_url.as_deref()
    }
    /// <p>A list of <code>Email</code> objects containing email addresses associated with the user.</p>
    pub fn emails(&self) -> ::std::option::Option<&[crate::types::Email]> {
        self.emails.as_deref()
    }
    /// <p>A list of <code>Address</code> objects containing addresses associated with the user.</p>
    pub fn addresses(&self) -> ::std::option::Option<&[crate::types::Address]> {
        self.addresses.as_deref()
    }
    /// <p>A list of <code>PhoneNumber</code> objects containing phone numbers associated with the user.</p>
    pub fn phone_numbers(&self) -> ::std::option::Option<&[crate::types::PhoneNumber]> {
        self.phone_numbers.as_deref()
    }
    /// <p>A string indicating the type of user. Possible values are left unspecified. The value can vary based on your specific use case.</p>
    pub fn user_type(&self) -> ::std::option::Option<&str> {
        self.user_type.as_deref()
    }
    /// <p>A string containing the title of the user. Possible values are left unspecified. The value can vary based on your specific use case.</p>
    pub fn title(&self) -> ::std::option::Option<&str> {
        self.title.as_deref()
    }
    /// <p>A string containing the preferred language of the user. For example, "American English" or "en-us."</p>
    pub fn preferred_language(&self) -> ::std::option::Option<&str> {
        self.preferred_language.as_deref()
    }
    /// <p>A string containing the geographical region or location of the user.</p>
    pub fn locale(&self) -> ::std::option::Option<&str> {
        self.locale.as_deref()
    }
    /// <p>A string containing the time zone of the user.</p>
    pub fn timezone(&self) -> ::std::option::Option<&str> {
        self.timezone.as_deref()
    }
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn identity_store_id(&self) -> ::std::option::Option<&str> {
        self.identity_store_id.as_deref()
    }
}
impl ::std::fmt::Debug for User {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("User");
        formatter.field("user_name", &"*** Sensitive Data Redacted ***");
        formatter.field("user_id", &self.user_id);
        formatter.field("external_ids", &self.external_ids);
        formatter.field("name", &self.name);
        formatter.field("display_name", &"*** Sensitive Data Redacted ***");
        formatter.field("nick_name", &"*** Sensitive Data Redacted ***");
        formatter.field("profile_url", &"*** Sensitive Data Redacted ***");
        formatter.field("emails", &self.emails);
        formatter.field("addresses", &self.addresses);
        formatter.field("phone_numbers", &self.phone_numbers);
        formatter.field("user_type", &"*** Sensitive Data Redacted ***");
        formatter.field("title", &"*** Sensitive Data Redacted ***");
        formatter.field("preferred_language", &"*** Sensitive Data Redacted ***");
        formatter.field("locale", &"*** Sensitive Data Redacted ***");
        formatter.field("timezone", &"*** Sensitive Data Redacted ***");
        formatter.field("identity_store_id", &self.identity_store_id);
        formatter.finish()
    }
}
impl User {
    /// Creates a new builder-style object to manufacture [`User`](crate::types::User).
    pub fn builder() -> crate::types::builders::UserBuilder {
        crate::types::builders::UserBuilder::default()
    }
}

/// A builder for [`User`](crate::types::User).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct UserBuilder {
    pub(crate) user_name: ::std::option::Option<::std::string::String>,
    pub(crate) user_id: ::std::option::Option<::std::string::String>,
    pub(crate) external_ids: ::std::option::Option<::std::vec::Vec<crate::types::ExternalId>>,
    pub(crate) name: ::std::option::Option<crate::types::Name>,
    pub(crate) display_name: ::std::option::Option<::std::string::String>,
    pub(crate) nick_name: ::std::option::Option<::std::string::String>,
    pub(crate) profile_url: ::std::option::Option<::std::string::String>,
    pub(crate) emails: ::std::option::Option<::std::vec::Vec<crate::types::Email>>,
    pub(crate) addresses: ::std::option::Option<::std::vec::Vec<crate::types::Address>>,
    pub(crate) phone_numbers: ::std::option::Option<::std::vec::Vec<crate::types::PhoneNumber>>,
    pub(crate) user_type: ::std::option::Option<::std::string::String>,
    pub(crate) title: ::std::option::Option<::std::string::String>,
    pub(crate) preferred_language: ::std::option::Option<::std::string::String>,
    pub(crate) locale: ::std::option::Option<::std::string::String>,
    pub(crate) timezone: ::std::option::Option<::std::string::String>,
    pub(crate) identity_store_id: ::std::option::Option<::std::string::String>,
}
impl UserBuilder {
    /// <p>A unique string used to identify the user. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store.</p>
    pub fn user_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique string used to identify the user. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store.</p>
    pub fn set_user_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_name = input;
        self
    }
    /// <p>The identifier for a user in the identity store.</p>
    pub fn user_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for a user in the identity store.</p>
    pub fn set_user_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_id = input;
        self
    }
    /// Appends an item to `external_ids`.
    ///
    /// To override the contents of this collection use [`set_external_ids`](Self::set_external_ids).
    ///
    /// <p>A list of <code>ExternalId</code> objects that contains the identifiers issued to this resource by an external identity provider.</p>
    pub fn external_ids(mut self, input: crate::types::ExternalId) -> Self {
        let mut v = self.external_ids.unwrap_or_default();
        v.push(input);
        self.external_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>ExternalId</code> objects that contains the identifiers issued to this resource by an external identity provider.</p>
    pub fn set_external_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ExternalId>>,
    ) -> Self {
        self.external_ids = input;
        self
    }
    /// <p>An object containing the name of the user.</p>
    pub fn name(mut self, input: crate::types::Name) -> Self {
        self.name = ::std::option::Option::Some(input);
        self
    }
    /// <p>An object containing the name of the user.</p>
    pub fn set_name(mut self, input: ::std::option::Option<crate::types::Name>) -> Self {
        self.name = input;
        self
    }
    /// <p>A string containing the name of the user that is formatted for display when the user is referenced. For example, "John Doe."</p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.display_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string containing the name of the user that is formatted for display when the user is referenced. For example, "John Doe."</p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.display_name = input;
        self
    }
    /// <p>A string containing an alternate name for the user.</p>
    pub fn nick_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.nick_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string containing an alternate name for the user.</p>
    pub fn set_nick_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.nick_name = input;
        self
    }
    /// <p>A string containing a URL that might be associated with the user.</p>
    pub fn profile_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.profile_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string containing a URL that might be associated with the user.</p>
    pub fn set_profile_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.profile_url = input;
        self
    }
    /// Appends an item to `emails`.
    ///
    /// To override the contents of this collection use [`set_emails`](Self::set_emails).
    ///
    /// <p>A list of <code>Email</code> objects containing email addresses associated with the user.</p>
    pub fn emails(mut self, input: crate::types::Email) -> Self {
        let mut v = self.emails.unwrap_or_default();
        v.push(input);
        self.emails = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>Email</code> objects containing email addresses associated with the user.</p>
    pub fn set_emails(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Email>>,
    ) -> Self {
        self.emails = input;
        self
    }
    /// Appends an item to `addresses`.
    ///
    /// To override the contents of this collection use [`set_addresses`](Self::set_addresses).
    ///
    /// <p>A list of <code>Address</code> objects containing addresses associated with the user.</p>
    pub fn addresses(mut self, input: crate::types::Address) -> Self {
        let mut v = self.addresses.unwrap_or_default();
        v.push(input);
        self.addresses = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>Address</code> objects containing addresses associated with the user.</p>
    pub fn set_addresses(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Address>>,
    ) -> Self {
        self.addresses = input;
        self
    }
    /// Appends an item to `phone_numbers`.
    ///
    /// To override the contents of this collection use [`set_phone_numbers`](Self::set_phone_numbers).
    ///
    /// <p>A list of <code>PhoneNumber</code> objects containing phone numbers associated with the user.</p>
    pub fn phone_numbers(mut self, input: crate::types::PhoneNumber) -> Self {
        let mut v = self.phone_numbers.unwrap_or_default();
        v.push(input);
        self.phone_numbers = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>PhoneNumber</code> objects containing phone numbers associated with the user.</p>
    pub fn set_phone_numbers(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PhoneNumber>>,
    ) -> Self {
        self.phone_numbers = input;
        self
    }
    /// <p>A string indicating the type of user. Possible values are left unspecified. The value can vary based on your specific use case.</p>
    pub fn user_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string indicating the type of user. Possible values are left unspecified. The value can vary based on your specific use case.</p>
    pub fn set_user_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_type = input;
        self
    }
    /// <p>A string containing the title of the user. Possible values are left unspecified. The value can vary based on your specific use case.</p>
    pub fn title(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.title = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string containing the title of the user. Possible values are left unspecified. The value can vary based on your specific use case.</p>
    pub fn set_title(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.title = input;
        self
    }
    /// <p>A string containing the preferred language of the user. For example, "American English" or "en-us."</p>
    pub fn preferred_language(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.preferred_language = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string containing the preferred language of the user. For example, "American English" or "en-us."</p>
    pub fn set_preferred_language(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.preferred_language = input;
        self
    }
    /// <p>A string containing the geographical region or location of the user.</p>
    pub fn locale(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.locale = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string containing the geographical region or location of the user.</p>
    pub fn set_locale(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.locale = input;
        self
    }
    /// <p>A string containing the time zone of the user.</p>
    pub fn timezone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.timezone = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string containing the time zone of the user.</p>
    pub fn set_timezone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.timezone = input;
        self
    }
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn identity_store_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.identity_store_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn set_identity_store_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.identity_store_id = input;
        self
    }
    /// Consumes the builder and constructs a [`User`](crate::types::User).
    pub fn build(self) -> crate::types::User {
        crate::types::User {
            user_name: self.user_name,
            user_id: self.user_id,
            external_ids: self.external_ids,
            name: self.name,
            display_name: self.display_name,
            nick_name: self.nick_name,
            profile_url: self.profile_url,
            emails: self.emails,
            addresses: self.addresses,
            phone_numbers: self.phone_numbers,
            user_type: self.user_type,
            title: self.title,
            preferred_language: self.preferred_language,
            locale: self.locale,
            timezone: self.timezone,
            identity_store_id: self.identity_store_id,
        }
    }
}
impl ::std::fmt::Debug for UserBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("UserBuilder");
        formatter.field("user_name", &"*** Sensitive Data Redacted ***");
        formatter.field("user_id", &self.user_id);
        formatter.field("external_ids", &self.external_ids);
        formatter.field("name", &self.name);
        formatter.field("display_name", &"*** Sensitive Data Redacted ***");
        formatter.field("nick_name", &"*** Sensitive Data Redacted ***");
        formatter.field("profile_url", &"*** Sensitive Data Redacted ***");
        formatter.field("emails", &self.emails);
        formatter.field("addresses", &self.addresses);
        formatter.field("phone_numbers", &self.phone_numbers);
        formatter.field("user_type", &"*** Sensitive Data Redacted ***");
        formatter.field("title", &"*** Sensitive Data Redacted ***");
        formatter.field("preferred_language", &"*** Sensitive Data Redacted ***");
        formatter.field("locale", &"*** Sensitive Data Redacted ***");
        formatter.field("timezone", &"*** Sensitive Data Redacted ***");
        formatter.field("identity_store_id", &self.identity_store_id);
        formatter.finish()
    }
}
