// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SearchChannels`](crate::operation::search_channels::builders::SearchChannelsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::search_channels::builders::SearchChannelsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`chime_bearer(impl ::std::convert::Into<String>)`](crate::operation::search_channels::builders::SearchChannelsFluentBuilder::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::operation::search_channels::builders::SearchChannelsFluentBuilder::set_chime_bearer): <p>The <code>AppInstanceUserArn</code> of the user making the API call.</p>
    ///   - [`fields(Vec<SearchField>)`](crate::operation::search_channels::builders::SearchChannelsFluentBuilder::fields) / [`set_fields(Option<Vec<SearchField>>)`](crate::operation::search_channels::builders::SearchChannelsFluentBuilder::set_fields): <p>A list of the <code>Field</code> objects in the channel being searched.</p>
    ///   - [`max_results(i32)`](crate::operation::search_channels::builders::SearchChannelsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::search_channels::builders::SearchChannelsFluentBuilder::set_max_results): <p>The maximum number of channels that you want returned.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::search_channels::builders::SearchChannelsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::search_channels::builders::SearchChannelsFluentBuilder::set_next_token): <p>The token returned from previous API requests until the number of channels is reached.</p>
    /// - On success, responds with [`SearchChannelsOutput`](crate::operation::search_channels::SearchChannelsOutput) with field(s):
    ///   - [`channels(Option<Vec<ChannelSummary>>)`](crate::operation::search_channels::SearchChannelsOutput::channels): <p>A list of the channels in the request.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::search_channels::SearchChannelsOutput::next_token): <p>The token returned from previous API responses until the number of channels is reached.</p>
    /// - On failure, responds with [`SdkError<SearchChannelsError>`](crate::operation::search_channels::SearchChannelsError)
    pub fn search_channels(
        &self,
    ) -> crate::operation::search_channels::builders::SearchChannelsFluentBuilder {
        crate::operation::search_channels::builders::SearchChannelsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
