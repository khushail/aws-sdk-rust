// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeChannel`](crate::operation::describe_channel::builders::DescribeChannelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_name(impl ::std::convert::Into<String>)`](crate::operation::describe_channel::builders::DescribeChannelFluentBuilder::channel_name) / [`set_channel_name(Option<String>)`](crate::operation::describe_channel::builders::DescribeChannelFluentBuilder::set_channel_name): <p>The name of the channel whose information is retrieved.</p>
    ///   - [`include_statistics(bool)`](crate::operation::describe_channel::builders::DescribeChannelFluentBuilder::include_statistics) / [`set_include_statistics(bool)`](crate::operation::describe_channel::builders::DescribeChannelFluentBuilder::set_include_statistics): <p>If true, additional statistical information about the channel is included in the response. This feature can't be used with a channel whose S3 storage is customer-managed.</p>
    /// - On success, responds with [`DescribeChannelOutput`](crate::operation::describe_channel::DescribeChannelOutput) with field(s):
    ///   - [`channel(Option<Channel>)`](crate::operation::describe_channel::DescribeChannelOutput::channel): <p>An object that contains information about the channel.</p>
    ///   - [`statistics(Option<ChannelStatistics>)`](crate::operation::describe_channel::DescribeChannelOutput::statistics): <p>Statistics about the channel. Included if the <code>includeStatistics</code> parameter is set to <code>true</code> in the request.</p>
    /// - On failure, responds with [`SdkError<DescribeChannelError>`](crate::operation::describe_channel::DescribeChannelError)
    pub fn describe_channel(
        &self,
    ) -> crate::operation::describe_channel::builders::DescribeChannelFluentBuilder {
        crate::operation::describe_channel::builders::DescribeChannelFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
