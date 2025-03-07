// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociatePricingRulesInput {
    /// <p> The <code>PricingPlanArn</code> that the <code>PricingRuleArns</code> are associated with. </p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p> The <code>PricingRuleArns</code> that are associated with the Pricing Plan. </p>
    #[doc(hidden)]
    pub pricing_rule_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AssociatePricingRulesInput {
    /// <p> The <code>PricingPlanArn</code> that the <code>PricingRuleArns</code> are associated with. </p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p> The <code>PricingRuleArns</code> that are associated with the Pricing Plan. </p>
    pub fn pricing_rule_arns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.pricing_rule_arns.as_deref()
    }
}
impl AssociatePricingRulesInput {
    /// Creates a new builder-style object to manufacture [`AssociatePricingRulesInput`](crate::operation::associate_pricing_rules::AssociatePricingRulesInput).
    pub fn builder(
    ) -> crate::operation::associate_pricing_rules::builders::AssociatePricingRulesInputBuilder
    {
        crate::operation::associate_pricing_rules::builders::AssociatePricingRulesInputBuilder::default()
    }
}

/// A builder for [`AssociatePricingRulesInput`](crate::operation::associate_pricing_rules::AssociatePricingRulesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AssociatePricingRulesInputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) pricing_rule_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AssociatePricingRulesInputBuilder {
    /// <p> The <code>PricingPlanArn</code> that the <code>PricingRuleArns</code> are associated with. </p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The <code>PricingPlanArn</code> that the <code>PricingRuleArns</code> are associated with. </p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Appends an item to `pricing_rule_arns`.
    ///
    /// To override the contents of this collection use [`set_pricing_rule_arns`](Self::set_pricing_rule_arns).
    ///
    /// <p> The <code>PricingRuleArns</code> that are associated with the Pricing Plan. </p>
    pub fn pricing_rule_arns(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.pricing_rule_arns.unwrap_or_default();
        v.push(input.into());
        self.pricing_rule_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p> The <code>PricingRuleArns</code> that are associated with the Pricing Plan. </p>
    pub fn set_pricing_rule_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.pricing_rule_arns = input;
        self
    }
    /// Consumes the builder and constructs a [`AssociatePricingRulesInput`](crate::operation::associate_pricing_rules::AssociatePricingRulesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::associate_pricing_rules::AssociatePricingRulesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::associate_pricing_rules::AssociatePricingRulesInput {
                arn: self.arn,
                pricing_rule_arns: self.pricing_rule_arns,
            },
        )
    }
}
