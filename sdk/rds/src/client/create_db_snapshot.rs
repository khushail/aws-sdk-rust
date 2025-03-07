// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateDBSnapshot`](crate::operation::create_db_snapshot::builders::CreateDBSnapshotFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`db_snapshot_identifier(impl ::std::convert::Into<String>)`](crate::operation::create_db_snapshot::builders::CreateDBSnapshotFluentBuilder::db_snapshot_identifier) / [`set_db_snapshot_identifier(Option<String>)`](crate::operation::create_db_snapshot::builders::CreateDBSnapshotFluentBuilder::set_db_snapshot_identifier): <p>The identifier for the DB snapshot.</p>  <p>Constraints:</p>  <ul>   <li> <p>Can't be null, empty, or blank</p> </li>   <li> <p>Must contain from 1 to 255 letters, numbers, or hyphens</p> </li>   <li> <p>First character must be a letter</p> </li>   <li> <p>Can't end with a hyphen or contain two consecutive hyphens</p> </li>  </ul>  <p>Example: <code>my-snapshot-id</code> </p>
    ///   - [`db_instance_identifier(impl ::std::convert::Into<String>)`](crate::operation::create_db_snapshot::builders::CreateDBSnapshotFluentBuilder::db_instance_identifier) / [`set_db_instance_identifier(Option<String>)`](crate::operation::create_db_snapshot::builders::CreateDBSnapshotFluentBuilder::set_db_instance_identifier): <p>The identifier of the DB instance that you want to create the snapshot of.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must match the identifier of an existing DBInstance.</p> </li>  </ul>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_db_snapshot::builders::CreateDBSnapshotFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_db_snapshot::builders::CreateDBSnapshotFluentBuilder::set_tags): <p>A list of tags. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Tagging.html">Tagging Amazon RDS Resources</a> in the <i>Amazon RDS User Guide.</i> </p>
    /// - On success, responds with [`CreateDbSnapshotOutput`](crate::operation::create_db_snapshot::CreateDbSnapshotOutput) with field(s):
    ///   - [`db_snapshot(Option<DbSnapshot>)`](crate::operation::create_db_snapshot::CreateDbSnapshotOutput::db_snapshot): <p>Contains the details of an Amazon RDS DB snapshot.</p>  <p>This data type is used as a response element in the <code>DescribeDBSnapshots</code> action.</p>
    /// - On failure, responds with [`SdkError<CreateDBSnapshotError>`](crate::operation::create_db_snapshot::CreateDBSnapshotError)
    pub fn create_db_snapshot(
        &self,
    ) -> crate::operation::create_db_snapshot::builders::CreateDBSnapshotFluentBuilder {
        crate::operation::create_db_snapshot::builders::CreateDBSnapshotFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
