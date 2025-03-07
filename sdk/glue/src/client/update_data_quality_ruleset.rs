// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateDataQualityRuleset`](crate::operation::update_data_quality_ruleset::builders::UpdateDataQualityRulesetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_data_quality_ruleset::builders::UpdateDataQualityRulesetFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_data_quality_ruleset::builders::UpdateDataQualityRulesetFluentBuilder::set_name): <p>The name of the data quality ruleset.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_data_quality_ruleset::builders::UpdateDataQualityRulesetFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_data_quality_ruleset::builders::UpdateDataQualityRulesetFluentBuilder::set_description): <p>A description of the ruleset.</p>
    ///   - [`ruleset(impl ::std::convert::Into<String>)`](crate::operation::update_data_quality_ruleset::builders::UpdateDataQualityRulesetFluentBuilder::ruleset) / [`set_ruleset(Option<String>)`](crate::operation::update_data_quality_ruleset::builders::UpdateDataQualityRulesetFluentBuilder::set_ruleset): <p>A Data Quality Definition Language (DQDL) ruleset. For more information, see the Glue developer guide.</p>
    /// - On success, responds with [`UpdateDataQualityRulesetOutput`](crate::operation::update_data_quality_ruleset::UpdateDataQualityRulesetOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::operation::update_data_quality_ruleset::UpdateDataQualityRulesetOutput::name): <p>The name of the data quality ruleset.</p>
    ///   - [`description(Option<String>)`](crate::operation::update_data_quality_ruleset::UpdateDataQualityRulesetOutput::description): <p>A description of the ruleset.</p>
    ///   - [`ruleset(Option<String>)`](crate::operation::update_data_quality_ruleset::UpdateDataQualityRulesetOutput::ruleset): <p>A Data Quality Definition Language (DQDL) ruleset. For more information, see the Glue developer guide.</p>
    /// - On failure, responds with [`SdkError<UpdateDataQualityRulesetError>`](crate::operation::update_data_quality_ruleset::UpdateDataQualityRulesetError)
    pub fn update_data_quality_ruleset(&self) -> crate::operation::update_data_quality_ruleset::builders::UpdateDataQualityRulesetFluentBuilder{
        crate::operation::update_data_quality_ruleset::builders::UpdateDataQualityRulesetFluentBuilder::new(self.handle.clone())
    }
}
