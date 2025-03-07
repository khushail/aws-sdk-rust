// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AuthorizeEndpointAccess`](crate::operation::authorize_endpoint_access::builders::AuthorizeEndpointAccessFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_identifier(impl ::std::convert::Into<String>)`](crate::operation::authorize_endpoint_access::builders::AuthorizeEndpointAccessFluentBuilder::cluster_identifier) / [`set_cluster_identifier(Option<String>)`](crate::operation::authorize_endpoint_access::builders::AuthorizeEndpointAccessFluentBuilder::set_cluster_identifier): <p>The cluster identifier of the cluster to grant access to.</p>
    ///   - [`account(impl ::std::convert::Into<String>)`](crate::operation::authorize_endpoint_access::builders::AuthorizeEndpointAccessFluentBuilder::account) / [`set_account(Option<String>)`](crate::operation::authorize_endpoint_access::builders::AuthorizeEndpointAccessFluentBuilder::set_account): <p>The Amazon Web Services account ID to grant access to.</p>
    ///   - [`vpc_ids(Vec<String>)`](crate::operation::authorize_endpoint_access::builders::AuthorizeEndpointAccessFluentBuilder::vpc_ids) / [`set_vpc_ids(Option<Vec<String>>)`](crate::operation::authorize_endpoint_access::builders::AuthorizeEndpointAccessFluentBuilder::set_vpc_ids): <p>The virtual private cloud (VPC) identifiers to grant access to.</p>
    /// - On success, responds with [`AuthorizeEndpointAccessOutput`](crate::operation::authorize_endpoint_access::AuthorizeEndpointAccessOutput) with field(s):
    ///   - [`grantor(Option<String>)`](crate::operation::authorize_endpoint_access::AuthorizeEndpointAccessOutput::grantor): <p>The Amazon Web Services account ID of the cluster owner.</p>
    ///   - [`grantee(Option<String>)`](crate::operation::authorize_endpoint_access::AuthorizeEndpointAccessOutput::grantee): <p>The Amazon Web Services account ID of the grantee of the cluster.</p>
    ///   - [`cluster_identifier(Option<String>)`](crate::operation::authorize_endpoint_access::AuthorizeEndpointAccessOutput::cluster_identifier): <p>The cluster identifier.</p>
    ///   - [`authorize_time(Option<DateTime>)`](crate::operation::authorize_endpoint_access::AuthorizeEndpointAccessOutput::authorize_time): <p>The time (UTC) when the authorization was created.</p>
    ///   - [`cluster_status(Option<String>)`](crate::operation::authorize_endpoint_access::AuthorizeEndpointAccessOutput::cluster_status): <p>The status of the cluster.</p>
    ///   - [`status(Option<AuthorizationStatus>)`](crate::operation::authorize_endpoint_access::AuthorizeEndpointAccessOutput::status): <p>The status of the authorization action.</p>
    ///   - [`allowed_all_vp_cs(bool)`](crate::operation::authorize_endpoint_access::AuthorizeEndpointAccessOutput::allowed_all_vp_cs): <p>Indicates whether all VPCs in the grantee account are allowed access to the cluster.</p>
    ///   - [`allowed_vp_cs(Option<Vec<String>>)`](crate::operation::authorize_endpoint_access::AuthorizeEndpointAccessOutput::allowed_vp_cs): <p>The VPCs allowed access to the cluster.</p>
    ///   - [`endpoint_count(i32)`](crate::operation::authorize_endpoint_access::AuthorizeEndpointAccessOutput::endpoint_count): <p>The number of Redshift-managed VPC endpoints created for the authorization.</p>
    /// - On failure, responds with [`SdkError<AuthorizeEndpointAccessError>`](crate::operation::authorize_endpoint_access::AuthorizeEndpointAccessError)
    pub fn authorize_endpoint_access(
        &self,
    ) -> crate::operation::authorize_endpoint_access::builders::AuthorizeEndpointAccessFluentBuilder
    {
        crate::operation::authorize_endpoint_access::builders::AuthorizeEndpointAccessFluentBuilder::new(self.handle.clone())
    }
}
