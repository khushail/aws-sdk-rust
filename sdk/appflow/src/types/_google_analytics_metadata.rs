// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The connector metadata specific to Google Analytics. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GoogleAnalyticsMetadata {
    /// <p> The desired authorization scope for the Google Analytics account. </p>
    #[doc(hidden)]
    pub o_auth_scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl GoogleAnalyticsMetadata {
    /// <p> The desired authorization scope for the Google Analytics account. </p>
    pub fn o_auth_scopes(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.o_auth_scopes.as_deref()
    }
}
impl GoogleAnalyticsMetadata {
    /// Creates a new builder-style object to manufacture [`GoogleAnalyticsMetadata`](crate::types::GoogleAnalyticsMetadata).
    pub fn builder() -> crate::types::builders::GoogleAnalyticsMetadataBuilder {
        crate::types::builders::GoogleAnalyticsMetadataBuilder::default()
    }
}

/// A builder for [`GoogleAnalyticsMetadata`](crate::types::GoogleAnalyticsMetadata).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GoogleAnalyticsMetadataBuilder {
    pub(crate) o_auth_scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl GoogleAnalyticsMetadataBuilder {
    /// Appends an item to `o_auth_scopes`.
    ///
    /// To override the contents of this collection use [`set_o_auth_scopes`](Self::set_o_auth_scopes).
    ///
    /// <p> The desired authorization scope for the Google Analytics account. </p>
    pub fn o_auth_scopes(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.o_auth_scopes.unwrap_or_default();
        v.push(input.into());
        self.o_auth_scopes = ::std::option::Option::Some(v);
        self
    }
    /// <p> The desired authorization scope for the Google Analytics account. </p>
    pub fn set_o_auth_scopes(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.o_auth_scopes = input;
        self
    }
    /// Consumes the builder and constructs a [`GoogleAnalyticsMetadata`](crate::types::GoogleAnalyticsMetadata).
    pub fn build(self) -> crate::types::GoogleAnalyticsMetadata {
        crate::types::GoogleAnalyticsMetadata {
            o_auth_scopes: self.o_auth_scopes,
        }
    }
}
