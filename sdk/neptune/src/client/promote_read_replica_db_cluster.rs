// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PromoteReadReplicaDBCluster`](crate::operation::promote_read_replica_db_cluster::builders::PromoteReadReplicaDBClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`db_cluster_identifier(impl ::std::convert::Into<String>)`](crate::operation::promote_read_replica_db_cluster::builders::PromoteReadReplicaDBClusterFluentBuilder::db_cluster_identifier) / [`set_db_cluster_identifier(Option<String>)`](crate::operation::promote_read_replica_db_cluster::builders::PromoteReadReplicaDBClusterFluentBuilder::set_db_cluster_identifier): <p>Not supported.</p>
    /// - On success, responds with [`PromoteReadReplicaDbClusterOutput`](crate::operation::promote_read_replica_db_cluster::PromoteReadReplicaDbClusterOutput) with field(s):
    ///   - [`db_cluster(Option<DbCluster>)`](crate::operation::promote_read_replica_db_cluster::PromoteReadReplicaDbClusterOutput::db_cluster): <p>Contains the details of an Amazon Neptune DB cluster.</p>  <p>This data type is used as a response element in the <code>DescribeDBClusters</code> action.</p>
    /// - On failure, responds with [`SdkError<PromoteReadReplicaDBClusterError>`](crate::operation::promote_read_replica_db_cluster::PromoteReadReplicaDBClusterError)
    pub fn promote_read_replica_db_cluster(&self) -> crate::operation::promote_read_replica_db_cluster::builders::PromoteReadReplicaDBClusterFluentBuilder{
        crate::operation::promote_read_replica_db_cluster::builders::PromoteReadReplicaDBClusterFluentBuilder::new(self.handle.clone())
    }
}
