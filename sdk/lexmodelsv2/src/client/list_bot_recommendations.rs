// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListBotRecommendations`](crate::operation::list_bot_recommendations::builders::ListBotRecommendationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_bot_recommendations::builders::ListBotRecommendationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`bot_id(impl ::std::convert::Into<String>)`](crate::operation::list_bot_recommendations::builders::ListBotRecommendationsFluentBuilder::bot_id) / [`set_bot_id(Option<String>)`](crate::operation::list_bot_recommendations::builders::ListBotRecommendationsFluentBuilder::set_bot_id): <p>The unique identifier of the bot that contains the bot recommendation list.</p>
    ///   - [`bot_version(impl ::std::convert::Into<String>)`](crate::operation::list_bot_recommendations::builders::ListBotRecommendationsFluentBuilder::bot_version) / [`set_bot_version(Option<String>)`](crate::operation::list_bot_recommendations::builders::ListBotRecommendationsFluentBuilder::set_bot_version): <p>The version of the bot that contains the bot recommendation list.</p>
    ///   - [`locale_id(impl ::std::convert::Into<String>)`](crate::operation::list_bot_recommendations::builders::ListBotRecommendationsFluentBuilder::locale_id) / [`set_locale_id(Option<String>)`](crate::operation::list_bot_recommendations::builders::ListBotRecommendationsFluentBuilder::set_locale_id): <p>The identifier of the language and locale of the bot recommendation list.</p>
    ///   - [`max_results(i32)`](crate::operation::list_bot_recommendations::builders::ListBotRecommendationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_bot_recommendations::builders::ListBotRecommendationsFluentBuilder::set_max_results): <p>The maximum number of bot recommendations to return in each page of results. If there are fewer results than the max page size, only the actual number of results are returned.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_bot_recommendations::builders::ListBotRecommendationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_bot_recommendations::builders::ListBotRecommendationsFluentBuilder::set_next_token): <p>If the response from the ListBotRecommendation operation contains more results than specified in the maxResults parameter, a token is returned in the response. Use that token in the nextToken parameter to return the next page of results.</p>
    /// - On success, responds with [`ListBotRecommendationsOutput`](crate::operation::list_bot_recommendations::ListBotRecommendationsOutput) with field(s):
    ///   - [`bot_id(Option<String>)`](crate::operation::list_bot_recommendations::ListBotRecommendationsOutput::bot_id): <p>The unique identifier of the bot that contains the bot recommendation list.</p>
    ///   - [`bot_version(Option<String>)`](crate::operation::list_bot_recommendations::ListBotRecommendationsOutput::bot_version): <p>The version of the bot that contains the bot recommendation list.</p>
    ///   - [`locale_id(Option<String>)`](crate::operation::list_bot_recommendations::ListBotRecommendationsOutput::locale_id): <p>The identifier of the language and locale of the bot recommendation list.</p>
    ///   - [`bot_recommendation_summaries(Option<Vec<BotRecommendationSummary>>)`](crate::operation::list_bot_recommendations::ListBotRecommendationsOutput::bot_recommendation_summaries): <p>Summary information for the bot recommendations that meet the filter specified in this request. The length of the list is specified in the maxResults parameter of the request. If there are more bot recommendations available, the nextToken field contains a token to get the next page of results.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_bot_recommendations::ListBotRecommendationsOutput::next_token): <p>A token that indicates whether there are more results to return in a response to the ListBotRecommendations operation. If the nextToken field is present, you send the contents as the nextToken parameter of a ListBotRecommendations operation request to get the next page of results. </p>
    /// - On failure, responds with [`SdkError<ListBotRecommendationsError>`](crate::operation::list_bot_recommendations::ListBotRecommendationsError)
    pub fn list_bot_recommendations(
        &self,
    ) -> crate::operation::list_bot_recommendations::builders::ListBotRecommendationsFluentBuilder
    {
        crate::operation::list_bot_recommendations::builders::ListBotRecommendationsFluentBuilder::new(self.handle.clone())
    }
}
