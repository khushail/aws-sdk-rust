// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTestRecommendations`](crate::operation::list_test_recommendations::builders::ListTestRecommendationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_test_recommendations::builders::ListTestRecommendationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_test_recommendations::builders::ListTestRecommendationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_test_recommendations::builders::ListTestRecommendationsFluentBuilder::set_next_token): <p>Null, or the token from a previous call to get the next set of results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_test_recommendations::builders::ListTestRecommendationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_test_recommendations::builders::ListTestRecommendationsFluentBuilder::set_max_results): <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>
    ///   - [`assessment_arn(impl ::std::convert::Into<String>)`](crate::operation::list_test_recommendations::builders::ListTestRecommendationsFluentBuilder::assessment_arn) / [`set_assessment_arn(Option<String>)`](crate::operation::list_test_recommendations::builders::ListTestRecommendationsFluentBuilder::set_assessment_arn): <p>The Amazon Resource Name (ARN) of the assessment. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app-assessment/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    /// - On success, responds with [`ListTestRecommendationsOutput`](crate::operation::list_test_recommendations::ListTestRecommendationsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_test_recommendations::ListTestRecommendationsOutput::next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`test_recommendations(Option<Vec<TestRecommendation>>)`](crate::operation::list_test_recommendations::ListTestRecommendationsOutput::test_recommendations): <p>The test recommendations for the Resilience Hub application.</p>
    /// - On failure, responds with [`SdkError<ListTestRecommendationsError>`](crate::operation::list_test_recommendations::ListTestRecommendationsError)
    pub fn list_test_recommendations(
        &self,
    ) -> crate::operation::list_test_recommendations::builders::ListTestRecommendationsFluentBuilder
    {
        crate::operation::list_test_recommendations::builders::ListTestRecommendationsFluentBuilder::new(self.handle.clone())
    }
}
