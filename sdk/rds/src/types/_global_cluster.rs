// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A data type representing an Aurora global database.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GlobalCluster {
    /// <p>Contains a user-supplied global database cluster identifier. This identifier is the unique key that identifies a global database cluster.</p>
    #[doc(hidden)]
    pub global_cluster_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services Region-unique, immutable identifier for the global database cluster. This identifier is found in Amazon Web Services CloudTrail log entries whenever the Amazon Web Services KMS key for the DB cluster is accessed.</p>
    #[doc(hidden)]
    pub global_cluster_resource_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) for the global database cluster.</p>
    #[doc(hidden)]
    pub global_cluster_arn: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the current state of this global database cluster.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<::std::string::String>,
    /// <p>The Aurora database engine used by the global database cluster.</p>
    #[doc(hidden)]
    pub engine: ::std::option::Option<::std::string::String>,
    /// <p>Indicates the database engine version.</p>
    #[doc(hidden)]
    pub engine_version: ::std::option::Option<::std::string::String>,
    /// <p>The default database name within the new global database cluster.</p>
    #[doc(hidden)]
    pub database_name: ::std::option::Option<::std::string::String>,
    /// <p>The storage encryption setting for the global database cluster.</p>
    #[doc(hidden)]
    pub storage_encrypted: ::std::option::Option<bool>,
    /// <p>The deletion protection setting for the new global database cluster.</p>
    #[doc(hidden)]
    pub deletion_protection: ::std::option::Option<bool>,
    /// <p>The list of primary and secondary clusters within the global database cluster.</p>
    #[doc(hidden)]
    pub global_cluster_members:
        ::std::option::Option<::std::vec::Vec<crate::types::GlobalClusterMember>>,
    /// <p>A data object containing all properties for the current state of an in-process or pending failover process for this Aurora global database. This object is empty unless the <code>FailoverGlobalCluster</code> API operation has been called on this Aurora global database (<code>GlobalCluster</code>).</p>
    #[doc(hidden)]
    pub failover_state: ::std::option::Option<crate::types::FailoverState>,
}
impl GlobalCluster {
    /// <p>Contains a user-supplied global database cluster identifier. This identifier is the unique key that identifies a global database cluster.</p>
    pub fn global_cluster_identifier(&self) -> ::std::option::Option<&str> {
        self.global_cluster_identifier.as_deref()
    }
    /// <p>The Amazon Web Services Region-unique, immutable identifier for the global database cluster. This identifier is found in Amazon Web Services CloudTrail log entries whenever the Amazon Web Services KMS key for the DB cluster is accessed.</p>
    pub fn global_cluster_resource_id(&self) -> ::std::option::Option<&str> {
        self.global_cluster_resource_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) for the global database cluster.</p>
    pub fn global_cluster_arn(&self) -> ::std::option::Option<&str> {
        self.global_cluster_arn.as_deref()
    }
    /// <p>Specifies the current state of this global database cluster.</p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>The Aurora database engine used by the global database cluster.</p>
    pub fn engine(&self) -> ::std::option::Option<&str> {
        self.engine.as_deref()
    }
    /// <p>Indicates the database engine version.</p>
    pub fn engine_version(&self) -> ::std::option::Option<&str> {
        self.engine_version.as_deref()
    }
    /// <p>The default database name within the new global database cluster.</p>
    pub fn database_name(&self) -> ::std::option::Option<&str> {
        self.database_name.as_deref()
    }
    /// <p>The storage encryption setting for the global database cluster.</p>
    pub fn storage_encrypted(&self) -> ::std::option::Option<bool> {
        self.storage_encrypted
    }
    /// <p>The deletion protection setting for the new global database cluster.</p>
    pub fn deletion_protection(&self) -> ::std::option::Option<bool> {
        self.deletion_protection
    }
    /// <p>The list of primary and secondary clusters within the global database cluster.</p>
    pub fn global_cluster_members(
        &self,
    ) -> ::std::option::Option<&[crate::types::GlobalClusterMember]> {
        self.global_cluster_members.as_deref()
    }
    /// <p>A data object containing all properties for the current state of an in-process or pending failover process for this Aurora global database. This object is empty unless the <code>FailoverGlobalCluster</code> API operation has been called on this Aurora global database (<code>GlobalCluster</code>).</p>
    pub fn failover_state(&self) -> ::std::option::Option<&crate::types::FailoverState> {
        self.failover_state.as_ref()
    }
}
impl GlobalCluster {
    /// Creates a new builder-style object to manufacture [`GlobalCluster`](crate::types::GlobalCluster).
    pub fn builder() -> crate::types::builders::GlobalClusterBuilder {
        crate::types::builders::GlobalClusterBuilder::default()
    }
}

