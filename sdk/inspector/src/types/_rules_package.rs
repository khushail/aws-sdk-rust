// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about an Amazon Inspector rules package. This data type is used as the response element in the <code>DescribeRulesPackages</code> action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RulesPackage {
    /// <p>The ARN of the rules package.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the rules package.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The version ID of the rules package.</p>
    #[doc(hidden)]
    pub version: ::std::option::Option<::std::string::String>,
    /// <p>The provider of the rules package.</p>
    #[doc(hidden)]
    pub provider: ::std::option::Option<::std::string::String>,
    /// <p>The description of the rules package.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
}
impl RulesPackage {
    /// <p>The ARN of the rules package.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The name of the rules package.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The version ID of the rules package.</p>
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
    /// <p>The provider of the rules package.</p>
    pub fn provider(&self) -> ::std::option::Option<&str> {
        self.provider.as_deref()
    }
    /// <p>The description of the rules package.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
}
impl RulesPackage {
    /// Creates a new builder-style object to manufacture [`RulesPackage`](crate::types::RulesPackage).
    pub fn builder() -> crate::types::builders::RulesPackageBuilder {
        crate::types::builders::RulesPackageBuilder::default()
    }
}

/// A builder for [`RulesPackage`](crate::types::RulesPackage).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RulesPackageBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<::std::string::String>,
    pub(crate) provider: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
}
impl RulesPackageBuilder {
    /// <p>The ARN of the rules package.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the rules package.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The name of the rules package.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the rules package.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The version ID of the rules package.</p>
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version ID of the rules package.</p>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    /// <p>The provider of the rules package.</p>
    pub fn provider(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.provider = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The provider of the rules package.</p>
    pub fn set_provider(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.provider = input;
        self
    }
    /// <p>The description of the rules package.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the rules package.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Consumes the builder and constructs a [`RulesPackage`](crate::types::RulesPackage).
    pub fn build(self) -> crate::types::RulesPackage {
        crate::types::RulesPackage {
            arn: self.arn,
            name: self.name,
            version: self.version,
            provider: self.provider,
            description: self.description,
        }
    }
}
