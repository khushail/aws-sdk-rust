// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a license configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LicenseConfiguration {
    /// <p>The Amazon Resource Name (ARN) of the license configuration.</p>
    #[doc(hidden)]
    pub license_configuration_arn: ::std::option::Option<::std::string::String>,
}
impl LicenseConfiguration {
    /// <p>The Amazon Resource Name (ARN) of the license configuration.</p>
    pub fn license_configuration_arn(&self) -> ::std::option::Option<&str> {
        self.license_configuration_arn.as_deref()
    }
}
impl LicenseConfiguration {
    /// Creates a new builder-style object to manufacture [`LicenseConfiguration`](crate::types::LicenseConfiguration).
    pub fn builder() -> crate::types::builders::LicenseConfigurationBuilder {
        crate::types::builders::LicenseConfigurationBuilder::default()
    }
}

/// A builder for [`LicenseConfiguration`](crate::types::LicenseConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LicenseConfigurationBuilder {
    pub(crate) license_configuration_arn: ::std::option::Option<::std::string::String>,
}
impl LicenseConfigurationBuilder {
    /// <p>The Amazon Resource Name (ARN) of the license configuration.</p>
    pub fn license_configuration_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.license_configuration_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the license configuration.</p>
    pub fn set_license_configuration_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.license_configuration_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`LicenseConfiguration`](crate::types::LicenseConfiguration).
    pub fn build(self) -> crate::types::LicenseConfiguration {
        crate::types::LicenseConfiguration {
            license_configuration_arn: self.license_configuration_arn,
        }
    }
}
