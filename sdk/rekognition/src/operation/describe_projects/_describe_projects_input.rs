// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeProjectsInput {
    /// <p>If the previous response was incomplete (because there is more results to retrieve), Amazon Rekognition Custom Labels returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return per paginated call. The largest value you can specify is 100. If you specify a value greater than 100, a ValidationException error occurs. The default value is 100. </p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>A list of the projects that you want Amazon Rekognition Custom Labels to describe. If you don't specify a value, the response includes descriptions for all the projects in your AWS account.</p>
    #[doc(hidden)]
    pub project_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl DescribeProjectsInput {
    /// <p>If the previous response was incomplete (because there is more results to retrieve), Amazon Rekognition Custom Labels returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to return per paginated call. The largest value you can specify is 100. If you specify a value greater than 100, a ValidationException error occurs. The default value is 100. </p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>A list of the projects that you want Amazon Rekognition Custom Labels to describe. If you don't specify a value, the response includes descriptions for all the projects in your AWS account.</p>
    pub fn project_names(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.project_names.as_deref()
    }
}
impl DescribeProjectsInput {
    /// Creates a new builder-style object to manufacture [`DescribeProjectsInput`](crate::operation::describe_projects::DescribeProjectsInput).
    pub fn builder() -> crate::operation::describe_projects::builders::DescribeProjectsInputBuilder
    {
        crate::operation::describe_projects::builders::DescribeProjectsInputBuilder::default()
    }
}

/// A builder for [`DescribeProjectsInput`](crate::operation::describe_projects::DescribeProjectsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeProjectsInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) project_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl DescribeProjectsInputBuilder {
    /// <p>If the previous response was incomplete (because there is more results to retrieve), Amazon Rekognition Custom Labels returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If the previous response was incomplete (because there is more results to retrieve), Amazon Rekognition Custom Labels returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of results to return per paginated call. The largest value you can specify is 100. If you specify a value greater than 100, a ValidationException error occurs. The default value is 100. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return per paginated call. The largest value you can specify is 100. If you specify a value greater than 100, a ValidationException error occurs. The default value is 100. </p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Appends an item to `project_names`.
    ///
    /// To override the contents of this collection use [`set_project_names`](Self::set_project_names).
    ///
    /// <p>A list of the projects that you want Amazon Rekognition Custom Labels to describe. If you don't specify a value, the response includes descriptions for all the projects in your AWS account.</p>
    pub fn project_names(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.project_names.unwrap_or_default();
        v.push(input.into());
        self.project_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the projects that you want Amazon Rekognition Custom Labels to describe. If you don't specify a value, the response includes descriptions for all the projects in your AWS account.</p>
    pub fn set_project_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.project_names = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeProjectsInput`](crate::operation::describe_projects::DescribeProjectsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_projects::DescribeProjectsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_projects::DescribeProjectsInput {
            next_token: self.next_token,
            max_results: self.max_results,
            project_names: self.project_names,
        })
    }
}
