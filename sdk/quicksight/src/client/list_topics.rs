// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTopics`](crate::operation::list_topics::builders::ListTopicsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_topics::builders::ListTopicsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::list_topics::builders::ListTopicsFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::list_topics::builders::ListTopicsFluentBuilder::set_aws_account_id): <p>The ID of the Amazon Web Services account that contains the topics that you want to list.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_topics::builders::ListTopicsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_topics::builders::ListTopicsFluentBuilder::set_next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_topics::builders::ListTopicsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_topics::builders::ListTopicsFluentBuilder::set_max_results): <p>The maximum number of results to be returned per request.</p>
    /// - On success, responds with [`ListTopicsOutput`](crate::operation::list_topics::ListTopicsOutput) with field(s):
    ///   - [`topics_summaries(Option<Vec<TopicSummary>>)`](crate::operation::list_topics::ListTopicsOutput::topics_summaries): <p>A list of topic summaries.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_topics::ListTopicsOutput::next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::list_topics::ListTopicsOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::list_topics::ListTopicsOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<ListTopicsError>`](crate::operation::list_topics::ListTopicsError)
    pub fn list_topics(&self) -> crate::operation::list_topics::builders::ListTopicsFluentBuilder {
        crate::operation::list_topics::builders::ListTopicsFluentBuilder::new(self.handle.clone())
    }
}
