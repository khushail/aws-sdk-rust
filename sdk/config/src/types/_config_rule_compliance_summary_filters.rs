// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Filters the results based on the account IDs and regions.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ConfigRuleComplianceSummaryFilters {
    /// <p>The 12-digit account ID of the source account.</p>
    #[doc(hidden)]
    pub account_id: ::std::option::Option<::std::string::String>,
    /// <p>The source region where the data is aggregated.</p>
    #[doc(hidden)]
    pub aws_region: ::std::option::Option<::std::string::String>,
}
impl ConfigRuleComplianceSummaryFilters {
    /// <p>The 12-digit account ID of the source account.</p>
    pub fn account_id(&self) -> ::std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The source region where the data is aggregated.</p>
    pub fn aws_region(&self) -> ::std::option::Option<&str> {
        self.aws_region.as_deref()
    }
}
impl ConfigRuleComplianceSummaryFilters {
    /// Creates a new builder-style object to manufacture [`ConfigRuleComplianceSummaryFilters`](crate::types::ConfigRuleComplianceSummaryFilters).
    pub fn builder() -> crate::types::builders::ConfigRuleComplianceSummaryFiltersBuilder {
        crate::types::builders::ConfigRuleComplianceSummaryFiltersBuilder::default()
    }
}

/// A builder for [`ConfigRuleComplianceSummaryFilters`](crate::types::ConfigRuleComplianceSummaryFilters).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ConfigRuleComplianceSummaryFiltersBuilder {
    pub(crate) account_id: ::std::option::Option<::std::string::String>,
    pub(crate) aws_region: ::std::option::Option<::std::string::String>,
}
impl ConfigRuleComplianceSummaryFiltersBuilder {
    /// <p>The 12-digit account ID of the source account.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The 12-digit account ID of the source account.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The source region where the data is aggregated.</p>
    pub fn aws_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.aws_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The source region where the data is aggregated.</p>
    pub fn set_aws_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.aws_region = input;
        self
    }
    /// Consumes the builder and constructs a [`ConfigRuleComplianceSummaryFilters`](crate::types::ConfigRuleComplianceSummaryFilters).
    pub fn build(self) -> crate::types::ConfigRuleComplianceSummaryFilters {
        crate::types::ConfigRuleComplianceSummaryFilters {
            account_id: self.account_id,
            aws_region: self.aws_region,
        }
    }
}
