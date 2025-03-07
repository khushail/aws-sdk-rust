// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetStreamingDistribution`](crate::operation::get_streaming_distribution::builders::GetStreamingDistributionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::get_streaming_distribution::builders::GetStreamingDistributionFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::get_streaming_distribution::builders::GetStreamingDistributionFluentBuilder::set_id): <p>The streaming distribution's ID.</p>
    /// - On success, responds with [`GetStreamingDistributionOutput`](crate::operation::get_streaming_distribution::GetStreamingDistributionOutput) with field(s):
    ///   - [`streaming_distribution(Option<StreamingDistribution>)`](crate::operation::get_streaming_distribution::GetStreamingDistributionOutput::streaming_distribution): <p>The streaming distribution's information.</p>
    ///   - [`e_tag(Option<String>)`](crate::operation::get_streaming_distribution::GetStreamingDistributionOutput::e_tag): <p>The current version of the streaming distribution's information. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    /// - On failure, responds with [`SdkError<GetStreamingDistributionError>`](crate::operation::get_streaming_distribution::GetStreamingDistributionError)
    pub fn get_streaming_distribution(
        &self,
    ) -> crate::operation::get_streaming_distribution::builders::GetStreamingDistributionFluentBuilder
    {
        crate::operation::get_streaming_distribution::builders::GetStreamingDistributionFluentBuilder::new(self.handle.clone())
    }
}
