// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListQueuedMessages`](crate::operation::list_queued_messages::builders::ListQueuedMessagesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_queued_messages::builders::ListQueuedMessagesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::list_queued_messages::builders::ListQueuedMessagesFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::list_queued_messages::builders::ListQueuedMessagesFluentBuilder::set_id): <p>The ID of a given wireless device which the downlink message packets are being sent.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_queued_messages::builders::ListQueuedMessagesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_queued_messages::builders::ListQueuedMessagesFluentBuilder::set_next_token): <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_queued_messages::builders::ListQueuedMessagesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_queued_messages::builders::ListQueuedMessagesFluentBuilder::set_max_results): <p>The maximum number of results to return in this operation.</p>
    ///   - [`wireless_device_type(WirelessDeviceType)`](crate::operation::list_queued_messages::builders::ListQueuedMessagesFluentBuilder::wireless_device_type) / [`set_wireless_device_type(Option<WirelessDeviceType>)`](crate::operation::list_queued_messages::builders::ListQueuedMessagesFluentBuilder::set_wireless_device_type): <p>The wireless device type, whic can be either Sidewalk or LoRaWAN.</p>
    /// - On success, responds with [`ListQueuedMessagesOutput`](crate::operation::list_queued_messages::ListQueuedMessagesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_queued_messages::ListQueuedMessagesOutput::next_token): <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    ///   - [`downlink_queue_messages_list(Option<Vec<DownlinkQueueMessage>>)`](crate::operation::list_queued_messages::ListQueuedMessagesOutput::downlink_queue_messages_list): <p>The messages in the downlink queue.</p>
    /// - On failure, responds with [`SdkError<ListQueuedMessagesError>`](crate::operation::list_queued_messages::ListQueuedMessagesError)
    pub fn list_queued_messages(
        &self,
    ) -> crate::operation::list_queued_messages::builders::ListQueuedMessagesFluentBuilder {
        crate::operation::list_queued_messages::builders::ListQueuedMessagesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
