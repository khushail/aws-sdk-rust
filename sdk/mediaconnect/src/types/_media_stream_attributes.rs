// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Attributes that are related to the media stream.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MediaStreamAttributes {
    /// A set of parameters that define the media stream.
    #[doc(hidden)]
    pub fmtp: ::std::option::Option<crate::types::Fmtp>,
    /// The audio language, in a format that is recognized by the receiver.
    #[doc(hidden)]
    pub lang: ::std::option::Option<::std::string::String>,
}
impl MediaStreamAttributes {
    /// A set of parameters that define the media stream.
    pub fn fmtp(&self) -> ::std::option::Option<&crate::types::Fmtp> {
        self.fmtp.as_ref()
    }
    /// The audio language, in a format that is recognized by the receiver.
    pub fn lang(&self) -> ::std::option::Option<&str> {
        self.lang.as_deref()
    }
}
impl MediaStreamAttributes {
    /// Creates a new builder-style object to manufacture [`MediaStreamAttributes`](crate::types::MediaStreamAttributes).
    pub fn builder() -> crate::types::builders::MediaStreamAttributesBuilder {
        crate::types::builders::MediaStreamAttributesBuilder::default()
    }
}

/// A builder for [`MediaStreamAttributes`](crate::types::MediaStreamAttributes).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MediaStreamAttributesBuilder {
    pub(crate) fmtp: ::std::option::Option<crate::types::Fmtp>,
    pub(crate) lang: ::std::option::Option<::std::string::String>,
}
impl MediaStreamAttributesBuilder {
    /// A set of parameters that define the media stream.
    pub fn fmtp(mut self, input: crate::types::Fmtp) -> Self {
        self.fmtp = ::std::option::Option::Some(input);
        self
    }
    /// A set of parameters that define the media stream.
    pub fn set_fmtp(mut self, input: ::std::option::Option<crate::types::Fmtp>) -> Self {
        self.fmtp = input;
        self
    }
    /// The audio language, in a format that is recognized by the receiver.
    pub fn lang(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.lang = ::std::option::Option::Some(input.into());
        self
    }
    /// The audio language, in a format that is recognized by the receiver.
    pub fn set_lang(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.lang = input;
        self
    }
    /// Consumes the builder and constructs a [`MediaStreamAttributes`](crate::types::MediaStreamAttributes).
    pub fn build(self) -> crate::types::MediaStreamAttributes {
        crate::types::MediaStreamAttributes {
            fmtp: self.fmtp,
            lang: self.lang,
        }
    }
}
