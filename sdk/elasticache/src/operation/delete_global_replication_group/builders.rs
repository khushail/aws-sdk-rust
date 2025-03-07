// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_global_replication_group::_delete_global_replication_group_output::DeleteGlobalReplicationGroupOutputBuilder;

pub use crate::operation::delete_global_replication_group::_delete_global_replication_group_input::DeleteGlobalReplicationGroupInputBuilder;

/// Fluent builder constructing a request to `DeleteGlobalReplicationGroup`.
///
/// <p>Deleting a Global datastore is a two-step process: </p>
/// <ul>
/// <li> <p>First, you must <code>DisassociateGlobalReplicationGroup</code> to remove the secondary clusters in the Global datastore.</p> </li>
/// <li> <p>Once the Global datastore contains only the primary cluster, you can use the <code>DeleteGlobalReplicationGroup</code> API to delete the Global datastore while retainining the primary cluster using <code>RetainPrimaryReplicationGroup=true</code>.</p> </li>
/// </ul>
/// <p>Since the Global Datastore has only a primary cluster, you can delete the Global Datastore while retaining the primary by setting <code>RetainPrimaryReplicationGroup=true</code>. The primary cluster is never deleted when deleting a Global Datastore. It can only be deleted when it no longer is associated with any Global Datastore.</p>
/// <p>When you receive a successful response from this operation, Amazon ElastiCache immediately begins deleting the selected resources; you cannot cancel or revert this operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteGlobalReplicationGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::delete_global_replication_group::builders::DeleteGlobalReplicationGroupInputBuilder,
}
impl DeleteGlobalReplicationGroupFluentBuilder {
    /// Creates a new `DeleteGlobalReplicationGroup`.
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
            crate::operation::delete_global_replication_group::DeleteGlobalReplicationGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_global_replication_group::DeleteGlobalReplicationGroupError,
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
        crate::operation::delete_global_replication_group::DeleteGlobalReplicationGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_global_replication_group::DeleteGlobalReplicationGroupError,
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
        crate::operation::delete_global_replication_group::DeleteGlobalReplicationGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_global_replication_group::DeleteGlobalReplicationGroupError,
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
            crate::operation::delete_global_replication_group::DeleteGlobalReplicationGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_global_replication_group::DeleteGlobalReplicationGroupError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the Global datastore</p>
    pub fn global_replication_group_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.global_replication_group_id(input.into());
        self
    }
    /// <p>The name of the Global datastore</p>
    pub fn set_global_replication_group_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_global_replication_group_id(input);
        self
    }
    /// <p>The primary replication group is retained as a standalone replication group. </p>
    pub fn retain_primary_replication_group(mut self, input: bool) -> Self {
        self.inner = self.inner.retain_primary_replication_group(input);
        self
    }
    /// <p>The primary replication group is retained as a standalone replication group. </p>
    pub fn set_retain_primary_replication_group(
        mut self,
        input: ::std::option::Option<bool>,
    ) -> Self {
        self.inner = self.inner.set_retain_primary_replication_group(input);
        self
    }
}
