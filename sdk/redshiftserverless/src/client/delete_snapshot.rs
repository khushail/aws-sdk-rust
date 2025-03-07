// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteSnapshot`](crate::operation::delete_snapshot::builders::DeleteSnapshotFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`snapshot_name(impl ::std::convert::Into<String>)`](crate::operation::delete_snapshot::builders::DeleteSnapshotFluentBuilder::snapshot_name) / [`set_snapshot_name(Option<String>)`](crate::operation::delete_snapshot::builders::DeleteSnapshotFluentBuilder::set_snapshot_name): <p>The name of the snapshot to be deleted.</p>
    /// - On success, responds with [`DeleteSnapshotOutput`](crate::operation::delete_snapshot::DeleteSnapshotOutput) with field(s):
    ///   - [`snapshot(Option<Snapshot>)`](crate::operation::delete_snapshot::DeleteSnapshotOutput::snapshot): <p>The deleted snapshot object.</p>
    /// - On failure, responds with [`SdkError<DeleteSnapshotError>`](crate::operation::delete_snapshot::DeleteSnapshotError)
    pub fn delete_snapshot(
        &self,
    ) -> crate::operation::delete_snapshot::builders::DeleteSnapshotFluentBuilder {
        crate::operation::delete_snapshot::builders::DeleteSnapshotFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
