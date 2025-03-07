// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateCluster`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::set_client_token): <p>A unique, case-sensitive string of up to 64 ASCII characters. To make an idempotent API request with an action, specify a client token in the request.</p>
    ///   - [`cluster_name(impl ::std::convert::Into<String>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::cluster_name) / [`set_cluster_name(Option<String>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::set_cluster_name): <p>The name of the cluster.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::set_tags): <p>The tags associated with the cluster.</p>
    /// - On success, responds with [`CreateClusterOutput`](crate::operation::create_cluster::CreateClusterOutput) with field(s):
    ///   - [`cluster(Option<Cluster>)`](crate::operation::create_cluster::CreateClusterOutput::cluster): <p>The cluster that was created.</p>
    /// - On failure, responds with [`SdkError<CreateClusterError>`](crate::operation::create_cluster::CreateClusterError)
    pub fn create_cluster(
        &self,
    ) -> crate::operation::create_cluster::builders::CreateClusterFluentBuilder {
        crate::operation::create_cluster::builders::CreateClusterFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
