// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSolFunctionPackages`](crate::operation::list_sol_function_packages::builders::ListSolFunctionPackagesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_sol_function_packages::builders::ListSolFunctionPackagesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_sol_function_packages::builders::ListSolFunctionPackagesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_sol_function_packages::builders::ListSolFunctionPackagesFluentBuilder::set_max_results): <p>The maximum number of results to include in the response.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_sol_function_packages::builders::ListSolFunctionPackagesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_sol_function_packages::builders::ListSolFunctionPackagesFluentBuilder::set_next_token): <p>The token for the next page of results.</p>
    /// - On success, responds with [`ListSolFunctionPackagesOutput`](crate::operation::list_sol_function_packages::ListSolFunctionPackagesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_sol_function_packages::ListSolFunctionPackagesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    ///   - [`function_packages(Option<Vec<ListSolFunctionPackageInfo>>)`](crate::operation::list_sol_function_packages::ListSolFunctionPackagesOutput::function_packages): <p>Function packages. A function package is a .zip file in CSAR (Cloud Service Archive) format that contains a network function (an ETSI standard telecommunication application) and function package descriptor that uses the TOSCA standard to describe how the network functions should run on your network.</p>
    /// - On failure, responds with [`SdkError<ListSolFunctionPackagesError>`](crate::operation::list_sol_function_packages::ListSolFunctionPackagesError)
    pub fn list_sol_function_packages(
        &self,
    ) -> crate::operation::list_sol_function_packages::builders::ListSolFunctionPackagesFluentBuilder
    {
        crate::operation::list_sol_function_packages::builders::ListSolFunctionPackagesFluentBuilder::new(self.handle.clone())
    }
}
