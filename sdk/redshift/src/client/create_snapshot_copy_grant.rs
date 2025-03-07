// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateSnapshotCopyGrant`](crate::operation::create_snapshot_copy_grant::builders::CreateSnapshotCopyGrantFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`snapshot_copy_grant_name(impl ::std::convert::Into<String>)`](crate::operation::create_snapshot_copy_grant::builders::CreateSnapshotCopyGrantFluentBuilder::snapshot_copy_grant_name) / [`set_snapshot_copy_grant_name(Option<String>)`](crate::operation::create_snapshot_copy_grant::builders::CreateSnapshotCopyGrantFluentBuilder::set_snapshot_copy_grant_name): <p>The name of the snapshot copy grant. This name must be unique in the region for the Amazon Web Services account.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p> </li>   <li> <p>Alphabetic characters must be lowercase.</p> </li>   <li> <p>First character must be a letter.</p> </li>   <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li>   <li> <p>Must be unique for all clusters within an Amazon Web Services account.</p> </li>  </ul>
    ///   - [`kms_key_id(impl ::std::convert::Into<String>)`](crate::operation::create_snapshot_copy_grant::builders::CreateSnapshotCopyGrantFluentBuilder::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::operation::create_snapshot_copy_grant::builders::CreateSnapshotCopyGrantFluentBuilder::set_kms_key_id): <p>The unique identifier of the encrypted symmetric key to which to grant Amazon Redshift permission. If no key is specified, the default key is used.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_snapshot_copy_grant::builders::CreateSnapshotCopyGrantFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_snapshot_copy_grant::builders::CreateSnapshotCopyGrantFluentBuilder::set_tags): <p>A list of tag instances.</p>
    /// - On success, responds with [`CreateSnapshotCopyGrantOutput`](crate::operation::create_snapshot_copy_grant::CreateSnapshotCopyGrantOutput) with field(s):
    ///   - [`snapshot_copy_grant(Option<SnapshotCopyGrant>)`](crate::operation::create_snapshot_copy_grant::CreateSnapshotCopyGrantOutput::snapshot_copy_grant): <p>The snapshot copy grant that grants Amazon Redshift permission to encrypt copied snapshots with the specified encrypted symmetric key from Amazon Web Services KMS in the destination region.</p>  <p> For more information about managing snapshot copy grants, go to <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/working-with-db-encryption.html">Amazon Redshift Database Encryption</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    /// - On failure, responds with [`SdkError<CreateSnapshotCopyGrantError>`](crate::operation::create_snapshot_copy_grant::CreateSnapshotCopyGrantError)
    pub fn create_snapshot_copy_grant(
        &self,
    ) -> crate::operation::create_snapshot_copy_grant::builders::CreateSnapshotCopyGrantFluentBuilder
    {
        crate::operation::create_snapshot_copy_grant::builders::CreateSnapshotCopyGrantFluentBuilder::new(self.handle.clone())
    }
}
