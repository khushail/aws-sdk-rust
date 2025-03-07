// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateGlobalCluster`](crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`global_cluster_identifier(impl ::std::convert::Into<String>)`](crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder::global_cluster_identifier) / [`set_global_cluster_identifier(Option<String>)`](crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder::set_global_cluster_identifier): <p>The cluster identifier of the new global cluster.</p>
    ///   - [`source_db_cluster_identifier(impl ::std::convert::Into<String>)`](crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder::source_db_cluster_identifier) / [`set_source_db_cluster_identifier(Option<String>)`](crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder::set_source_db_cluster_identifier): <p>The Amazon Resource Name (ARN) to use as the primary cluster of the global cluster. This parameter is optional.</p>
    ///   - [`engine(impl ::std::convert::Into<String>)`](crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder::engine) / [`set_engine(Option<String>)`](crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder::set_engine): <p>The name of the database engine to be used for this cluster.</p>
    ///   - [`engine_version(impl ::std::convert::Into<String>)`](crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder::engine_version) / [`set_engine_version(Option<String>)`](crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder::set_engine_version): <p>The engine version of the global cluster.</p>
    ///   - [`deletion_protection(bool)`](crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder::deletion_protection) / [`set_deletion_protection(Option<bool>)`](crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder::set_deletion_protection): <p>The deletion protection setting for the new global cluster. The global cluster can't be deleted when deletion protection is enabled. </p>
    ///   - [`database_name(impl ::std::convert::Into<String>)`](crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder::database_name) / [`set_database_name(Option<String>)`](crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder::set_database_name): <p>The name for your database of up to 64 alpha-numeric characters. If you do not provide a name, Amazon DocumentDB will not create a database in the global cluster you are creating.</p>
    ///   - [`storage_encrypted(bool)`](crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder::storage_encrypted) / [`set_storage_encrypted(Option<bool>)`](crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder::set_storage_encrypted): <p>The storage encryption setting for the new global cluster. </p>
    /// - On success, responds with [`CreateGlobalClusterOutput`](crate::operation::create_global_cluster::CreateGlobalClusterOutput) with field(s):
    ///   - [`global_cluster(Option<GlobalCluster>)`](crate::operation::create_global_cluster::CreateGlobalClusterOutput::global_cluster): <p>A data type representing an Amazon DocumentDB global cluster.</p>
    /// - On failure, responds with [`SdkError<CreateGlobalClusterError>`](crate::operation::create_global_cluster::CreateGlobalClusterError)
    pub fn create_global_cluster(
        &self,
    ) -> crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder {
        crate::operation::create_global_cluster::builders::CreateGlobalClusterFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
