// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details about the recommended course of action to remediate the finding.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RemediationRecommendation {
    /// <p>The recommended course of action to remediate the finding.</p>
    #[doc(hidden)]
    pub text: ::std::option::Option<::std::string::String>,
    /// <p>A link to more information about the recommended remediation for this vulnerability.</p>
    #[doc(hidden)]
    pub url: ::std::option::Option<::std::string::String>,
}
impl RemediationRecommendation {
    /// <p>The recommended course of action to remediate the finding.</p>
    pub fn text(&self) -> ::std::option::Option<&str> {
        self.text.as_deref()
    }
    /// <p>A link to more information about the recommended remediation for this vulnerability.</p>
    pub fn url(&self) -> ::std::option::Option<&str> {
        self.url.as_deref()
    }
}
impl RemediationRecommendation {
    /// Creates a new builder-style object to manufacture [`RemediationRecommendation`](crate::types::RemediationRecommendation).
    pub fn builder() -> crate::types::builders::RemediationRecommendationBuilder {
        crate::types::builders::RemediationRecommendationBuilder::default()
    }
}

/// A builder for [`RemediationRecommendation`](crate::types::RemediationRecommendation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RemediationRecommendationBuilder {
    pub(crate) text: ::std::option::Option<::std::string::String>,
    pub(crate) url: ::std::option::Option<::std::string::String>,
}
impl RemediationRecommendationBuilder {
    /// <p>The recommended course of action to remediate the finding.</p>
    pub fn text(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.text = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The recommended course of action to remediate the finding.</p>
    pub fn set_text(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.text = input;
        self
    }
    /// <p>A link to more information about the recommended remediation for this vulnerability.</p>
    pub fn url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A link to more information about the recommended remediation for this vulnerability.</p>
    pub fn set_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.url = input;
        self
    }
    /// Consumes the builder and constructs a [`RemediationRecommendation`](crate::types::RemediationRecommendation).
    pub fn build(self) -> crate::types::RemediationRecommendation {
        crate::types::RemediationRecommendation {
            text: self.text,
            url: self.url,
        }
    }
}
