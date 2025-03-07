// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_identity_resolution_job::_get_identity_resolution_job_output::GetIdentityResolutionJobOutputBuilder;

pub use crate::operation::get_identity_resolution_job::_get_identity_resolution_job_input::GetIdentityResolutionJobInputBuilder;

/// Fluent builder constructing a request to `GetIdentityResolutionJob`.
///
/// <p>Returns information about an Identity Resolution Job in a specific domain. </p>
/// <p>Identity Resolution Jobs are set up using the Amazon Connect admin console. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/use-identity-resolution.html">Use Identity Resolution to consolidate similar profiles</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetIdentityResolutionJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::get_identity_resolution_job::builders::GetIdentityResolutionJobInputBuilder,
}
impl GetIdentityResolutionJobFluentBuilder {
    /// Creates a new `GetIdentityResolutionJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::get_identity_resolution_job::GetIdentityResolutionJob,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_identity_resolution_job::GetIdentityResolutionJobError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_identity_resolution_job::GetIdentityResolutionJobOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_identity_resolution_job::GetIdentityResolutionJobError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_identity_resolution_job::GetIdentityResolutionJobOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_identity_resolution_job::GetIdentityResolutionJobError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::get_identity_resolution_job::GetIdentityResolutionJob,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_identity_resolution_job::GetIdentityResolutionJobError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique name of the domain.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The unique name of the domain.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The unique identifier of the Identity Resolution Job.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.job_id(input.into());
        self
    }
    /// <p>The unique identifier of the Identity Resolution Job.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_job_id(input);
        self
    }
}
