// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Describes the settings for the subdomain. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SubDomainSetting {
    /// <p> The prefix setting for the subdomain. </p>
    #[doc(hidden)]
    pub prefix: ::std::option::Option<::std::string::String>,
    /// <p> The branch name setting for the subdomain. </p>
    #[doc(hidden)]
    pub branch_name: ::std::option::Option<::std::string::String>,
}
impl SubDomainSetting {
    /// <p> The prefix setting for the subdomain. </p>
    pub fn prefix(&self) -> ::std::option::Option<&str> {
        self.prefix.as_deref()
    }
    /// <p> The branch name setting for the subdomain. </p>
    pub fn branch_name(&self) -> ::std::option::Option<&str> {
        self.branch_name.as_deref()
    }
}
impl SubDomainSetting {
    /// Creates a new builder-style object to manufacture [`SubDomainSetting`](crate::types::SubDomainSetting).
    pub fn builder() -> crate::types::builders::SubDomainSettingBuilder {
        crate::types::builders::SubDomainSettingBuilder::default()
    }
}

/// A builder for [`SubDomainSetting`](crate::types::SubDomainSetting).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SubDomainSettingBuilder {
    pub(crate) prefix: ::std::option::Option<::std::string::String>,
    pub(crate) branch_name: ::std::option::Option<::std::string::String>,
}
impl SubDomainSettingBuilder {
    /// <p> The prefix setting for the subdomain. </p>
    pub fn prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The prefix setting for the subdomain. </p>
    pub fn set_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// <p> The branch name setting for the subdomain. </p>
    pub fn branch_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.branch_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The branch name setting for the subdomain. </p>
    pub fn set_branch_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.branch_name = input;
        self
    }
    /// Consumes the builder and constructs a [`SubDomainSetting`](crate::types::SubDomainSetting).
    pub fn build(self) -> crate::types::SubDomainSetting {
        crate::types::SubDomainSetting {
            prefix: self.prefix,
            branch_name: self.branch_name,
        }
    }
}
