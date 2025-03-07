// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_table::_update_table_output::UpdateTableOutputBuilder;

pub use crate::operation::update_table::_update_table_input::UpdateTableInputBuilder;

/// Fluent builder constructing a request to `UpdateTable`.
///
/// <p>Modifies the provisioned throughput settings, global secondary indexes, or DynamoDB Streams settings for a given table.</p> <important>
/// <p>This operation only applies to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html">Version 2019.11.21 (Current)</a> of global tables. </p>
/// </important>
/// <p>You can only perform one of the following operations at once:</p>
/// <ul>
/// <li> <p>Modify the provisioned throughput settings of the table.</p> </li>
/// <li> <p>Remove a global secondary index from the table.</p> </li>
/// <li> <p>Create a new global secondary index on the table. After the index begins backfilling, you can use <code>UpdateTable</code> to perform other operations.</p> </li>
/// </ul>
/// <p> <code>UpdateTable</code> is an asynchronous operation; while it is executing, the table status changes from <code>ACTIVE</code> to <code>UPDATING</code>. While it is <code>UPDATING</code>, you cannot issue another <code>UpdateTable</code> request. When the table returns to the <code>ACTIVE</code> state, the <code>UpdateTable</code> operation is complete.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateTableFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_table::builders::UpdateTableInputBuilder,
}
impl UpdateTableFluentBuilder {
    /// Creates a new `UpdateTable`.
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
            crate::operation::update_table::UpdateTable,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_table::UpdateTableError>,
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
        crate::operation::update_table::UpdateTableOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_table::UpdateTableError>,
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
        crate::operation::update_table::UpdateTableOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_table::UpdateTableError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_table::UpdateTable,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_table::UpdateTableError>,
    > {
        self.customize_middleware().await
    }
    /// Appends an item to `AttributeDefinitions`.
    ///
    /// To override the contents of this collection use [`set_attribute_definitions`](Self::set_attribute_definitions).
    ///
    /// <p>An array of attributes that describe the key schema for the table and indexes. If you are adding a new global secondary index to the table, <code>AttributeDefinitions</code> must include the key element(s) of the new index.</p>
    pub fn attribute_definitions(mut self, input: crate::types::AttributeDefinition) -> Self {
        self.inner = self.inner.attribute_definitions(input);
        self
    }
    /// <p>An array of attributes that describe the key schema for the table and indexes. If you are adding a new global secondary index to the table, <code>AttributeDefinitions</code> must include the key element(s) of the new index.</p>
    pub fn set_attribute_definitions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AttributeDefinition>>,
    ) -> Self {
        self.inner = self.inner.set_attribute_definitions(input);
        self
    }
    /// <p>The name of the table to be updated.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table to be updated.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>Controls how you are charged for read and write throughput and how you manage capacity. When switching from pay-per-request to provisioned capacity, initial provisioned capacity values must be set. The initial provisioned capacity values are estimated based on the consumed read and write capacity of your table and global secondary indexes over the past 30 minutes.</p>
    /// <ul>
    /// <li> <p> <code>PROVISIONED</code> - We recommend using <code>PROVISIONED</code> for predictable workloads. <code>PROVISIONED</code> sets the billing mode to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadWriteCapacityMode.html#HowItWorks.ProvisionedThroughput.Manual">Provisioned Mode</a>.</p> </li>
    /// <li> <p> <code>PAY_PER_REQUEST</code> - We recommend using <code>PAY_PER_REQUEST</code> for unpredictable workloads. <code>PAY_PER_REQUEST</code> sets the billing mode to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadWriteCapacityMode.html#HowItWorks.OnDemand">On-Demand Mode</a>. </p> </li>
    /// </ul>
    pub fn billing_mode(mut self, input: crate::types::BillingMode) -> Self {
        self.inner = self.inner.billing_mode(input);
        self
    }
    /// <p>Controls how you are charged for read and write throughput and how you manage capacity. When switching from pay-per-request to provisioned capacity, initial provisioned capacity values must be set. The initial provisioned capacity values are estimated based on the consumed read and write capacity of your table and global secondary indexes over the past 30 minutes.</p>
    /// <ul>
    /// <li> <p> <code>PROVISIONED</code> - We recommend using <code>PROVISIONED</code> for predictable workloads. <code>PROVISIONED</code> sets the billing mode to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadWriteCapacityMode.html#HowItWorks.ProvisionedThroughput.Manual">Provisioned Mode</a>.</p> </li>
    /// <li> <p> <code>PAY_PER_REQUEST</code> - We recommend using <code>PAY_PER_REQUEST</code> for unpredictable workloads. <code>PAY_PER_REQUEST</code> sets the billing mode to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadWriteCapacityMode.html#HowItWorks.OnDemand">On-Demand Mode</a>. </p> </li>
    /// </ul>
    pub fn set_billing_mode(
        mut self,
        input: ::std::option::Option<crate::types::BillingMode>,
    ) -> Self {
        self.inner = self.inner.set_billing_mode(input);
        self
    }
    /// <p>The new provisioned throughput settings for the specified table or index.</p>
    pub fn provisioned_throughput(mut self, input: crate::types::ProvisionedThroughput) -> Self {
        self.inner = self.inner.provisioned_throughput(input);
        self
    }
    /// <p>The new provisioned throughput settings for the specified table or index.</p>
    pub fn set_provisioned_throughput(
        mut self,
        input: ::std::option::Option<crate::types::ProvisionedThroughput>,
    ) -> Self {
        self.inner = self.inner.set_provisioned_throughput(input);
        self
    }
    /// Appends an item to `GlobalSecondaryIndexUpdates`.
    ///
    /// To override the contents of this collection use [`set_global_secondary_index_updates`](Self::set_global_secondary_index_updates).
    ///
    /// <p>An array of one or more global secondary indexes for the table. For each index in the array, you can request one action:</p>
    /// <ul>
    /// <li> <p> <code>Create</code> - add a new global secondary index to the table.</p> </li>
    /// <li> <p> <code>Update</code> - modify the provisioned throughput settings of an existing global secondary index.</p> </li>
    /// <li> <p> <code>Delete</code> - remove a global secondary index from the table.</p> </li>
    /// </ul>
    /// <p>You can create or delete only one global secondary index per <code>UpdateTable</code> operation.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GSI.OnlineOps.html">Managing Global Secondary Indexes</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p>
    pub fn global_secondary_index_updates(
        mut self,
        input: crate::types::GlobalSecondaryIndexUpdate,
    ) -> Self {
        self.inner = self.inner.global_secondary_index_updates(input);
        self
    }
    /// <p>An array of one or more global secondary indexes for the table. For each index in the array, you can request one action:</p>
    /// <ul>
    /// <li> <p> <code>Create</code> - add a new global secondary index to the table.</p> </li>
    /// <li> <p> <code>Update</code> - modify the provisioned throughput settings of an existing global secondary index.</p> </li>
    /// <li> <p> <code>Delete</code> - remove a global secondary index from the table.</p> </li>
    /// </ul>
    /// <p>You can create or delete only one global secondary index per <code>UpdateTable</code> operation.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GSI.OnlineOps.html">Managing Global Secondary Indexes</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p>
    pub fn set_global_secondary_index_updates(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::GlobalSecondaryIndexUpdate>>,
    ) -> Self {
        self.inner = self.inner.set_global_secondary_index_updates(input);
        self
    }
    /// <p>Represents the DynamoDB Streams configuration for the table.</p> <note>
    /// <p>You receive a <code>ResourceInUseException</code> if you try to enable a stream on a table that already has a stream, or if you try to disable a stream on a table that doesn't have a stream.</p>
    /// </note>
    pub fn stream_specification(mut self, input: crate::types::StreamSpecification) -> Self {
        self.inner = self.inner.stream_specification(input);
        self
    }
    /// <p>Represents the DynamoDB Streams configuration for the table.</p> <note>
    /// <p>You receive a <code>ResourceInUseException</code> if you try to enable a stream on a table that already has a stream, or if you try to disable a stream on a table that doesn't have a stream.</p>
    /// </note>
    pub fn set_stream_specification(
        mut self,
        input: ::std::option::Option<crate::types::StreamSpecification>,
    ) -> Self {
        self.inner = self.inner.set_stream_specification(input);
        self
    }
    /// <p>The new server-side encryption settings for the specified table.</p>
    pub fn sse_specification(mut self, input: crate::types::SseSpecification) -> Self {
        self.inner = self.inner.sse_specification(input);
        self
    }
    /// <p>The new server-side encryption settings for the specified table.</p>
    pub fn set_sse_specification(
        mut self,
        input: ::std::option::Option<crate::types::SseSpecification>,
    ) -> Self {
        self.inner = self.inner.set_sse_specification(input);
        self
    }
    /// Appends an item to `ReplicaUpdates`.
    ///
    /// To override the contents of this collection use [`set_replica_updates`](Self::set_replica_updates).
    ///
    /// <p>A list of replica update actions (create, delete, or update) for the table.</p> <note>
    /// <p>This property only applies to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html">Version 2019.11.21 (Current)</a> of global tables. </p>
    /// </note>
    pub fn replica_updates(mut self, input: crate::types::ReplicationGroupUpdate) -> Self {
        self.inner = self.inner.replica_updates(input);
        self
    }
    /// <p>A list of replica update actions (create, delete, or update) for the table.</p> <note>
    /// <p>This property only applies to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html">Version 2019.11.21 (Current)</a> of global tables. </p>
    /// </note>
    pub fn set_replica_updates(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ReplicationGroupUpdate>>,
    ) -> Self {
        self.inner = self.inner.set_replica_updates(input);
        self
    }
    /// <p>The table class of the table to be updated. Valid values are <code>STANDARD</code> and <code>STANDARD_INFREQUENT_ACCESS</code>.</p>
    pub fn table_class(mut self, input: crate::types::TableClass) -> Self {
        self.inner = self.inner.table_class(input);
        self
    }
    /// <p>The table class of the table to be updated. Valid values are <code>STANDARD</code> and <code>STANDARD_INFREQUENT_ACCESS</code>.</p>
    pub fn set_table_class(
        mut self,
        input: ::std::option::Option<crate::types::TableClass>,
    ) -> Self {
        self.inner = self.inner.set_table_class(input);
        self
    }
    /// <p>Indicates whether deletion protection is to be enabled (true) or disabled (false) on the table.</p>
    pub fn deletion_protection_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.deletion_protection_enabled(input);
        self
    }
    /// <p>Indicates whether deletion protection is to be enabled (true) or disabled (false) on the table.</p>
    pub fn set_deletion_protection_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_deletion_protection_enabled(input);
        self
    }
}
