// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeliverConfigSnapshot`](crate::operation::deliver_config_snapshot::builders::DeliverConfigSnapshotFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`delivery_channel_name(impl ::std::convert::Into<String>)`](crate::operation::deliver_config_snapshot::builders::DeliverConfigSnapshotFluentBuilder::delivery_channel_name) / [`set_delivery_channel_name(Option<String>)`](crate::operation::deliver_config_snapshot::builders::DeliverConfigSnapshotFluentBuilder::set_delivery_channel_name): <p>The name of the delivery channel through which the snapshot is delivered.</p>
    /// - On success, responds with [`DeliverConfigSnapshotOutput`](crate::operation::deliver_config_snapshot::DeliverConfigSnapshotOutput) with field(s):
    ///   - [`config_snapshot_id(Option<String>)`](crate::operation::deliver_config_snapshot::DeliverConfigSnapshotOutput::config_snapshot_id): <p>The ID of the snapshot that is being created.</p>
    /// - On failure, responds with [`SdkError<DeliverConfigSnapshotError>`](crate::operation::deliver_config_snapshot::DeliverConfigSnapshotError)
    pub fn deliver_config_snapshot(
        &self,
    ) -> crate::operation::deliver_config_snapshot::builders::DeliverConfigSnapshotFluentBuilder
    {
        crate::operation::deliver_config_snapshot::builders::DeliverConfigSnapshotFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
