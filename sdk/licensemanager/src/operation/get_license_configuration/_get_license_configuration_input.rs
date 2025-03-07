// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetLicenseConfigurationInput {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[doc(hidden)]
    pub license_configuration_arn: ::std::option::Option<::std::string::String>,
}
impl GetLicenseConfigurationInput {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    pub fn license_configuration_arn(&self) -> ::std::option::Option<&str> {
        self.license_configuration_arn.as_deref()
    }
}
impl GetLicenseConfigurationInput {
    /// Creates a new builder-style object to manufacture [`GetLicenseConfigurationInput`](crate::operation::get_license_configuration::GetLicenseConfigurationInput).
    pub fn builder(
    ) -> crate::operation::get_license_configuration::builders::GetLicenseConfigurationInputBuilder
    {
        crate::operation::get_license_configuration::builders::GetLicenseConfigurationInputBuilder::default()
    }
}

/// A builder for [`GetLicenseConfigurationInput`](crate::operation::get_license_configuration::GetLicenseConfigurationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetLicenseConfigurationInputBuilder {
    pub(crate) license_configuration_arn: ::std::option::Option<::std::string::String>,
}
impl GetLicenseConfigurationInputBuilder {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    pub fn license_configuration_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.license_configuration_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    pub fn set_license_configuration_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.license_configuration_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`GetLicenseConfigurationInput`](crate::operation::get_license_configuration::GetLicenseConfigurationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_license_configuration::GetLicenseConfigurationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_license_configuration::GetLicenseConfigurationInput {
                license_configuration_arn: self.license_configuration_arn,
            },
        )
    }
}
