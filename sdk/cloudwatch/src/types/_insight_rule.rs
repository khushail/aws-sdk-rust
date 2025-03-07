// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This structure contains the definition for a Contributor Insights rule. For more information about this rule, see<a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/ContributorInsights.html"> Using Constributor Insights to analyze high-cardinality data</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InsightRule {
    /// <p>The name of the rule.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether the rule is enabled or disabled.</p>
    #[doc(hidden)]
    pub state: ::std::option::Option<::std::string::String>,
    /// <p>For rules that you create, this is always <code>{"Name": "CloudWatchLogRule", "Version": 1}</code>. For managed rules, this is <code>{"Name": "ServiceLogRule", "Version": 1}</code> </p>
    #[doc(hidden)]
    pub schema: ::std::option::Option<::std::string::String>,
    /// <p>The definition of the rule, as a JSON object. The definition contains the keywords used to define contributors, the value to aggregate on if this rule returns a sum instead of a count, and the filters. For details on the valid syntax, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/ContributorInsights-RuleSyntax.html">Contributor Insights Rule Syntax</a>.</p>
    #[doc(hidden)]
    pub definition: ::std::option::Option<::std::string::String>,
    /// <p> An optional built-in rule that Amazon Web Services manages. </p>
    #[doc(hidden)]
    pub managed_rule: bool,
}
impl InsightRule {
    /// <p>The name of the rule.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Indicates whether the rule is enabled or disabled.</p>
    pub fn state(&self) -> ::std::option::Option<&str> {
        self.state.as_deref()
    }
    /// <p>For rules that you create, this is always <code>{"Name": "CloudWatchLogRule", "Version": 1}</code>. For managed rules, this is <code>{"Name": "ServiceLogRule", "Version": 1}</code> </p>
    pub fn schema(&self) -> ::std::option::Option<&str> {
        self.schema.as_deref()
    }
    /// <p>The definition of the rule, as a JSON object. The definition contains the keywords used to define contributors, the value to aggregate on if this rule returns a sum instead of a count, and the filters. For details on the valid syntax, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/ContributorInsights-RuleSyntax.html">Contributor Insights Rule Syntax</a>.</p>
    pub fn definition(&self) -> ::std::option::Option<&str> {
        self.definition.as_deref()
    }
    /// <p> An optional built-in rule that Amazon Web Services manages. </p>
    pub fn managed_rule(&self) -> bool {
        self.managed_rule
    }
}
impl InsightRule {
    /// Creates a new builder-style object to manufacture [`InsightRule`](crate::types::InsightRule).
    pub fn builder() -> crate::types::builders::InsightRuleBuilder {
        crate::types::builders::InsightRuleBuilder::default()
    }
}

/// A builder for [`InsightRule`](crate::types::InsightRule).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct InsightRuleBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<::std::string::String>,
    pub(crate) schema: ::std::option::Option<::std::string::String>,
    pub(crate) definition: ::std::option::Option<::std::string::String>,
    pub(crate) managed_rule: ::std::option::Option<bool>,
}
impl InsightRuleBuilder {
    /// <p>The name of the rule.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the rule.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Indicates whether the rule is enabled or disabled.</p>
    pub fn state(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.state = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Indicates whether the rule is enabled or disabled.</p>
    pub fn set_state(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.state = input;
        self
    }
    /// <p>For rules that you create, this is always <code>{"Name": "CloudWatchLogRule", "Version": 1}</code>. For managed rules, this is <code>{"Name": "ServiceLogRule", "Version": 1}</code> </p>
    pub fn schema(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.schema = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>For rules that you create, this is always <code>{"Name": "CloudWatchLogRule", "Version": 1}</code>. For managed rules, this is <code>{"Name": "ServiceLogRule", "Version": 1}</code> </p>
    pub fn set_schema(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.schema = input;
        self
    }
    /// <p>The definition of the rule, as a JSON object. The definition contains the keywords used to define contributors, the value to aggregate on if this rule returns a sum instead of a count, and the filters. For details on the valid syntax, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/ContributorInsights-RuleSyntax.html">Contributor Insights Rule Syntax</a>.</p>
    pub fn definition(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.definition = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The definition of the rule, as a JSON object. The definition contains the keywords used to define contributors, the value to aggregate on if this rule returns a sum instead of a count, and the filters. For details on the valid syntax, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/ContributorInsights-RuleSyntax.html">Contributor Insights Rule Syntax</a>.</p>
    pub fn set_definition(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.definition = input;
        self
    }
    /// <p> An optional built-in rule that Amazon Web Services manages. </p>
    pub fn managed_rule(mut self, input: bool) -> Self {
        self.managed_rule = ::std::option::Option::Some(input);
        self
    }
    /// <p> An optional built-in rule that Amazon Web Services manages. </p>
    pub fn set_managed_rule(mut self, input: ::std::option::Option<bool>) -> Self {
        self.managed_rule = input;
        self
    }
    /// Consumes the builder and constructs a [`InsightRule`](crate::types::InsightRule).
    pub fn build(self) -> crate::types::InsightRule {
        crate::types::InsightRule {
            name: self.name,
            state: self.state,
            schema: self.schema,
            definition: self.definition,
            managed_rule: self.managed_rule.unwrap_or_default(),
        }
    }
}
