// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details for associating a license configuration with a resource.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LicenseSpecification {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[doc(hidden)]
    pub license_configuration_arn: ::std::option::Option<::std::string::String>,
    /// <p>Scope of AMI associations. The possible value is <code>cross-account</code>.</p>
    #[doc(hidden)]
    pub ami_association_scope: ::std::option::Option<::std::string::String>,
}
impl LicenseSpecification {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    pub fn license_configuration_arn(&self) -> ::std::option::Option<&str> {
        self.license_configuration_arn.as_deref()
    }
    /// <p>Scope of AMI associations. The possible value is <code>cross-account</code>.</p>
    pub fn ami_association_scope(&self) -> ::std::option::Option<&str> {
        self.ami_association_scope.as_deref()
    }
}
impl LicenseSpecification {
    /// Creates a new builder-style object to manufacture [`LicenseSpecification`](crate::types::LicenseSpecification).
    pub fn builder() -> crate::types::builders::LicenseSpecificationBuilder {
        crate::types::builders::LicenseSpecificationBuilder::default()
    }
}

/// A builder for [`LicenseSpecification`](crate::types::LicenseSpecification).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LicenseSpecificationBuilder {
    pub(crate) license_configuration_arn: ::std::option::Option<::std::string::String>,
    pub(crate) ami_association_scope: ::std::option::Option<::std::string::String>,
}
impl LicenseSpecificationBuilder {
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
    /// <p>Scope of AMI associations. The possible value is <code>cross-account</code>.</p>
    pub fn ami_association_scope(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.ami_association_scope = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Scope of AMI associations. The possible value is <code>cross-account</code>.</p>
    pub fn set_ami_association_scope(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.ami_association_scope = input;
        self
    }
    /// Consumes the builder and constructs a [`LicenseSpecification`](crate::types::LicenseSpecification).
    pub fn build(self) -> crate::types::LicenseSpecification {
        crate::types::LicenseSpecification {
            license_configuration_arn: self.license_configuration_arn,
            ami_association_scope: self.ami_association_scope,
        }
    }
}
