// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetIntents`](crate::operation::get_intents::builders::GetIntentsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_intents::builders::GetIntentsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::get_intents::builders::GetIntentsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_intents::builders::GetIntentsFluentBuilder::set_next_token): <p>A pagination token that fetches the next page of intents. If the response to this API call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of intents, specify the pagination token in the next request. </p>
    ///   - [`max_results(i32)`](crate::operation::get_intents::builders::GetIntentsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_intents::builders::GetIntentsFluentBuilder::set_max_results): <p>The maximum number of intents to return in the response. The default is 10.</p>
    ///   - [`name_contains(impl ::std::convert::Into<String>)`](crate::operation::get_intents::builders::GetIntentsFluentBuilder::name_contains) / [`set_name_contains(Option<String>)`](crate::operation::get_intents::builders::GetIntentsFluentBuilder::set_name_contains): <p>Substring to match in intent names. An intent will be returned if any part of its name matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz."</p>
    /// - On success, responds with [`GetIntentsOutput`](crate::operation::get_intents::GetIntentsOutput) with field(s):
    ///   - [`intents(Option<Vec<IntentMetadata>>)`](crate::operation::get_intents::GetIntentsOutput::intents): <p>An array of <code>Intent</code> objects. For more information, see <code>PutBot</code>.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_intents::GetIntentsOutput::next_token): <p>If the response is truncated, the response includes a pagination token that you can specify in your next request to fetch the next page of intents. </p>
    /// - On failure, responds with [`SdkError<GetIntentsError>`](crate::operation::get_intents::GetIntentsError)
    pub fn get_intents(&self) -> crate::operation::get_intents::builders::GetIntentsFluentBuilder {
        crate::operation::get_intents::builders::GetIntentsFluentBuilder::new(self.handle.clone())
    }
}
