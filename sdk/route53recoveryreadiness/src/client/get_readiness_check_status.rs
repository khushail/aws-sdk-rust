// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetReadinessCheckStatus`](crate::operation::get_readiness_check_status::builders::GetReadinessCheckStatusFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_readiness_check_status::builders::GetReadinessCheckStatusFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::get_readiness_check_status::builders::GetReadinessCheckStatusFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_readiness_check_status::builders::GetReadinessCheckStatusFluentBuilder::set_max_results): <p>The number of objects that you want to return with this call.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::get_readiness_check_status::builders::GetReadinessCheckStatusFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_readiness_check_status::builders::GetReadinessCheckStatusFluentBuilder::set_next_token): <p>The token that identifies which batch of results you want to see.</p>
    ///   - [`readiness_check_name(impl ::std::convert::Into<String>)`](crate::operation::get_readiness_check_status::builders::GetReadinessCheckStatusFluentBuilder::readiness_check_name) / [`set_readiness_check_name(Option<String>)`](crate::operation::get_readiness_check_status::builders::GetReadinessCheckStatusFluentBuilder::set_readiness_check_name): <p>Name of a readiness check.</p>
    /// - On success, responds with [`GetReadinessCheckStatusOutput`](crate::operation::get_readiness_check_status::GetReadinessCheckStatusOutput) with field(s):
    ///   - [`messages(Option<Vec<Message>>)`](crate::operation::get_readiness_check_status::GetReadinessCheckStatusOutput::messages): <p>Top level messages for readiness check status</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_readiness_check_status::GetReadinessCheckStatusOutput::next_token): <p>The token that identifies which batch of results you want to see.</p>
    ///   - [`readiness(Option<Readiness>)`](crate::operation::get_readiness_check_status::GetReadinessCheckStatusOutput::readiness): <p>The readiness at rule level.</p>
    ///   - [`resources(Option<Vec<ResourceResult>>)`](crate::operation::get_readiness_check_status::GetReadinessCheckStatusOutput::resources): <p>Summary of the readiness of resources.</p>
    /// - On failure, responds with [`SdkError<GetReadinessCheckStatusError>`](crate::operation::get_readiness_check_status::GetReadinessCheckStatusError)
    pub fn get_readiness_check_status(
        &self,
    ) -> crate::operation::get_readiness_check_status::builders::GetReadinessCheckStatusFluentBuilder
    {
        crate::operation::get_readiness_check_status::builders::GetReadinessCheckStatusFluentBuilder::new(self.handle.clone())
    }
}
