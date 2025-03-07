// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateWorkloadShare`](crate::operation::create_workload_share::builders::CreateWorkloadShareFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`workload_id(impl ::std::convert::Into<String>)`](crate::operation::create_workload_share::builders::CreateWorkloadShareFluentBuilder::workload_id) / [`set_workload_id(Option<String>)`](crate::operation::create_workload_share::builders::CreateWorkloadShareFluentBuilder::set_workload_id): <p>The ID assigned to the workload. This ID is unique within an Amazon Web Services Region.</p>
    ///   - [`shared_with(impl ::std::convert::Into<String>)`](crate::operation::create_workload_share::builders::CreateWorkloadShareFluentBuilder::shared_with) / [`set_shared_with(Option<String>)`](crate::operation::create_workload_share::builders::CreateWorkloadShareFluentBuilder::set_shared_with): <p>The Amazon Web Services account ID, IAM role, organization ID, or organizational unit (OU) ID with which the workload is shared.</p>
    ///   - [`permission_type(PermissionType)`](crate::operation::create_workload_share::builders::CreateWorkloadShareFluentBuilder::permission_type) / [`set_permission_type(Option<PermissionType>)`](crate::operation::create_workload_share::builders::CreateWorkloadShareFluentBuilder::set_permission_type): <p>Permission granted on a workload share.</p>
    ///   - [`client_request_token(impl ::std::convert::Into<String>)`](crate::operation::create_workload_share::builders::CreateWorkloadShareFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::create_workload_share::builders::CreateWorkloadShareFluentBuilder::set_client_request_token): <p>A unique case-sensitive string used to ensure that this request is idempotent (executes only once).</p>  <p>You should not reuse the same token for other requests. If you retry a request with the same client request token and the same parameters after the original request has completed successfully, the result of the original request is returned.</p> <important>   <p>This token is listed as required, however, if you do not specify it, the Amazon Web Services SDKs automatically generate one for you. If you are not using the Amazon Web Services SDK or the CLI, you must provide this token or the request will fail.</p>  </important>
    /// - On success, responds with [`CreateWorkloadShareOutput`](crate::operation::create_workload_share::CreateWorkloadShareOutput) with field(s):
    ///   - [`workload_id(Option<String>)`](crate::operation::create_workload_share::CreateWorkloadShareOutput::workload_id): <p>The ID assigned to the workload. This ID is unique within an Amazon Web Services Region.</p>
    ///   - [`share_id(Option<String>)`](crate::operation::create_workload_share::CreateWorkloadShareOutput::share_id): <p>The ID associated with the workload share.</p>
    /// - On failure, responds with [`SdkError<CreateWorkloadShareError>`](crate::operation::create_workload_share::CreateWorkloadShareError)
    pub fn create_workload_share(
        &self,
    ) -> crate::operation::create_workload_share::builders::CreateWorkloadShareFluentBuilder {
        crate::operation::create_workload_share::builders::CreateWorkloadShareFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
