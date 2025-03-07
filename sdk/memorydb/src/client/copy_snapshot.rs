// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CopySnapshot`](crate::operation::copy_snapshot::builders::CopySnapshotFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`source_snapshot_name(impl ::std::convert::Into<String>)`](crate::operation::copy_snapshot::builders::CopySnapshotFluentBuilder::source_snapshot_name) / [`set_source_snapshot_name(Option<String>)`](crate::operation::copy_snapshot::builders::CopySnapshotFluentBuilder::set_source_snapshot_name): <p>The name of an existing snapshot from which to make a copy.</p>
    ///   - [`target_snapshot_name(impl ::std::convert::Into<String>)`](crate::operation::copy_snapshot::builders::CopySnapshotFluentBuilder::target_snapshot_name) / [`set_target_snapshot_name(Option<String>)`](crate::operation::copy_snapshot::builders::CopySnapshotFluentBuilder::set_target_snapshot_name): <p>A name for the snapshot copy. MemoryDB does not permit overwriting a snapshot, therefore this name must be unique within its context - MemoryDB or an Amazon S3 bucket if exporting.</p>
    ///   - [`target_bucket(impl ::std::convert::Into<String>)`](crate::operation::copy_snapshot::builders::CopySnapshotFluentBuilder::target_bucket) / [`set_target_bucket(Option<String>)`](crate::operation::copy_snapshot::builders::CopySnapshotFluentBuilder::set_target_bucket): <p>The Amazon S3 bucket to which the snapshot is exported. This parameter is used only when exporting a snapshot for external access. When using this parameter to export a snapshot, be sure MemoryDB has the needed permissions to this S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/MemoryDB/latest/devguide/snapshots-exporting.html">Step 2: Grant MemoryDB Access to Your Amazon S3 Bucket</a>. </p>
    ///   - [`kms_key_id(impl ::std::convert::Into<String>)`](crate::operation::copy_snapshot::builders::CopySnapshotFluentBuilder::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::operation::copy_snapshot::builders::CopySnapshotFluentBuilder::set_kms_key_id): <p>The ID of the KMS key used to encrypt the target snapshot.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::copy_snapshot::builders::CopySnapshotFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::copy_snapshot::builders::CopySnapshotFluentBuilder::set_tags): <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value, although null is accepted.</p>
    /// - On success, responds with [`CopySnapshotOutput`](crate::operation::copy_snapshot::CopySnapshotOutput) with field(s):
    ///   - [`snapshot(Option<Snapshot>)`](crate::operation::copy_snapshot::CopySnapshotOutput::snapshot): <p>Represents a copy of an entire cluster as of the time when the snapshot was taken.</p>
    /// - On failure, responds with [`SdkError<CopySnapshotError>`](crate::operation::copy_snapshot::CopySnapshotError)
    pub fn copy_snapshot(
        &self,
    ) -> crate::operation::copy_snapshot::builders::CopySnapshotFluentBuilder {
        crate::operation::copy_snapshot::builders::CopySnapshotFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
