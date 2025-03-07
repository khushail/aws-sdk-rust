// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListWorldExportJobs`](crate::operation::list_world_export_jobs::builders::ListWorldExportJobsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_world_export_jobs::builders::ListWorldExportJobsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_world_export_jobs::builders::ListWorldExportJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_world_export_jobs::builders::ListWorldExportJobsFluentBuilder::set_next_token): <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListWorldExportJobs</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    ///   - [`max_results(i32)`](crate::operation::list_world_export_jobs::builders::ListWorldExportJobsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_world_export_jobs::builders::ListWorldExportJobsFluentBuilder::set_max_results): <p>When this parameter is used, <code>ListWorldExportJobs</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListWorldExportJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListWorldExportJobs</code> returns up to 100 results and a <code>nextToken</code> value if applicable. </p>
    ///   - [`filters(Vec<Filter>)`](crate::operation::list_world_export_jobs::builders::ListWorldExportJobsFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::list_world_export_jobs::builders::ListWorldExportJobsFluentBuilder::set_filters): <p>Optional filters to limit results. You can use <code>generationJobId</code> and <code>templateId</code>.</p>
    /// - On success, responds with [`ListWorldExportJobsOutput`](crate::operation::list_world_export_jobs::ListWorldExportJobsOutput) with field(s):
    ///   - [`world_export_job_summaries(Option<Vec<WorldExportJobSummary>>)`](crate::operation::list_world_export_jobs::ListWorldExportJobsOutput::world_export_job_summaries): <p>Summary information for world export jobs.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_world_export_jobs::ListWorldExportJobsOutput::next_token): <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListWorldExportJobsRequest</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    /// - On failure, responds with [`SdkError<ListWorldExportJobsError>`](crate::operation::list_world_export_jobs::ListWorldExportJobsError)
    pub fn list_world_export_jobs(
        &self,
    ) -> crate::operation::list_world_export_jobs::builders::ListWorldExportJobsFluentBuilder {
        crate::operation::list_world_export_jobs::builders::ListWorldExportJobsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
