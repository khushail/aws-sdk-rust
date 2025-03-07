// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The summary of network settings.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct NetworkSettingsSummary {
    /// <p>The ARN of the network settings.</p>
    #[doc(hidden)]
    pub network_settings_arn: ::std::option::Option<::std::string::String>,
    /// <p>The VPC ID of the network settings.</p>
    #[doc(hidden)]
    pub vpc_id: ::std::option::Option<::std::string::String>,
}
impl NetworkSettingsSummary {
    /// <p>The ARN of the network settings.</p>
    pub fn network_settings_arn(&self) -> ::std::option::Option<&str> {
        self.network_settings_arn.as_deref()
    }
    /// <p>The VPC ID of the network settings.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
}
impl NetworkSettingsSummary {
    /// Creates a new builder-style object to manufacture [`NetworkSettingsSummary`](crate::types::NetworkSettingsSummary).
    pub fn builder() -> crate::types::builders::NetworkSettingsSummaryBuilder {
        crate::types::builders::NetworkSettingsSummaryBuilder::default()
    }
}

/// A builder for [`NetworkSettingsSummary`](crate::types::NetworkSettingsSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct NetworkSettingsSummaryBuilder {
    pub(crate) network_settings_arn: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
}
impl NetworkSettingsSummaryBuilder {
    /// <p>The ARN of the network settings.</p>
    pub fn network_settings_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.network_settings_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the network settings.</p>
    pub fn set_network_settings_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.network_settings_arn = input;
        self
    }
    /// <p>The VPC ID of the network settings.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The VPC ID of the network settings.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// Consumes the builder and constructs a [`NetworkSettingsSummary`](crate::types::NetworkSettingsSummary).
    pub fn build(self) -> crate::types::NetworkSettingsSummary {
        crate::types::NetworkSettingsSummary {
            network_settings_arn: self.network_settings_arn,
            vpc_id: self.vpc_id,
        }
    }
}
