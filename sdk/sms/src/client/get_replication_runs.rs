// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetReplicationRuns`](crate::operation::get_replication_runs::builders::GetReplicationRunsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_replication_runs::builders::GetReplicationRunsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`replication_job_id(impl ::std::convert::Into<String>)`](crate::operation::get_replication_runs::builders::GetReplicationRunsFluentBuilder::replication_job_id) / [`set_replication_job_id(Option<String>)`](crate::operation::get_replication_runs::builders::GetReplicationRunsFluentBuilder::set_replication_job_id): <p>The ID of the replication job.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::get_replication_runs::builders::GetReplicationRunsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_replication_runs::builders::GetReplicationRunsFluentBuilder::set_next_token): <p>The token for the next set of results.</p>
    ///   - [`max_results(i32)`](crate::operation::get_replication_runs::builders::GetReplicationRunsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_replication_runs::builders::GetReplicationRunsFluentBuilder::set_max_results): <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    /// - On success, responds with [`GetReplicationRunsOutput`](crate::operation::get_replication_runs::GetReplicationRunsOutput) with field(s):
    ///   - [`replication_job(Option<ReplicationJob>)`](crate::operation::get_replication_runs::GetReplicationRunsOutput::replication_job): <p>Information about the replication job.</p>
    ///   - [`replication_run_list(Option<Vec<ReplicationRun>>)`](crate::operation::get_replication_runs::GetReplicationRunsOutput::replication_run_list): <p>Information about the replication runs.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_replication_runs::GetReplicationRunsOutput::next_token): <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<GetReplicationRunsError>`](crate::operation::get_replication_runs::GetReplicationRunsError)
    pub fn get_replication_runs(
        &self,
    ) -> crate::operation::get_replication_runs::builders::GetReplicationRunsFluentBuilder {
        crate::operation::get_replication_runs::builders::GetReplicationRunsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
