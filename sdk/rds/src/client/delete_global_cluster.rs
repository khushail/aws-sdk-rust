// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteGlobalCluster`](crate::operation::delete_global_cluster::builders::DeleteGlobalClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`global_cluster_identifier(impl ::std::convert::Into<String>)`](crate::operation::delete_global_cluster::builders::DeleteGlobalClusterFluentBuilder::global_cluster_identifier) / [`set_global_cluster_identifier(Option<String>)`](crate::operation::delete_global_cluster::builders::DeleteGlobalClusterFluentBuilder::set_global_cluster_identifier): <p>The cluster identifier of the global database cluster being deleted.</p>
    /// - On success, responds with [`DeleteGlobalClusterOutput`](crate::operation::delete_global_cluster::DeleteGlobalClusterOutput) with field(s):
    ///   - [`global_cluster(Option<GlobalCluster>)`](crate::operation::delete_global_cluster::DeleteGlobalClusterOutput::global_cluster): <p>A data type representing an Aurora global database.</p>
    /// - On failure, responds with [`SdkError<DeleteGlobalClusterError>`](crate::operation::delete_global_cluster::DeleteGlobalClusterError)
    pub fn delete_global_cluster(
        &self,
    ) -> crate::operation::delete_global_cluster::builders::DeleteGlobalClusterFluentBuilder {
        crate::operation::delete_global_cluster::builders::DeleteGlobalClusterFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
