// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListEvaluationFormVersionsOutput {
    /// <p>Provides details about a list of evaluation forms belonging to an instance.</p>
    #[doc(hidden)]
    pub evaluation_form_version_summary_list:
        ::std::option::Option<::std::vec::Vec<crate::types::EvaluationFormVersionSummary>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListEvaluationFormVersionsOutput {
    /// <p>Provides details about a list of evaluation forms belonging to an instance.</p>
    pub fn evaluation_form_version_summary_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::EvaluationFormVersionSummary]> {
        self.evaluation_form_version_summary_list.as_deref()
    }
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListEvaluationFormVersionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListEvaluationFormVersionsOutput {
    /// Creates a new builder-style object to manufacture [`ListEvaluationFormVersionsOutput`](crate::operation::list_evaluation_form_versions::ListEvaluationFormVersionsOutput).
    pub fn builder() -> crate::operation::list_evaluation_form_versions::builders::ListEvaluationFormVersionsOutputBuilder{
        crate::operation::list_evaluation_form_versions::builders::ListEvaluationFormVersionsOutputBuilder::default()
    }
}

/// A builder for [`ListEvaluationFormVersionsOutput`](crate::operation::list_evaluation_form_versions::ListEvaluationFormVersionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListEvaluationFormVersionsOutputBuilder {
    pub(crate) evaluation_form_version_summary_list:
        ::std::option::Option<::std::vec::Vec<crate::types::EvaluationFormVersionSummary>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListEvaluationFormVersionsOutputBuilder {
    /// Appends an item to `evaluation_form_version_summary_list`.
    ///
    /// To override the contents of this collection use [`set_evaluation_form_version_summary_list`](Self::set_evaluation_form_version_summary_list).
    ///
    /// <p>Provides details about a list of evaluation forms belonging to an instance.</p>
    pub fn evaluation_form_version_summary_list(
        mut self,
        input: crate::types::EvaluationFormVersionSummary,
    ) -> Self {
        let mut v = self
            .evaluation_form_version_summary_list
            .unwrap_or_default();
        v.push(input);
        self.evaluation_form_version_summary_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>Provides details about a list of evaluation forms belonging to an instance.</p>
    pub fn set_evaluation_form_version_summary_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EvaluationFormVersionSummary>>,
    ) -> Self {
        self.evaluation_form_version_summary_list = input;
        self
    }
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If there are additional results, this is the token for the next set of results.</p>
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
    /// Consumes the builder and constructs a [`ListEvaluationFormVersionsOutput`](crate::operation::list_evaluation_form_versions::ListEvaluationFormVersionsOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_evaluation_form_versions::ListEvaluationFormVersionsOutput {
        crate::operation::list_evaluation_form_versions::ListEvaluationFormVersionsOutput {
            evaluation_form_version_summary_list: self.evaluation_form_version_summary_list,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
