// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeEvents`](crate::operation::describe_events::builders::DescribeEventsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`source_name(impl ::std::convert::Into<String>)`](crate::operation::describe_events::builders::DescribeEventsFluentBuilder::source_name) / [`set_source_name(Option<String>)`](crate::operation::describe_events::builders::DescribeEventsFluentBuilder::set_source_name): <p>The identifier of the event source for which events will be returned. If not specified, then all sources are included in the response.</p>
    ///   - [`source_type(SourceType)`](crate::operation::describe_events::builders::DescribeEventsFluentBuilder::source_type) / [`set_source_type(Option<SourceType>)`](crate::operation::describe_events::builders::DescribeEventsFluentBuilder::set_source_type): <p>The event source to retrieve events for. If no value is specified, all events are returned.</p>
    ///   - [`start_time(DateTime)`](crate::operation::describe_events::builders::DescribeEventsFluentBuilder::start_time) / [`set_start_time(Option<DateTime>)`](crate::operation::describe_events::builders::DescribeEventsFluentBuilder::set_start_time): <p>The beginning of the time interval to retrieve events for, specified in ISO 8601 format.</p>
    ///   - [`end_time(DateTime)`](crate::operation::describe_events::builders::DescribeEventsFluentBuilder::end_time) / [`set_end_time(Option<DateTime>)`](crate::operation::describe_events::builders::DescribeEventsFluentBuilder::set_end_time): <p>The end of the time interval for which to retrieve events, specified in ISO 8601 format.</p>
    ///   - [`duration(i32)`](crate::operation::describe_events::builders::DescribeEventsFluentBuilder::duration) / [`set_duration(Option<i32>)`](crate::operation::describe_events::builders::DescribeEventsFluentBuilder::set_duration): <p>The number of minutes' worth of events to retrieve.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_events::builders::DescribeEventsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_events::builders::DescribeEventsFluentBuilder::set_max_results): <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>  <p>The value for <code>MaxResults</code> must be between 20 and 100.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_events::builders::DescribeEventsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_events::builders::DescribeEventsFluentBuilder::set_next_token): <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    /// - On success, responds with [`DescribeEventsOutput`](crate::operation::describe_events::DescribeEventsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::describe_events::DescribeEventsOutput::next_token): <p>Provides an identifier to allow retrieval of paginated results.</p>
    ///   - [`events(Option<Vec<Event>>)`](crate::operation::describe_events::DescribeEventsOutput::events): <p>An array of events. Each element in the array represents one event.</p>
    /// - On failure, responds with [`SdkError<DescribeEventsError>`](crate::operation::describe_events::DescribeEventsError)
    pub fn describe_events(
        &self,
    ) -> crate::operation::describe_events::builders::DescribeEventsFluentBuilder {
        crate::operation::describe_events::builders::DescribeEventsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
