// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeFleetAdvisorLsaAnalysisOutput {
    /// <p>A list of <code>FleetAdvisorLsaAnalysisResponse</code> objects.</p>
    #[doc(hidden)]
    pub analysis:
        ::std::option::Option<::std::vec::Vec<crate::types::FleetAdvisorLsaAnalysisResponse>>,
    /// <p>If <code>NextToken</code> is returned, there are more results available. The value of <code>NextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeFleetAdvisorLsaAnalysisOutput {
    /// <p>A list of <code>FleetAdvisorLsaAnalysisResponse</code> objects.</p>
    pub fn analysis(
        &self,
    ) -> ::std::option::Option<&[crate::types::FleetAdvisorLsaAnalysisResponse]> {
        self.analysis.as_deref()
    }
    /// <p>If <code>NextToken</code> is returned, there are more results available. The value of <code>NextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeFleetAdvisorLsaAnalysisOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeFleetAdvisorLsaAnalysisOutput {
    /// Creates a new builder-style object to manufacture [`DescribeFleetAdvisorLsaAnalysisOutput`](crate::operation::describe_fleet_advisor_lsa_analysis::DescribeFleetAdvisorLsaAnalysisOutput).
    pub fn builder() -> crate::operation::describe_fleet_advisor_lsa_analysis::builders::DescribeFleetAdvisorLsaAnalysisOutputBuilder{
        crate::operation::describe_fleet_advisor_lsa_analysis::builders::DescribeFleetAdvisorLsaAnalysisOutputBuilder::default()
    }
}

/// A builder for [`DescribeFleetAdvisorLsaAnalysisOutput`](crate::operation::describe_fleet_advisor_lsa_analysis::DescribeFleetAdvisorLsaAnalysisOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeFleetAdvisorLsaAnalysisOutputBuilder {
    pub(crate) analysis:
        ::std::option::Option<::std::vec::Vec<crate::types::FleetAdvisorLsaAnalysisResponse>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeFleetAdvisorLsaAnalysisOutputBuilder {
    /// Appends an item to `analysis`.
    ///
    /// To override the contents of this collection use [`set_analysis`](Self::set_analysis).
    ///
    /// <p>A list of <code>FleetAdvisorLsaAnalysisResponse</code> objects.</p>
    pub fn analysis(mut self, input: crate::types::FleetAdvisorLsaAnalysisResponse) -> Self {
        let mut v = self.analysis.unwrap_or_default();
        v.push(input);
        self.analysis = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>FleetAdvisorLsaAnalysisResponse</code> objects.</p>
    pub fn set_analysis(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::FleetAdvisorLsaAnalysisResponse>,
        >,
    ) -> Self {
        self.analysis = input;
        self
    }
    /// <p>If <code>NextToken</code> is returned, there are more results available. The value of <code>NextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If <code>NextToken</code> is returned, there are more results available. The value of <code>NextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
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
    /// Consumes the builder and constructs a [`DescribeFleetAdvisorLsaAnalysisOutput`](crate::operation::describe_fleet_advisor_lsa_analysis::DescribeFleetAdvisorLsaAnalysisOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_fleet_advisor_lsa_analysis::DescribeFleetAdvisorLsaAnalysisOutput
    {
        crate::operation::describe_fleet_advisor_lsa_analysis::DescribeFleetAdvisorLsaAnalysisOutput {
            analysis: self.analysis
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}
