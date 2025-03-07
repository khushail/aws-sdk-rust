// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDiscoveryJobs`](crate::operation::list_discovery_jobs::builders::ListDiscoveryJobsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_discovery_jobs::builders::ListDiscoveryJobsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`storage_system_arn(impl ::std::convert::Into<String>)`](crate::operation::list_discovery_jobs::builders::ListDiscoveryJobsFluentBuilder::storage_system_arn) / [`set_storage_system_arn(Option<String>)`](crate::operation::list_discovery_jobs::builders::ListDiscoveryJobsFluentBuilder::set_storage_system_arn): <p>Specifies the Amazon Resource Name (ARN) of an on-premises storage system. Use this parameter if you only want to list the discovery jobs that are associated with a specific storage system.</p>
    ///   - [`max_results(i32)`](crate::operation::list_discovery_jobs::builders::ListDiscoveryJobsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_discovery_jobs::builders::ListDiscoveryJobsFluentBuilder::set_max_results): <p>Specifies how many results you want in the response.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_discovery_jobs::builders::ListDiscoveryJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_discovery_jobs::builders::ListDiscoveryJobsFluentBuilder::set_next_token): <p>Specifies an opaque string that indicates the position to begin the next list of results in the response.</p>
    /// - On success, responds with [`ListDiscoveryJobsOutput`](crate::operation::list_discovery_jobs::ListDiscoveryJobsOutput) with field(s):
    ///   - [`discovery_jobs(Option<Vec<DiscoveryJobListEntry>>)`](crate::operation::list_discovery_jobs::ListDiscoveryJobsOutput::discovery_jobs): <p>The discovery jobs that you've run.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_discovery_jobs::ListDiscoveryJobsOutput::next_token): <p>The opaque string that indicates the position to begin the next list of results in the response.</p>
    /// - On failure, responds with [`SdkError<ListDiscoveryJobsError>`](crate::operation::list_discovery_jobs::ListDiscoveryJobsError)
    pub fn list_discovery_jobs(
        &self,
    ) -> crate::operation::list_discovery_jobs::builders::ListDiscoveryJobsFluentBuilder {
        crate::operation::list_discovery_jobs::builders::ListDiscoveryJobsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
