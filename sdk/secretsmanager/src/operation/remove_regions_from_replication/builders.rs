// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_regions_from_replication::_remove_regions_from_replication_output::RemoveRegionsFromReplicationOutputBuilder;

pub use crate::operation::remove_regions_from_replication::_remove_regions_from_replication_input::RemoveRegionsFromReplicationInputBuilder;

/// Fluent builder constructing a request to `RemoveRegionsFromReplication`.
///
/// <p>For a secret that is replicated to other Regions, deletes the secret replicas from the Regions you specify.</p>
/// <p>Secrets Manager generates a CloudTrail log entry when you call this action. Do not include sensitive information in request parameters because it might be logged. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/retrieve-ct-entries.html">Logging Secrets Manager events with CloudTrail</a>.</p>
/// <p> <b>Required permissions: </b> <code>secretsmanager:RemoveRegionsFromReplication</code>. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/reference_iam-permissions.html#reference_iam-permissions_actions"> IAM policy actions for Secrets Manager</a> and <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access.html">Authentication and access control in Secrets Manager</a>. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RemoveRegionsFromReplicationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::remove_regions_from_replication::builders::RemoveRegionsFromReplicationInputBuilder,
}
impl RemoveRegionsFromReplicationFluentBuilder {
    /// Creates a new `RemoveRegionsFromReplication`.
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
            crate::operation::remove_regions_from_replication::RemoveRegionsFromReplication,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError,
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
        crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError,
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
        crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError,
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
            crate::operation::remove_regions_from_replication::RemoveRegionsFromReplication,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ARN or name of the secret.</p>
    pub fn secret_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.secret_id(input.into());
        self
    }
    /// <p>The ARN or name of the secret.</p>
    pub fn set_secret_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_secret_id(input);
        self
    }
    /// Appends an item to `RemoveReplicaRegions`.
    ///
    /// To override the contents of this collection use [`set_remove_replica_regions`](Self::set_remove_replica_regions).
    ///
    /// <p>The Regions of the replicas to remove.</p>
    pub fn remove_replica_regions(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.remove_replica_regions(input.into());
        self
    }
    /// <p>The Regions of the replicas to remove.</p>
    pub fn set_remove_replica_regions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_remove_replica_regions(input);
        self
    }
}
