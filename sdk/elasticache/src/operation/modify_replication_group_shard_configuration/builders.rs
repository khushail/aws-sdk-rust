// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_replication_group_shard_configuration::_modify_replication_group_shard_configuration_output::ModifyReplicationGroupShardConfigurationOutputBuilder;

pub use crate::operation::modify_replication_group_shard_configuration::_modify_replication_group_shard_configuration_input::ModifyReplicationGroupShardConfigurationInputBuilder;

/// Fluent builder constructing a request to `ModifyReplicationGroupShardConfiguration`.
///
/// <p>Modifies a replication group's shards (node groups) by allowing you to add shards, remove shards, or rebalance the keyspaces among existing shards.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyReplicationGroupShardConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::modify_replication_group_shard_configuration::builders::ModifyReplicationGroupShardConfigurationInputBuilder,
}
impl ModifyReplicationGroupShardConfigurationFluentBuilder {
    /// Creates a new `ModifyReplicationGroupShardConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::modify_replication_group_shard_configuration::ModifyReplicationGroupShardConfiguration, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::modify_replication_group_shard_configuration::ModifyReplicationGroupShardConfigurationError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::modify_replication_group_shard_configuration::ModifyReplicationGroupShardConfigurationOutput, ::aws_smithy_http::result::SdkError<crate::operation::modify_replication_group_shard_configuration::ModifyReplicationGroupShardConfigurationError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::modify_replication_group_shard_configuration::ModifyReplicationGroupShardConfigurationOutput, ::aws_smithy_http::result::SdkError<crate::operation::modify_replication_group_shard_configuration::ModifyReplicationGroupShardConfigurationError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::modify_replication_group_shard_configuration::ModifyReplicationGroupShardConfiguration, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::modify_replication_group_shard_configuration::ModifyReplicationGroupShardConfigurationError>
    >{
        self.customize_middleware().await
    }
    /// <p>The name of the Redis (cluster mode enabled) cluster (replication group) on which the shards are to be configured.</p>
    pub fn replication_group_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.replication_group_id(input.into());
        self
    }
    /// <p>The name of the Redis (cluster mode enabled) cluster (replication group) on which the shards are to be configured.</p>
    pub fn set_replication_group_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_replication_group_id(input);
        self
    }
    /// <p>The number of node groups (shards) that results from the modification of the shard configuration.</p>
    pub fn node_group_count(mut self, input: i32) -> Self {
        self.inner = self.inner.node_group_count(input);
        self
    }
    /// <p>The number of node groups (shards) that results from the modification of the shard configuration.</p>
    pub fn set_node_group_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_node_group_count(input);
        self
    }
    /// <p>Indicates that the shard reconfiguration process begins immediately. At present, the only permitted value for this parameter is <code>true</code>.</p>
    /// <p>Value: true</p>
    pub fn apply_immediately(mut self, input: bool) -> Self {
        self.inner = self.inner.apply_immediately(input);
        self
    }
    /// <p>Indicates that the shard reconfiguration process begins immediately. At present, the only permitted value for this parameter is <code>true</code>.</p>
    /// <p>Value: true</p>
    pub fn set_apply_immediately(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_apply_immediately(input);
        self
    }
    /// Appends an item to `ReshardingConfiguration`.
    ///
    /// To override the contents of this collection use [`set_resharding_configuration`](Self::set_resharding_configuration).
    ///
    /// <p>Specifies the preferred availability zones for each node group in the cluster. If the value of <code>NodeGroupCount</code> is greater than the current number of node groups (shards), you can use this parameter to specify the preferred availability zones of the cluster's shards. If you omit this parameter ElastiCache selects availability zones for you.</p>
    /// <p>You can specify this parameter only if the value of <code>NodeGroupCount</code> is greater than the current number of node groups (shards).</p>
    pub fn resharding_configuration(
        mut self,
        input: crate::types::ReshardingConfiguration,
    ) -> Self {
        self.inner = self.inner.resharding_configuration(input);
        self
    }
    /// <p>Specifies the preferred availability zones for each node group in the cluster. If the value of <code>NodeGroupCount</code> is greater than the current number of node groups (shards), you can use this parameter to specify the preferred availability zones of the cluster's shards. If you omit this parameter ElastiCache selects availability zones for you.</p>
    /// <p>You can specify this parameter only if the value of <code>NodeGroupCount</code> is greater than the current number of node groups (shards).</p>
    pub fn set_resharding_configuration(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ReshardingConfiguration>>,
    ) -> Self {
        self.inner = self.inner.set_resharding_configuration(input);
        self
    }
    /// Appends an item to `NodeGroupsToRemove`.
    ///
    /// To override the contents of this collection use [`set_node_groups_to_remove`](Self::set_node_groups_to_remove).
    ///
    /// <p>If the value of <code>NodeGroupCount</code> is less than the current number of node groups (shards), then either <code>NodeGroupsToRemove</code> or <code>NodeGroupsToRetain</code> is required. <code>NodeGroupsToRemove</code> is a list of <code>NodeGroupId</code>s to remove from the cluster.</p>
    /// <p>ElastiCache for Redis will attempt to remove all node groups listed by <code>NodeGroupsToRemove</code> from the cluster.</p>
    pub fn node_groups_to_remove(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.node_groups_to_remove(input.into());
        self
    }
    /// <p>If the value of <code>NodeGroupCount</code> is less than the current number of node groups (shards), then either <code>NodeGroupsToRemove</code> or <code>NodeGroupsToRetain</code> is required. <code>NodeGroupsToRemove</code> is a list of <code>NodeGroupId</code>s to remove from the cluster.</p>
    /// <p>ElastiCache for Redis will attempt to remove all node groups listed by <code>NodeGroupsToRemove</code> from the cluster.</p>
    pub fn set_node_groups_to_remove(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_node_groups_to_remove(input);
        self
    }
    /// Appends an item to `NodeGroupsToRetain`.
    ///
    /// To override the contents of this collection use [`set_node_groups_to_retain`](Self::set_node_groups_to_retain).
    ///
    /// <p>If the value of <code>NodeGroupCount</code> is less than the current number of node groups (shards), then either <code>NodeGroupsToRemove</code> or <code>NodeGroupsToRetain</code> is required. <code>NodeGroupsToRetain</code> is a list of <code>NodeGroupId</code>s to retain in the cluster.</p>
    /// <p>ElastiCache for Redis will attempt to remove all node groups except those listed by <code>NodeGroupsToRetain</code> from the cluster.</p>
    pub fn node_groups_to_retain(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.node_groups_to_retain(input.into());
        self
    }
    /// <p>If the value of <code>NodeGroupCount</code> is less than the current number of node groups (shards), then either <code>NodeGroupsToRemove</code> or <code>NodeGroupsToRetain</code> is required. <code>NodeGroupsToRetain</code> is a list of <code>NodeGroupId</code>s to retain in the cluster.</p>
    /// <p>ElastiCache for Redis will attempt to remove all node groups except those listed by <code>NodeGroupsToRetain</code> from the cluster.</p>
    pub fn set_node_groups_to_retain(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_node_groups_to_retain(input);
        self
    }
}
