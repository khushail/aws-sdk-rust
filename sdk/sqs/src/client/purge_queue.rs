// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PurgeQueue`](crate::operation::purge_queue::builders::PurgeQueueFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`queue_url(impl ::std::convert::Into<String>)`](crate::operation::purge_queue::builders::PurgeQueueFluentBuilder::queue_url) / [`set_queue_url(Option<String>)`](crate::operation::purge_queue::builders::PurgeQueueFluentBuilder::set_queue_url): <p>The URL of the queue from which the <code>PurgeQueue</code> action deletes messages.</p>  <p>Queue URLs and names are case-sensitive.</p>
    /// - On success, responds with [`PurgeQueueOutput`](crate::operation::purge_queue::PurgeQueueOutput)
    /// - On failure, responds with [`SdkError<PurgeQueueError>`](crate::operation::purge_queue::PurgeQueueError)
    pub fn purge_queue(&self) -> crate::operation::purge_queue::builders::PurgeQueueFluentBuilder {
        crate::operation::purge_queue::builders::PurgeQueueFluentBuilder::new(self.handle.clone())
    }
}
