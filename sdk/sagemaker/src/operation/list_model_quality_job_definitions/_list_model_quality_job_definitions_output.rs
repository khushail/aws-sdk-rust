// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListModelQualityJobDefinitionsOutput {
    /// <p>A list of summaries of model quality monitoring job definitions.</p>
    #[doc(hidden)]
    pub job_definition_summaries:
        ::std::option::Option<::std::vec::Vec<crate::types::MonitoringJobDefinitionSummary>>,
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of model quality monitoring job definitions, use it in the next request.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListModelQualityJobDefinitionsOutput {
    /// <p>A list of summaries of model quality monitoring job definitions.</p>
    pub fn job_definition_summaries(
        &self,
    ) -> ::std::option::Option<&[crate::types::MonitoringJobDefinitionSummary]> {
        self.job_definition_summaries.as_deref()
    }
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of model quality monitoring job definitions, use it in the next request.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListModelQualityJobDefinitionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListModelQualityJobDefinitionsOutput {
    /// Creates a new builder-style object to manufacture [`ListModelQualityJobDefinitionsOutput`](crate::operation::list_model_quality_job_definitions::ListModelQualityJobDefinitionsOutput).
    pub fn builder() -> crate::operation::list_model_quality_job_definitions::builders::ListModelQualityJobDefinitionsOutputBuilder{
        crate::operation::list_model_quality_job_definitions::builders::ListModelQualityJobDefinitionsOutputBuilder::default()
    }
}

/// A builder for [`ListModelQualityJobDefinitionsOutput`](crate::operation::list_model_quality_job_definitions::ListModelQualityJobDefinitionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListModelQualityJobDefinitionsOutputBuilder {
    pub(crate) job_definition_summaries:
        ::std::option::Option<::std::vec::Vec<crate::types::MonitoringJobDefinitionSummary>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListModelQualityJobDefinitionsOutputBuilder {
    /// Appends an item to `job_definition_summaries`.
    ///
    /// To override the contents of this collection use [`set_job_definition_summaries`](Self::set_job_definition_summaries).
    ///
    /// <p>A list of summaries of model quality monitoring job definitions.</p>
    pub fn job_definition_summaries(
        mut self,
        input: crate::types::MonitoringJobDefinitionSummary,
    ) -> Self {
        let mut v = self.job_definition_summaries.unwrap_or_default();
        v.push(input);
        self.job_definition_summaries = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of summaries of model quality monitoring job definitions.</p>
    pub fn set_job_definition_summaries(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MonitoringJobDefinitionSummary>>,
    ) -> Self {
        self.job_definition_summaries = input;
        self
    }
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of model quality monitoring job definitions, use it in the next request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of model quality monitoring job definitions, use it in the next request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListModelQualityJobDefinitionsOutput`](crate::operation::list_model_quality_job_definitions::ListModelQualityJobDefinitionsOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_model_quality_job_definitions::ListModelQualityJobDefinitionsOutput
    {
        crate::operation::list_model_quality_job_definitions::ListModelQualityJobDefinitionsOutput {
            job_definition_summaries: self.job_definition_summaries,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
