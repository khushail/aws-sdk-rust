// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeProjects`](crate::operation::describe_projects::builders::DescribeProjectsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_projects::builders::DescribeProjectsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_projects::builders::DescribeProjectsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_projects::builders::DescribeProjectsFluentBuilder::set_next_token): <p>If the previous response was incomplete (because there is more results to retrieve), Amazon Rekognition Custom Labels returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
    ///   - [`max_results(i32)`](crate::operation::describe_projects::builders::DescribeProjectsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_projects::builders::DescribeProjectsFluentBuilder::set_max_results): <p>The maximum number of results to return per paginated call. The largest value you can specify is 100. If you specify a value greater than 100, a ValidationException error occurs. The default value is 100. </p>
    ///   - [`project_names(Vec<String>)`](crate::operation::describe_projects::builders::DescribeProjectsFluentBuilder::project_names) / [`set_project_names(Option<Vec<String>>)`](crate::operation::describe_projects::builders::DescribeProjectsFluentBuilder::set_project_names): <p>A list of the projects that you want Amazon Rekognition Custom Labels to describe. If you don't specify a value, the response includes descriptions for all the projects in your AWS account.</p>
    /// - On success, responds with [`DescribeProjectsOutput`](crate::operation::describe_projects::DescribeProjectsOutput) with field(s):
    ///   - [`project_descriptions(Option<Vec<ProjectDescription>>)`](crate::operation::describe_projects::DescribeProjectsOutput::project_descriptions): <p>A list of project descriptions. The list is sorted by the date and time the projects are created.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_projects::DescribeProjectsOutput::next_token): <p>If the previous response was incomplete (because there is more results to retrieve), Amazon Rekognition Custom Labels returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
    /// - On failure, responds with [`SdkError<DescribeProjectsError>`](crate::operation::describe_projects::DescribeProjectsError)
    pub fn describe_projects(
        &self,
    ) -> crate::operation::describe_projects::builders::DescribeProjectsFluentBuilder {
        crate::operation::describe_projects::builders::DescribeProjectsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