/// A builder for [`GlobalCluster`](crate::types::GlobalCluster).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GlobalClusterBuilder {
    pub(crate) global_cluster_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) global_cluster_resource_id: ::std::option::Option<::std::string::String>,
    pub(crate) global_cluster_arn: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<::std::string::String>,
    pub(crate) engine: ::std::option::Option<::std::string::String>,
    pub(crate) engine_version: ::std::option::Option<::std::string::String>,
    pub(crate) database_name: ::std::option::Option<::std::string::String>,
    pub(crate) storage_encrypted: ::std::option::Option<bool>,
    pub(crate) deletion_protection: ::std::option::Option<bool>,
    pub(crate) global_cluster_members:
        ::std::option::Option<::std::vec::Vec<crate::types::GlobalClusterMember>>,
    pub(crate) failover_state: ::std::option::Option<crate::types::FailoverState>,
}
impl GlobalClusterBuilder {
    /// <p>Contains a user-supplied global database cluster identifier. This identifier is the unique key that identifies a global database cluster.</p>
    pub fn global_cluster_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.global_cluster_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Contains a user-supplied global database cluster identifier. This identifier is the unique key that identifies a global database cluster.</p>
    pub fn set_global_cluster_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.global_cluster_identifier = input;
        self
    }
    /// <p>The Amazon Web Services Region-unique, immutable identifier for the global database cluster. This identifier is found in Amazon Web Services CloudTrail log entries whenever the Amazon Web Services KMS key for the DB cluster is accessed.</p>
    pub fn global_cluster_resource_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.global_cluster_resource_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services Region-unique, immutable identifier for the global database cluster. This identifier is found in Amazon Web Services CloudTrail log entries whenever the Amazon Web Services KMS key for the DB cluster is accessed.</p>
    pub fn set_global_cluster_resource_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.global_cluster_resource_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the global database cluster.</p>
    pub fn global_cluster_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.global_cluster_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the global database cluster.</p>
    pub fn set_global_cluster_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.global_cluster_arn = input;
        self
    }
    /// <p>Specifies the current state of this global database cluster.</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the current state of this global database cluster.</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// <p>The Aurora database engine used by the global database cluster.</p>
    pub fn engine(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.engine = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Aurora database engine used by the global database cluster.</p>
    pub fn set_engine(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.engine = input;
        self
    }
    /// <p>Indicates the database engine version.</p>
    pub fn engine_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.engine_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Indicates the database engine version.</p>
    pub fn set_engine_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.engine_version = input;
        self
    }
    /// <p>The default database name within the new global database cluster.</p>
    pub fn database_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.database_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The default database name within the new global database cluster.</p>
    pub fn set_database_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.database_name = input;
        self
    }
    /// <p>The storage encryption setting for the global database cluster.</p>
    pub fn storage_encrypted(mut self, input: bool) -> Self {
        self.storage_encrypted = ::std::option::Option::Some(input);
        self
    }
    /// <p>The storage encryption setting for the global database cluster.</p>
    pub fn set_storage_encrypted(mut self, input: ::std::option::Option<bool>) -> Self {
        self.storage_encrypted = input;
        self
    }
    /// <p>The deletion protection setting for the new global database cluster.</p>
    pub fn deletion_protection(mut self, input: bool) -> Self {
        self.deletion_protection = ::std::option::Option::Some(input);
        self
    }
    /// <p>The deletion protection setting for the new global database cluster.</p>
    pub fn set_deletion_protection(mut self, input: ::std::option::Option<bool>) -> Self {
        self.deletion_protection = input;
        self
    }
    /// Appends an item to `global_cluster_members`.
    ///
    /// To override the contents of this collection use [`set_global_cluster_members`](Self::set_global_cluster_members).
    ///
    /// <p>The list of primary and secondary clusters within the global database cluster.</p>
    pub fn global_cluster_members(mut self, input: crate::types::GlobalClusterMember) -> Self {
        let mut v = self.global_cluster_members.unwrap_or_default();
        v.push(input);
        self.global_cluster_members = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of primary and secondary clusters within the global database cluster.</p>
    pub fn set_global_cluster_members(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::GlobalClusterMember>>,
    ) -> Self {
        self.global_cluster_members = input;
        self
    }
    /// <p>A data object containing all properties for the current state of an in-process or pending failover process for this Aurora global database. This object is empty unless the <code>FailoverGlobalCluster</code> API operation has been called on this Aurora global database (<code>GlobalCluster</code>).</p>
    pub fn failover_state(mut self, input: crate::types::FailoverState) -> Self {
        self.failover_state = ::std::option::Option::Some(input);
        self
    }
    /// <p>A data object containing all properties for the current state of an in-process or pending failover process for this Aurora global database. This object is empty unless the <code>FailoverGlobalCluster</code> API operation has been called on this Aurora global database (<code>GlobalCluster</code>).</p>
    pub fn set_failover_state(
        mut self,
        input: ::std::option::Option<crate::types::FailoverState>,
    ) -> Self {
        self.failover_state = input;
        self
    }
    /// Consumes the builder and constructs a [`GlobalCluster`](crate::types::GlobalCluster).
    pub fn build(self) -> crate::types::GlobalCluster {
        crate::types::GlobalCluster {
            global_cluster_identifier: self.global_cluster_identifier,
            global_cluster_resource_id: self.global_cluster_resource_id,
            global_cluster_arn: self.global_cluster_arn,
            status: self.status,
            engine: self.engine,
            engine_version: self.engine_version,
            database_name: self.database_name,
            storage_encrypted: self.storage_encrypted,
            deletion_protection: self.deletion_protection,
            global_cluster_members: self.global_cluster_members,
            failover_state: self.failover_state,
        }
    }
}
