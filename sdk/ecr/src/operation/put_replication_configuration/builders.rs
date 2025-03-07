// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_replication_configuration::_put_replication_configuration_output::PutReplicationConfigurationOutputBuilder;

pub use crate::operation::put_replication_configuration::_put_replication_configuration_input::PutReplicationConfigurationInputBuilder;

/// Fluent builder constructing a request to `PutReplicationConfiguration`.
///
/// <p>Creates or updates the replication configuration for a registry. The existing replication configuration for a repository can be retrieved with the <code>DescribeRegistry</code> API action. The first time the PutReplicationConfiguration API is called, a service-linked IAM role is created in your account for the replication process. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/using-service-linked-roles.html">Using service-linked roles for Amazon ECR</a> in the <i>Amazon Elastic Container Registry User Guide</i>.</p> <note>
/// <p>When configuring cross-account replication, the destination account must grant the source account permission to replicate. This permission is controlled using a registry permissions policy. For more information, see <code>PutRegistryPolicy</code>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutReplicationConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::put_replication_configuration::builders::PutReplicationConfigurationInputBuilder,
}
impl PutReplicationConfigurationFluentBuilder {
    /// Creates a new `PutReplicationConfiguration`.
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
            crate::operation::put_replication_configuration::PutReplicationConfiguration,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_replication_configuration::PutReplicationConfigurationError,
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
        crate::operation::put_replication_configuration::PutReplicationConfigurationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_replication_configuration::PutReplicationConfigurationError,
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
        crate::operation::put_replication_configuration::PutReplicationConfigurationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_replication_configuration::PutReplicationConfigurationError,
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
            crate::operation::put_replication_configuration::PutReplicationConfiguration,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_replication_configuration::PutReplicationConfigurationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>An object representing the replication configuration for a registry.</p>
    pub fn replication_configuration(
        mut self,
        input: crate::types::ReplicationConfiguration,
    ) -> Self {
        self.inner = self.inner.replication_configuration(input);
        self
    }
    /// <p>An object representing the replication configuration for a registry.</p>
    pub fn set_replication_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ReplicationConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_replication_configuration(input);
        self
    }
}
