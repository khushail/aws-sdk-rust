// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateLensShare`](crate::operation::create_lens_share::builders::CreateLensShareFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`lens_alias(impl ::std::convert::Into<String>)`](crate::operation::create_lens_share::builders::CreateLensShareFluentBuilder::lens_alias) / [`set_lens_alias(Option<String>)`](crate::operation::create_lens_share::builders::CreateLensShareFluentBuilder::set_lens_alias): <p>The alias of the lens.</p>  <p>For Amazon Web Services official lenses, this is either the lens alias, such as <code>serverless</code>, or the lens ARN, such as <code>arn:aws:wellarchitected:us-east-1::lens/serverless</code>. Note that some operations (such as ExportLens and CreateLensShare) are not permitted on Amazon Web Services official lenses.</p>  <p>For custom lenses, this is the lens ARN, such as <code>arn:aws:wellarchitected:us-west-2:123456789012:lens/0123456789abcdef01234567890abcdef</code>. </p>  <p>Each lens is identified by its <code>LensSummary$LensAlias</code>.</p>
    ///   - [`shared_with(impl ::std::convert::Into<String>)`](crate::operation::create_lens_share::builders::CreateLensShareFluentBuilder::shared_with) / [`set_shared_with(Option<String>)`](crate::operation::create_lens_share::builders::CreateLensShareFluentBuilder::set_shared_with): <p>The Amazon Web Services account ID, IAM role, organization ID, or organizational unit (OU) ID with which the workload is shared.</p>
    ///   - [`client_request_token(impl ::std::convert::Into<String>)`](crate::operation::create_lens_share::builders::CreateLensShareFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::create_lens_share::builders::CreateLensShareFluentBuilder::set_client_request_token): <p>A unique case-sensitive string used to ensure that this request is idempotent (executes only once).</p>  <p>You should not reuse the same token for other requests. If you retry a request with the same client request token and the same parameters after the original request has completed successfully, the result of the original request is returned.</p> <important>   <p>This token is listed as required, however, if you do not specify it, the Amazon Web Services SDKs automatically generate one for you. If you are not using the Amazon Web Services SDK or the CLI, you must provide this token or the request will fail.</p>  </important>
    /// - On success, responds with [`CreateLensShareOutput`](crate::operation::create_lens_share::CreateLensShareOutput) with field(s):
    ///   - [`share_id(Option<String>)`](crate::operation::create_lens_share::CreateLensShareOutput::share_id): <p>The ID associated with the workload share.</p>
    /// - On failure, responds with [`SdkError<CreateLensShareError>`](crate::operation::create_lens_share::CreateLensShareError)
    pub fn create_lens_share(
        &self,
    ) -> crate::operation::create_lens_share::builders::CreateLensShareFluentBuilder {
        crate::operation::create_lens_share::builders::CreateLensShareFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
