// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetLatestAssessmentIdOutput {
    /// <p>The latest ID for the specific assessment task.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetLatestAssessmentIdOutput {
    /// <p>The latest ID for the specific assessment task.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetLatestAssessmentIdOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetLatestAssessmentIdOutput {
    /// Creates a new builder-style object to manufacture [`GetLatestAssessmentIdOutput`](crate::operation::get_latest_assessment_id::GetLatestAssessmentIdOutput).
    pub fn builder(
    ) -> crate::operation::get_latest_assessment_id::builders::GetLatestAssessmentIdOutputBuilder
    {
        crate::operation::get_latest_assessment_id::builders::GetLatestAssessmentIdOutputBuilder::default()
    }
}

/// A builder for [`GetLatestAssessmentIdOutput`](crate::operation::get_latest_assessment_id::GetLatestAssessmentIdOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetLatestAssessmentIdOutputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetLatestAssessmentIdOutputBuilder {
    /// <p>The latest ID for the specific assessment task.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The latest ID for the specific assessment task.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
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
    /// Consumes the builder and constructs a [`GetLatestAssessmentIdOutput`](crate::operation::get_latest_assessment_id::GetLatestAssessmentIdOutput).
    pub fn build(self) -> crate::operation::get_latest_assessment_id::GetLatestAssessmentIdOutput {
        crate::operation::get_latest_assessment_id::GetLatestAssessmentIdOutput {
            id: self.id,
            _request_id: self._request_id,
        }
    }
}
