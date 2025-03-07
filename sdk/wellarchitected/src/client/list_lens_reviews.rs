// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListLensReviews`](crate::operation::list_lens_reviews::builders::ListLensReviewsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_lens_reviews::builders::ListLensReviewsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`workload_id(impl ::std::convert::Into<String>)`](crate::operation::list_lens_reviews::builders::ListLensReviewsFluentBuilder::workload_id) / [`set_workload_id(Option<String>)`](crate::operation::list_lens_reviews::builders::ListLensReviewsFluentBuilder::set_workload_id): <p>The ID assigned to the workload. This ID is unique within an Amazon Web Services Region.</p>
    ///   - [`milestone_number(i32)`](crate::operation::list_lens_reviews::builders::ListLensReviewsFluentBuilder::milestone_number) / [`set_milestone_number(Option<i32>)`](crate::operation::list_lens_reviews::builders::ListLensReviewsFluentBuilder::set_milestone_number): <p>The milestone number.</p>  <p>A workload can have a maximum of 100 milestones.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_lens_reviews::builders::ListLensReviewsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_lens_reviews::builders::ListLensReviewsFluentBuilder::set_next_token): <p>The token to use to retrieve the next set of results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_lens_reviews::builders::ListLensReviewsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_lens_reviews::builders::ListLensReviewsFluentBuilder::set_max_results): <p>The maximum number of results to return for this request.</p>
    /// - On success, responds with [`ListLensReviewsOutput`](crate::operation::list_lens_reviews::ListLensReviewsOutput) with field(s):
    ///   - [`workload_id(Option<String>)`](crate::operation::list_lens_reviews::ListLensReviewsOutput::workload_id): <p>The ID assigned to the workload. This ID is unique within an Amazon Web Services Region.</p>
    ///   - [`milestone_number(i32)`](crate::operation::list_lens_reviews::ListLensReviewsOutput::milestone_number): <p>The milestone number.</p>  <p>A workload can have a maximum of 100 milestones.</p>
    ///   - [`lens_review_summaries(Option<Vec<LensReviewSummary>>)`](crate::operation::list_lens_reviews::ListLensReviewsOutput::lens_review_summaries): <p>List of lens summaries of lens reviews of a workload.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_lens_reviews::ListLensReviewsOutput::next_token): <p>The token to use to retrieve the next set of results.</p>
    /// - On failure, responds with [`SdkError<ListLensReviewsError>`](crate::operation::list_lens_reviews::ListLensReviewsError)
    pub fn list_lens_reviews(
        &self,
    ) -> crate::operation::list_lens_reviews::builders::ListLensReviewsFluentBuilder {
        crate::operation::list_lens_reviews::builders::ListLensReviewsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
