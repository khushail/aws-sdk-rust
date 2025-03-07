// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_resource_data_sync::_create_resource_data_sync_output::CreateResourceDataSyncOutputBuilder;

pub use crate::operation::create_resource_data_sync::_create_resource_data_sync_input::CreateResourceDataSyncInputBuilder;

/// Fluent builder constructing a request to `CreateResourceDataSync`.
///
/// <p>A resource data sync helps you view data from multiple sources in a single location. Amazon Web Services Systems Manager offers two types of resource data sync: <code>SyncToDestination</code> and <code>SyncFromSource</code>.</p>
/// <p>You can configure Systems Manager Inventory to use the <code>SyncToDestination</code> type to synchronize Inventory data from multiple Amazon Web Services Regions to a single Amazon Simple Storage Service (Amazon S3) bucket. For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-inventory-datasync.html">Configuring resource data sync for Inventory</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p>
/// <p>You can configure Systems Manager Explorer to use the <code>SyncFromSource</code> type to synchronize operational work items (OpsItems) and operational data (OpsData) from multiple Amazon Web Services Regions to a single Amazon S3 bucket. This type can synchronize OpsItems and OpsData from multiple Amazon Web Services accounts and Amazon Web Services Regions or <code>EntireOrganization</code> by using Organizations. For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/Explorer-resource-data-sync.html">Setting up Systems Manager Explorer to display data from multiple accounts and Regions</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p>
/// <p>A resource data sync is an asynchronous operation that returns immediately. After a successful initial sync is completed, the system continuously syncs data. To check the status of a sync, use the <code>ListResourceDataSync</code>.</p> <note>
/// <p>By default, data isn't encrypted in Amazon S3. We strongly recommend that you enable encryption in Amazon S3 to ensure secure data storage. We also recommend that you secure access to the Amazon S3 bucket by creating a restrictive bucket policy. </p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateResourceDataSyncFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::create_resource_data_sync::builders::CreateResourceDataSyncInputBuilder,
}
impl CreateResourceDataSyncFluentBuilder {
    /// Creates a new `CreateResourceDataSync`.
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
            crate::operation::create_resource_data_sync::CreateResourceDataSync,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_resource_data_sync::CreateResourceDataSyncError,
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
        crate::operation::create_resource_data_sync::CreateResourceDataSyncOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_resource_data_sync::CreateResourceDataSyncError,
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
        crate::operation::create_resource_data_sync::CreateResourceDataSyncOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_resource_data_sync::CreateResourceDataSyncError,
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
            crate::operation::create_resource_data_sync::CreateResourceDataSync,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_resource_data_sync::CreateResourceDataSyncError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>A name for the configuration.</p>
    pub fn sync_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sync_name(input.into());
        self
    }
    /// <p>A name for the configuration.</p>
    pub fn set_sync_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sync_name(input);
        self
    }
    /// <p>Amazon S3 configuration details for the sync. This parameter is required if the <code>SyncType</code> value is SyncToDestination.</p>
    pub fn s3_destination(mut self, input: crate::types::ResourceDataSyncS3Destination) -> Self {
        self.inner = self.inner.s3_destination(input);
        self
    }
    /// <p>Amazon S3 configuration details for the sync. This parameter is required if the <code>SyncType</code> value is SyncToDestination.</p>
    pub fn set_s3_destination(
        mut self,
        input: ::std::option::Option<crate::types::ResourceDataSyncS3Destination>,
    ) -> Self {
        self.inner = self.inner.set_s3_destination(input);
        self
    }
    /// <p>Specify <code>SyncToDestination</code> to create a resource data sync that synchronizes data to an S3 bucket for Inventory. If you specify <code>SyncToDestination</code>, you must provide a value for <code>S3Destination</code>. Specify <code>SyncFromSource</code> to synchronize data from a single account and multiple Regions, or multiple Amazon Web Services accounts and Amazon Web Services Regions, as listed in Organizations for Explorer. If you specify <code>SyncFromSource</code>, you must provide a value for <code>SyncSource</code>. The default value is <code>SyncToDestination</code>.</p>
    pub fn sync_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sync_type(input.into());
        self
    }
    /// <p>Specify <code>SyncToDestination</code> to create a resource data sync that synchronizes data to an S3 bucket for Inventory. If you specify <code>SyncToDestination</code>, you must provide a value for <code>S3Destination</code>. Specify <code>SyncFromSource</code> to synchronize data from a single account and multiple Regions, or multiple Amazon Web Services accounts and Amazon Web Services Regions, as listed in Organizations for Explorer. If you specify <code>SyncFromSource</code>, you must provide a value for <code>SyncSource</code>. The default value is <code>SyncToDestination</code>.</p>
    pub fn set_sync_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sync_type(input);
        self
    }
    /// <p>Specify information about the data sources to synchronize. This parameter is required if the <code>SyncType</code> value is SyncFromSource.</p>
    pub fn sync_source(mut self, input: crate::types::ResourceDataSyncSource) -> Self {
        self.inner = self.inner.sync_source(input);
        self
    }
    /// <p>Specify information about the data sources to synchronize. This parameter is required if the <code>SyncType</code> value is SyncFromSource.</p>
    pub fn set_sync_source(
        mut self,
        input: ::std::option::Option<crate::types::ResourceDataSyncSource>,
    ) -> Self {
        self.inner = self.inner.set_sync_source(input);
        self
    }
}
