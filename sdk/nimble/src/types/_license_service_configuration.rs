// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration for a license service that is associated with a studio resource.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct LicenseServiceConfiguration {
    /// <p>The endpoint of the license service that is accessed by the studio component resource.</p>
    #[doc(hidden)]
    pub endpoint: ::std::option::Option<::std::string::String>,
}
impl LicenseServiceConfiguration {
    /// <p>The endpoint of the license service that is accessed by the studio component resource.</p>
    pub fn endpoint(&self) -> ::std::option::Option<&str> {
        self.endpoint.as_deref()
    }
}
impl ::std::fmt::Debug for LicenseServiceConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("LicenseServiceConfiguration");
        formatter.field("endpoint", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl LicenseServiceConfiguration {
    /// Creates a new builder-style object to manufacture [`LicenseServiceConfiguration`](crate::types::LicenseServiceConfiguration).
    pub fn builder() -> crate::types::builders::LicenseServiceConfigurationBuilder {
        crate::types::builders::LicenseServiceConfigurationBuilder::default()
    }
}

/// A builder for [`LicenseServiceConfiguration`](crate::types::LicenseServiceConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct LicenseServiceConfigurationBuilder {
    pub(crate) endpoint: ::std::option::Option<::std::string::String>,
}
impl LicenseServiceConfigurationBuilder {
    /// <p>The endpoint of the license service that is accessed by the studio component resource.</p>
    pub fn endpoint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.endpoint = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The endpoint of the license service that is accessed by the studio component resource.</p>
    pub fn set_endpoint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.endpoint = input;
        self
    }
    /// Consumes the builder and constructs a [`LicenseServiceConfiguration`](crate::types::LicenseServiceConfiguration).
    pub fn build(self) -> crate::types::LicenseServiceConfiguration {
        crate::types::LicenseServiceConfiguration {
            endpoint: self.endpoint,
        }
    }
}
impl ::std::fmt::Debug for LicenseServiceConfigurationBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("LicenseServiceConfigurationBuilder");
        formatter.field("endpoint", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
