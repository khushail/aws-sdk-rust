// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IncreaseReplicaCountInput {
    /// <p>The id of the replication group to which you want to add replica nodes.</p>
    #[doc(hidden)]
    pub replication_group_id: ::std::option::Option<::std::string::String>,
    /// <p>The number of read replica nodes you want at the completion of this operation. For Redis (cluster mode disabled) replication groups, this is the number of replica nodes in the replication group. For Redis (cluster mode enabled) replication groups, this is the number of replica nodes in each of the replication group's node groups.</p>
    #[doc(hidden)]
    pub new_replica_count: ::std::option::Option<i32>,
    /// <p>A list of <code>ConfigureShard</code> objects that can be used to configure each shard in a Redis (cluster mode enabled) replication group. The <code>ConfigureShard</code> has three members: <code>NewReplicaCount</code>, <code>NodeGroupId</code>, and <code>PreferredAvailabilityZones</code>.</p>
    #[doc(hidden)]
    pub replica_configuration: ::std::option::Option<::std::vec::Vec<crate::types::ConfigureShard>>,
    /// <p>If <code>True</code>, the number of replica nodes is increased immediately. <code>ApplyImmediately=False</code> is not currently supported.</p>
    #[doc(hidden)]
    pub apply_immediately: bool,
}
impl IncreaseReplicaCountInput {
    /// <p>The id of the replication group to which you want to add replica nodes.</p>
    pub fn replication_group_id(&self) -> ::std::option::Option<&str> {
        self.replication_group_id.as_deref()
    }
    /// <p>The number of read replica nodes you want at the completion of this operation. For Redis (cluster mode disabled) replication groups, this is the number of replica nodes in the replication group. For Redis (cluster mode enabled) replication groups, this is the number of replica nodes in each of the replication group's node groups.</p>
    pub fn new_replica_count(&self) -> ::std::option::Option<i32> {
        self.new_replica_count
    }
    /// <p>A list of <code>ConfigureShard</code> objects that can be used to configure each shard in a Redis (cluster mode enabled) replication group. The <code>ConfigureShard</code> has three members: <code>NewReplicaCount</code>, <code>NodeGroupId</code>, and <code>PreferredAvailabilityZones</code>.</p>
    pub fn replica_configuration(&self) -> ::std::option::Option<&[crate::types::ConfigureShard]> {
        self.replica_configuration.as_deref()
    }
    /// <p>If <code>True</code>, the number of replica nodes is increased immediately. <code>ApplyImmediately=False</code> is not currently supported.</p>
    pub fn apply_immediately(&self) -> bool {
        self.apply_immediately
    }
}
impl IncreaseReplicaCountInput {
    /// Creates a new builder-style object to manufacture [`IncreaseReplicaCountInput`](crate::operation::increase_replica_count::IncreaseReplicaCountInput).
    pub fn builder(
    ) -> crate::operation::increase_replica_count::builders::IncreaseReplicaCountInputBuilder {
        crate::operation::increase_replica_count::builders::IncreaseReplicaCountInputBuilder::default()
    }
}

/// A builder for [`IncreaseReplicaCountInput`](crate::operation::increase_replica_count::IncreaseReplicaCountInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct IncreaseReplicaCountInputBuilder {
    pub(crate) replication_group_id: ::std::option::Option<::std::string::String>,
    pub(crate) new_replica_count: ::std::option::Option<i32>,
    pub(crate) replica_configuration:
        ::std::option::Option<::std::vec::Vec<crate::types::ConfigureShard>>,
    pub(crate) apply_immediately: ::std::option::Option<bool>,
}
impl IncreaseReplicaCountInputBuilder {
    /// <p>The id of the replication group to which you want to add replica nodes.</p>
    pub fn replication_group_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.replication_group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The id of the replication group to which you want to add replica nodes.</p>
    pub fn set_replication_group_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.replication_group_id = input;
        self
    }
    /// <p>The number of read replica nodes you want at the completion of this operation. For Redis (cluster mode disabled) replication groups, this is the number of replica nodes in the replication group. For Redis (cluster mode enabled) replication groups, this is the number of replica nodes in each of the replication group's node groups.</p>
    pub fn new_replica_count(mut self, input: i32) -> Self {
        self.new_replica_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of read replica nodes you want at the completion of this operation. For Redis (cluster mode disabled) replication groups, this is the number of replica nodes in the replication group. For Redis (cluster mode enabled) replication groups, this is the number of replica nodes in each of the replication group's node groups.</p>
    pub fn set_new_replica_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.new_replica_count = input;
        self
    }
    /// Appends an item to `replica_configuration`.
    ///
    /// To override the contents of this collection use [`set_replica_configuration`](Self::set_replica_configuration).
    ///
    /// <p>A list of <code>ConfigureShard</code> objects that can be used to configure each shard in a Redis (cluster mode enabled) replication group. The <code>ConfigureShard</code> has three members: <code>NewReplicaCount</code>, <code>NodeGroupId</code>, and <code>PreferredAvailabilityZones</code>.</p>
    pub fn replica_configuration(mut self, input: crate::types::ConfigureShard) -> Self {
        let mut v = self.replica_configuration.unwrap_or_default();
        v.push(input);
        self.replica_configuration = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>ConfigureShard</code> objects that can be used to configure each shard in a Redis (cluster mode enabled) replication group. The <code>ConfigureShard</code> has three members: <code>NewReplicaCount</code>, <code>NodeGroupId</code>, and <code>PreferredAvailabilityZones</code>.</p>
    pub fn set_replica_configuration(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ConfigureShard>>,
    ) -> Self {
        self.replica_configuration = input;
        self
    }
    /// <p>If <code>True</code>, the number of replica nodes is increased immediately. <code>ApplyImmediately=False</code> is not currently supported.</p>
    pub fn apply_immediately(mut self, input: bool) -> Self {
        self.apply_immediately = ::std::option::Option::Some(input);
        self
    }
    /// <p>If <code>True</code>, the number of replica nodes is increased immediately. <code>ApplyImmediately=False</code> is not currently supported.</p>
    pub fn set_apply_immediately(mut self, input: ::std::option::Option<bool>) -> Self {
        self.apply_immediately = input;
        self
    }
    /// Consumes the builder and constructs a [`IncreaseReplicaCountInput`](crate::operation::increase_replica_count::IncreaseReplicaCountInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::increase_replica_count::IncreaseReplicaCountInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::increase_replica_count::IncreaseReplicaCountInput {
                replication_group_id: self.replication_group_id,
                new_replica_count: self.new_replica_count,
                replica_configuration: self.replica_configuration,
                apply_immediately: self.apply_immediately.unwrap_or_default(),
            },
        )
    }
}
