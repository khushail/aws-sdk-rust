// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetDataQualityRuleRecommendationRun`](crate::operation::get_data_quality_rule_recommendation_run::builders::GetDataQualityRuleRecommendationRunFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`run_id(impl ::std::convert::Into<String>)`](crate::operation::get_data_quality_rule_recommendation_run::builders::GetDataQualityRuleRecommendationRunFluentBuilder::run_id) / [`set_run_id(Option<String>)`](crate::operation::get_data_quality_rule_recommendation_run::builders::GetDataQualityRuleRecommendationRunFluentBuilder::set_run_id): <p>The unique run identifier associated with this run.</p>
    /// - On success, responds with [`GetDataQualityRuleRecommendationRunOutput`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunOutput) with field(s):
    ///   - [`run_id(Option<String>)`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunOutput::run_id): <p>The unique run identifier associated with this run.</p>
    ///   - [`data_source(Option<DataSource>)`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunOutput::data_source): <p>The data source (an Glue table) associated with this run.</p>
    ///   - [`role(Option<String>)`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunOutput::role): <p>An IAM role supplied to encrypt the results of the run.</p>
    ///   - [`number_of_workers(Option<i32>)`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunOutput::number_of_workers): <p>The number of <code>G.1X</code> workers to be used in the run. The default is 5.</p>
    ///   - [`timeout(Option<i32>)`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunOutput::timeout): <p>The timeout for a run in minutes. This is the maximum time that a run can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours).</p>
    ///   - [`status(Option<TaskStatusType>)`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunOutput::status): <p>The status for this run.</p>
    ///   - [`error_string(Option<String>)`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunOutput::error_string): <p>The error strings that are associated with the run.</p>
    ///   - [`started_on(Option<DateTime>)`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunOutput::started_on): <p>The date and time when this run started.</p>
    ///   - [`last_modified_on(Option<DateTime>)`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunOutput::last_modified_on): <p>A timestamp. The last point in time when this data quality rule recommendation run was modified.</p>
    ///   - [`completed_on(Option<DateTime>)`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunOutput::completed_on): <p>The date and time when this run was completed.</p>
    ///   - [`execution_time(i32)`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunOutput::execution_time): <p>The amount of time (in seconds) that the run consumed resources.</p>
    ///   - [`recommended_ruleset(Option<String>)`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunOutput::recommended_ruleset): <p>When a start rule recommendation run completes, it creates a recommended ruleset (a set of rules). This member has those rules in Data Quality Definition Language (DQDL) format.</p>
    ///   - [`created_ruleset_name(Option<String>)`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunOutput::created_ruleset_name): <p>The name of the ruleset that was created by the run.</p>
    /// - On failure, responds with [`SdkError<GetDataQualityRuleRecommendationRunError>`](crate::operation::get_data_quality_rule_recommendation_run::GetDataQualityRuleRecommendationRunError)
    pub fn get_data_quality_rule_recommendation_run(&self) -> crate::operation::get_data_quality_rule_recommendation_run::builders::GetDataQualityRuleRecommendationRunFluentBuilder{
        crate::operation::get_data_quality_rule_recommendation_run::builders::GetDataQualityRuleRecommendationRunFluentBuilder::new(self.handle.clone())
    }
}
