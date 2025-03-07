// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateClusterVersion`](crate::operation::update_cluster_version::builders::UpdateClusterVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_cluster_version::builders::UpdateClusterVersionFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_cluster_version::builders::UpdateClusterVersionFluentBuilder::set_name): <p>The name of the Amazon EKS cluster to update.</p>
    ///   - [`version(impl ::std::convert::Into<String>)`](crate::operation::update_cluster_version::builders::UpdateClusterVersionFluentBuilder::version) / [`set_version(Option<String>)`](crate::operation::update_cluster_version::builders::UpdateClusterVersionFluentBuilder::set_version): <p>The desired Kubernetes version following a successful update.</p>
    ///   - [`client_request_token(impl ::std::convert::Into<String>)`](crate::operation::update_cluster_version::builders::UpdateClusterVersionFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::update_cluster_version::builders::UpdateClusterVersionFluentBuilder::set_client_request_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    /// - On success, responds with [`UpdateClusterVersionOutput`](crate::operation::update_cluster_version::UpdateClusterVersionOutput) with field(s):
    ///   - [`update(Option<Update>)`](crate::operation::update_cluster_version::UpdateClusterVersionOutput::update): <p>The full description of the specified update</p>
    /// - On failure, responds with [`SdkError<UpdateClusterVersionError>`](crate::operation::update_cluster_version::UpdateClusterVersionError)
    pub fn update_cluster_version(
        &self,
    ) -> crate::operation::update_cluster_version::builders::UpdateClusterVersionFluentBuilder {
        crate::operation::update_cluster_version::builders::UpdateClusterVersionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
