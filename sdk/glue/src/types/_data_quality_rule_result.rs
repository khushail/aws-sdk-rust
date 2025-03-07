// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the result of the evaluation of a data quality rule.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DataQualityRuleResult {
    /// <p>The name of the data quality rule.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>A description of the data quality rule.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>An evaluation message.</p>
    #[doc(hidden)]
    pub evaluation_message: ::std::option::Option<::std::string::String>,
    /// <p>A pass or fail status for the rule.</p>
    #[doc(hidden)]
    pub result: ::std::option::Option<crate::types::DataQualityRuleResultStatus>,
}
impl DataQualityRuleResult {
    /// <p>The name of the data quality rule.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>A description of the data quality rule.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>An evaluation message.</p>
    pub fn evaluation_message(&self) -> ::std::option::Option<&str> {
        self.evaluation_message.as_deref()
    }
    /// <p>A pass or fail status for the rule.</p>
    pub fn result(&self) -> ::std::option::Option<&crate::types::DataQualityRuleResultStatus> {
        self.result.as_ref()
    }
}
impl DataQualityRuleResult {
    /// Creates a new builder-style object to manufacture [`DataQualityRuleResult`](crate::types::DataQualityRuleResult).
    pub fn builder() -> crate::types::builders::DataQualityRuleResultBuilder {
        crate::types::builders::DataQualityRuleResultBuilder::default()
    }
}

/// A builder for [`DataQualityRuleResult`](crate::types::DataQualityRuleResult).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DataQualityRuleResultBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) evaluation_message: ::std::option::Option<::std::string::String>,
    pub(crate) result: ::std::option::Option<crate::types::DataQualityRuleResultStatus>,
}
impl DataQualityRuleResultBuilder {
    /// <p>The name of the data quality rule.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the data quality rule.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>A description of the data quality rule.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the data quality rule.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>An evaluation message.</p>
    pub fn evaluation_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.evaluation_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An evaluation message.</p>
    pub fn set_evaluation_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.evaluation_message = input;
        self
    }
    /// <p>A pass or fail status for the rule.</p>
    pub fn result(mut self, input: crate::types::DataQualityRuleResultStatus) -> Self {
        self.result = ::std::option::Option::Some(input);
        self
    }
    /// <p>A pass or fail status for the rule.</p>
    pub fn set_result(
        mut self,
        input: ::std::option::Option<crate::types::DataQualityRuleResultStatus>,
    ) -> Self {
        self.result = input;
        self
    }
    /// Consumes the builder and constructs a [`DataQualityRuleResult`](crate::types::DataQualityRuleResult).
    pub fn build(self) -> crate::types::DataQualityRuleResult {
        crate::types::DataQualityRuleResult {
            name: self.name,
            description: self.description,
            evaluation_message: self.evaluation_message,
            result: self.result,
        }
    }
}
