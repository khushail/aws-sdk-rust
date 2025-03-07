// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the input of a <code>ListPipelineExecutions</code> action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListPipelineExecutionsInput {
    /// <p>The name of the pipeline for which you want to get execution summary information.</p>
    #[doc(hidden)]
    pub pipeline_name: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned nextToken value. Pipeline history is limited to the most recent 12 months, based on pipeline execution start times. Default value is 100.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token that was returned from the previous <code>ListPipelineExecutions</code> call, which can be used to return the next set of pipeline executions in the list.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListPipelineExecutionsInput {
    /// <p>The name of the pipeline for which you want to get execution summary information.</p>
    pub fn pipeline_name(&self) -> ::std::option::Option<&str> {
        self.pipeline_name.as_deref()
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned nextToken value. Pipeline history is limited to the most recent 12 months, based on pipeline execution start times. Default value is 100.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token that was returned from the previous <code>ListPipelineExecutions</code> call, which can be used to return the next set of pipeline executions in the list.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListPipelineExecutionsInput {
    /// Creates a new builder-style object to manufacture [`ListPipelineExecutionsInput`](crate::operation::list_pipeline_executions::ListPipelineExecutionsInput).
    pub fn builder(
    ) -> crate::operation::list_pipeline_executions::builders::ListPipelineExecutionsInputBuilder
    {
        crate::operation::list_pipeline_executions::builders::ListPipelineExecutionsInputBuilder::default()
    }
}

/// A builder for [`ListPipelineExecutionsInput`](crate::operation::list_pipeline_executions::ListPipelineExecutionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListPipelineExecutionsInputBuilder {
    pub(crate) pipeline_name: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListPipelineExecutionsInputBuilder {
    /// <p>The name of the pipeline for which you want to get execution summary information.</p>
    pub fn pipeline_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.pipeline_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the pipeline for which you want to get execution summary information.</p>
    pub fn set_pipeline_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.pipeline_name = input;
        self
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned nextToken value. Pipeline history is limited to the most recent 12 months, based on pipeline execution start times. Default value is 100.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned nextToken value. Pipeline history is limited to the most recent 12 months, based on pipeline execution start times. Default value is 100.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The token that was returned from the previous <code>ListPipelineExecutions</code> call, which can be used to return the next set of pipeline executions in the list.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token that was returned from the previous <code>ListPipelineExecutions</code> call, which can be used to return the next set of pipeline executions in the list.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListPipelineExecutionsInput`](crate::operation::list_pipeline_executions::ListPipelineExecutionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_pipeline_executions::ListPipelineExecutionsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_pipeline_executions::ListPipelineExecutionsInput {
                pipeline_name: self.pipeline_name,
                max_results: self.max_results,
                next_token: self.next_token,
            },
        )
    }
}
