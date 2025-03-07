// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SwitchoverReadReplica`](crate::operation::switchover_read_replica::builders::SwitchoverReadReplicaFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`db_instance_identifier(impl ::std::convert::Into<String>)`](crate::operation::switchover_read_replica::builders::SwitchoverReadReplicaFluentBuilder::db_instance_identifier) / [`set_db_instance_identifier(Option<String>)`](crate::operation::switchover_read_replica::builders::SwitchoverReadReplicaFluentBuilder::set_db_instance_identifier): <p>The DB instance identifier of the current standby database. This value is stored as a lowercase string.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must match the identiﬁer of an existing Oracle read replica DB instance.</p> </li>  </ul>
    /// - On success, responds with [`SwitchoverReadReplicaOutput`](crate::operation::switchover_read_replica::SwitchoverReadReplicaOutput) with field(s):
    ///   - [`db_instance(Option<DbInstance>)`](crate::operation::switchover_read_replica::SwitchoverReadReplicaOutput::db_instance): <p>Contains the details of an Amazon RDS DB instance.</p>  <p>This data type is used as a response element in the operations <code>CreateDBInstance</code>, <code>CreateDBInstanceReadReplica</code>, <code>DeleteDBInstance</code>, <code>DescribeDBInstances</code>, <code>ModifyDBInstance</code>, <code>PromoteReadReplica</code>, <code>RebootDBInstance</code>, <code>RestoreDBInstanceFromDBSnapshot</code>, <code>RestoreDBInstanceFromS3</code>, <code>RestoreDBInstanceToPointInTime</code>, <code>StartDBInstance</code>, and <code>StopDBInstance</code>.</p>
    /// - On failure, responds with [`SdkError<SwitchoverReadReplicaError>`](crate::operation::switchover_read_replica::SwitchoverReadReplicaError)
    pub fn switchover_read_replica(
        &self,
    ) -> crate::operation::switchover_read_replica::builders::SwitchoverReadReplicaFluentBuilder
    {
        crate::operation::switchover_read_replica::builders::SwitchoverReadReplicaFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
