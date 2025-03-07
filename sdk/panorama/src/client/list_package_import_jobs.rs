// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListPackageImportJobs`](crate::operation::list_package_import_jobs::builders::ListPackageImportJobsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_package_import_jobs::builders::ListPackageImportJobsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_package_import_jobs::builders::ListPackageImportJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_package_import_jobs::builders::ListPackageImportJobsFluentBuilder::set_next_token): <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_package_import_jobs::builders::ListPackageImportJobsFluentBuilder::max_results) / [`set_max_results(i32)`](crate::operation::list_package_import_jobs::builders::ListPackageImportJobsFluentBuilder::set_max_results): <p>The maximum number of package import jobs to return in one page of results.</p>
    /// - On success, responds with [`ListPackageImportJobsOutput`](crate::operation::list_package_import_jobs::ListPackageImportJobsOutput) with field(s):
    ///   - [`package_import_jobs(Option<Vec<PackageImportJob>>)`](crate::operation::list_package_import_jobs::ListPackageImportJobsOutput::package_import_jobs): <p>A list of package import jobs.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_package_import_jobs::ListPackageImportJobsOutput::next_token): <p>A pagination token that's included if more results are available.</p>
    /// - On failure, responds with [`SdkError<ListPackageImportJobsError>`](crate::operation::list_package_import_jobs::ListPackageImportJobsError)
    pub fn list_package_import_jobs(
        &self,
    ) -> crate::operation::list_package_import_jobs::builders::ListPackageImportJobsFluentBuilder
    {
        crate::operation::list_package_import_jobs::builders::ListPackageImportJobsFluentBuilder::new(self.handle.clone())
    }
}
