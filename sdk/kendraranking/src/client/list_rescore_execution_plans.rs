// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListRescoreExecutionPlans`](crate::operation::list_rescore_execution_plans::builders::ListRescoreExecutionPlansFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_rescore_execution_plans::builders::ListRescoreExecutionPlansFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_rescore_execution_plans::builders::ListRescoreExecutionPlansFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_rescore_execution_plans::builders::ListRescoreExecutionPlansFluentBuilder::set_next_token): <p>If the response is truncated, Amazon Kendra Intelligent Ranking returns a pagination token in the response. You can use this pagination token to retrieve the next set of rescore execution plans.</p>
    ///   - [`max_results(i32)`](crate::operation::list_rescore_execution_plans::builders::ListRescoreExecutionPlansFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_rescore_execution_plans::builders::ListRescoreExecutionPlansFluentBuilder::set_max_results): <p>The maximum number of rescore execution plans to return.</p>
    /// - On success, responds with [`ListRescoreExecutionPlansOutput`](crate::operation::list_rescore_execution_plans::ListRescoreExecutionPlansOutput) with field(s):
    ///   - [`summary_items(Option<Vec<RescoreExecutionPlanSummary>>)`](crate::operation::list_rescore_execution_plans::ListRescoreExecutionPlansOutput::summary_items): <p>An array of summary information for one or more rescore execution plans.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_rescore_execution_plans::ListRescoreExecutionPlansOutput::next_token): <p>If the response is truncated, Amazon Kendra Intelligent Ranking returns a pagination token in the response.</p>
    /// - On failure, responds with [`SdkError<ListRescoreExecutionPlansError>`](crate::operation::list_rescore_execution_plans::ListRescoreExecutionPlansError)
    pub fn list_rescore_execution_plans(&self) -> crate::operation::list_rescore_execution_plans::builders::ListRescoreExecutionPlansFluentBuilder{
        crate::operation::list_rescore_execution_plans::builders::ListRescoreExecutionPlansFluentBuilder::new(self.handle.clone())
    }
}
