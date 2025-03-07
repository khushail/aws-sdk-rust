// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetUtterancesView`](crate::operation::get_utterances_view::builders::GetUtterancesViewFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bot_name(impl ::std::convert::Into<String>)`](crate::operation::get_utterances_view::builders::GetUtterancesViewFluentBuilder::bot_name) / [`set_bot_name(Option<String>)`](crate::operation::get_utterances_view::builders::GetUtterancesViewFluentBuilder::set_bot_name): <p>The name of the bot for which utterance information should be returned.</p>
    ///   - [`bot_versions(Vec<String>)`](crate::operation::get_utterances_view::builders::GetUtterancesViewFluentBuilder::bot_versions) / [`set_bot_versions(Option<Vec<String>>)`](crate::operation::get_utterances_view::builders::GetUtterancesViewFluentBuilder::set_bot_versions): <p>An array of bot versions for which utterance information should be returned. The limit is 5 versions per request.</p>
    ///   - [`status_type(StatusType)`](crate::operation::get_utterances_view::builders::GetUtterancesViewFluentBuilder::status_type) / [`set_status_type(Option<StatusType>)`](crate::operation::get_utterances_view::builders::GetUtterancesViewFluentBuilder::set_status_type): <p>To return utterances that were recognized and handled, use <code>Detected</code>. To return utterances that were not recognized, use <code>Missed</code>.</p>
    /// - On success, responds with [`GetUtterancesViewOutput`](crate::operation::get_utterances_view::GetUtterancesViewOutput) with field(s):
    ///   - [`bot_name(Option<String>)`](crate::operation::get_utterances_view::GetUtterancesViewOutput::bot_name): <p>The name of the bot for which utterance information was returned.</p>
    ///   - [`utterances(Option<Vec<UtteranceList>>)`](crate::operation::get_utterances_view::GetUtterancesViewOutput::utterances): <p>An array of <code>UtteranceList</code> objects, each containing a list of <code>UtteranceData</code> objects describing the utterances that were processed by your bot. The response contains a maximum of 100 <code>UtteranceData</code> objects for each version. Amazon Lex returns the most frequent utterances received by the bot in the last 15 days.</p>
    /// - On failure, responds with [`SdkError<GetUtterancesViewError>`](crate::operation::get_utterances_view::GetUtterancesViewError)
    pub fn get_utterances_view(
        &self,
    ) -> crate::operation::get_utterances_view::builders::GetUtterancesViewFluentBuilder {
        crate::operation::get_utterances_view::builders::GetUtterancesViewFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
