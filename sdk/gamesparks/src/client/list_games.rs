// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListGames`](crate::operation::list_games::builders::ListGamesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_games::builders::ListGamesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_games::builders::ListGamesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_games::builders::ListGamesFluentBuilder::set_max_results): <p>The maximum number of results to return.</p>  <p> Use this parameter with NextToken to get results as a set of sequential pages. </p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_games::builders::ListGamesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_games::builders::ListGamesFluentBuilder::set_next_token): <p>The token that indicates the start of the next sequential page of results.</p>  <p> Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value. </p>
    /// - On success, responds with [`ListGamesOutput`](crate::operation::list_games::ListGamesOutput) with field(s):
    ///   - [`games(Option<Vec<GameSummary>>)`](crate::operation::list_games::ListGamesOutput::games): <p>The list of games.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_games::ListGamesOutput::next_token): <p>The token that indicates the start of the next sequential page of results.</p>  <p> Use this value when making the next call to this operation to continue where the last one finished. </p>
    /// - On failure, responds with [`SdkError<ListGamesError>`](crate::operation::list_games::ListGamesError)
    pub fn list_games(&self) -> crate::operation::list_games::builders::ListGamesFluentBuilder {
        crate::operation::list_games::builders::ListGamesFluentBuilder::new(self.handle.clone())
    }
}
