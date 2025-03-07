// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_db_instance::_create_db_instance_output::CreateDbInstanceOutputBuilder;

pub use crate::operation::create_db_instance::_create_db_instance_input::CreateDbInstanceInputBuilder;

/// Fluent builder constructing a request to `CreateDBInstance`.
///
/// <p>Creates a new DB instance.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDBInstanceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_db_instance::builders::CreateDbInstanceInputBuilder,
}
impl CreateDBInstanceFluentBuilder {
    /// Creates a new `CreateDBInstance`.
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
            crate::operation::create_db_instance::CreateDBInstance,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_db_instance::CreateDBInstanceError,
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
        crate::operation::create_db_instance::CreateDbInstanceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_db_instance::CreateDBInstanceError,
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
        crate::operation::create_db_instance::CreateDbInstanceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_db_instance::CreateDBInstanceError,
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
            crate::operation::create_db_instance::CreateDBInstance,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_db_instance::CreateDBInstanceError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Not supported.</p>
    pub fn db_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.db_name(input.into());
        self
    }
    /// <p>Not supported.</p>
    pub fn set_db_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_db_name(input);
        self
    }
    /// <p>The DB instance identifier. This parameter is stored as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li>
    /// <li> <p>First character must be a letter.</p> </li>
    /// <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li>
    /// </ul>
    /// <p>Example: <code>mydbinstance</code> </p>
    pub fn db_instance_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.db_instance_identifier(input.into());
        self
    }
    /// <p>The DB instance identifier. This parameter is stored as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li>
    /// <li> <p>First character must be a letter.</p> </li>
    /// <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li>
    /// </ul>
    /// <p>Example: <code>mydbinstance</code> </p>
    pub fn set_db_instance_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_db_instance_identifier(input);
        self
    }
    /// <p>Not supported by Neptune.</p>
    pub fn allocated_storage(mut self, input: i32) -> Self {
        self.inner = self.inner.allocated_storage(input);
        self
    }
    /// <p>Not supported by Neptune.</p>
    pub fn set_allocated_storage(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_allocated_storage(input);
        self
    }
    /// <p>The compute and memory capacity of the DB instance, for example, <code>db.m4.large</code>. Not all DB instance classes are available in all Amazon Regions.</p>
    pub fn db_instance_class(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.db_instance_class(input.into());
        self
    }
    /// <p>The compute and memory capacity of the DB instance, for example, <code>db.m4.large</code>. Not all DB instance classes are available in all Amazon Regions.</p>
    pub fn set_db_instance_class(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_db_instance_class(input);
        self
    }
    /// <p>The name of the database engine to be used for this instance.</p>
    /// <p>Valid Values: <code>neptune</code> </p>
    pub fn engine(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.engine(input.into());
        self
    }
    /// <p>The name of the database engine to be used for this instance.</p>
    /// <p>Valid Values: <code>neptune</code> </p>
    pub fn set_engine(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_engine(input);
        self
    }
    /// <p>Not supported by Neptune.</p>
    pub fn master_username(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.master_username(input.into());
        self
    }
    /// <p>Not supported by Neptune.</p>
    pub fn set_master_username(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_master_username(input);
        self
    }
    /// <p>Not supported by Neptune.</p>
    pub fn master_user_password(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.master_user_password(input.into());
        self
    }
    /// <p>Not supported by Neptune.</p>
    pub fn set_master_user_password(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_master_user_password(input);
        self
    }
    /// Appends an item to `DBSecurityGroups`.
    ///
    /// To override the contents of this collection use [`set_db_security_groups`](Self::set_db_security_groups).
    ///
    /// <p>A list of DB security groups to associate with this DB instance.</p>
    /// <p>Default: The default DB security group for the database engine.</p>
    pub fn db_security_groups(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.db_security_groups(input.into());
        self
    }
    /// <p>A list of DB security groups to associate with this DB instance.</p>
    /// <p>Default: The default DB security group for the database engine.</p>
    pub fn set_db_security_groups(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_db_security_groups(input);
        self
    }
    /// Appends an item to `VpcSecurityGroupIds`.
    ///
    /// To override the contents of this collection use [`set_vpc_security_group_ids`](Self::set_vpc_security_group_ids).
    ///
    /// <p>A list of EC2 VPC security groups to associate with this DB instance.</p>
    /// <p>Not applicable. The associated list of EC2 VPC security groups is managed by the DB cluster. For more information, see <code>CreateDBCluster</code>.</p>
    /// <p>Default: The default EC2 VPC security group for the DB subnet group's VPC.</p>
    pub fn vpc_security_group_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.vpc_security_group_ids(input.into());
        self
    }
    /// <p>A list of EC2 VPC security groups to associate with this DB instance.</p>
    /// <p>Not applicable. The associated list of EC2 VPC security groups is managed by the DB cluster. For more information, see <code>CreateDBCluster</code>.</p>
    /// <p>Default: The default EC2 VPC security group for the DB subnet group's VPC.</p>
    pub fn set_vpc_security_group_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_vpc_security_group_ids(input);
        self
    }
    /// <p> The EC2 Availability Zone that the DB instance is created in</p>
    /// <p>Default: A random, system-chosen Availability Zone in the endpoint's Amazon Region.</p>
    /// <p> Example: <code>us-east-1d</code> </p>
    /// <p> Constraint: The AvailabilityZone parameter can't be specified if the MultiAZ parameter is set to <code>true</code>. The specified Availability Zone must be in the same Amazon Region as the current endpoint.</p>
    pub fn availability_zone(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.availability_zone(input.into());
        self
    }
    /// <p> The EC2 Availability Zone that the DB instance is created in</p>
    /// <p>Default: A random, system-chosen Availability Zone in the endpoint's Amazon Region.</p>
    /// <p> Example: <code>us-east-1d</code> </p>
    /// <p> Constraint: The AvailabilityZone parameter can't be specified if the MultiAZ parameter is set to <code>true</code>. The specified Availability Zone must be in the same Amazon Region as the current endpoint.</p>
    pub fn set_availability_zone(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_availability_zone(input);
        self
    }
    /// <p>A DB subnet group to associate with this DB instance.</p>
    /// <p>If there is no DB subnet group, then it is a non-VPC DB instance.</p>
    pub fn db_subnet_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.db_subnet_group_name(input.into());
        self
    }
    /// <p>A DB subnet group to associate with this DB instance.</p>
    /// <p>If there is no DB subnet group, then it is a non-VPC DB instance.</p>
    pub fn set_db_subnet_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_db_subnet_group_name(input);
        self
    }
    /// <p>The time range each week during which system maintenance can occur, in Universal Coordinated Time (UTC).</p>
    /// <p> Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p>
    /// <p>The default is a 30-minute window selected at random from an 8-hour block of time for each Amazon Region, occurring on a random day of the week.</p>
    /// <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p>
    /// <p>Constraints: Minimum 30-minute window.</p>
    pub fn preferred_maintenance_window(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.preferred_maintenance_window(input.into());
        self
    }
    /// <p>The time range each week during which system maintenance can occur, in Universal Coordinated Time (UTC).</p>
    /// <p> Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p>
    /// <p>The default is a 30-minute window selected at random from an 8-hour block of time for each Amazon Region, occurring on a random day of the week.</p>
    /// <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p>
    /// <p>Constraints: Minimum 30-minute window.</p>
    pub fn set_preferred_maintenance_window(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_preferred_maintenance_window(input);
        self
    }
    /// <p>The name of the DB parameter group to associate with this DB instance. If this argument is omitted, the default DBParameterGroup for the specified engine is used.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must be 1 to 255 letters, numbers, or hyphens.</p> </li>
    /// <li> <p>First character must be a letter</p> </li>
    /// <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li>
    /// </ul>
    pub fn db_parameter_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.db_parameter_group_name(input.into());
        self
    }
    /// <p>The name of the DB parameter group to associate with this DB instance. If this argument is omitted, the default DBParameterGroup for the specified engine is used.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must be 1 to 255 letters, numbers, or hyphens.</p> </li>
    /// <li> <p>First character must be a letter</p> </li>
    /// <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li>
    /// </ul>
    pub fn set_db_parameter_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_db_parameter_group_name(input);
        self
    }
    /// <p>The number of days for which automated backups are retained.</p>
    /// <p>Not applicable. The retention period for automated backups is managed by the DB cluster. For more information, see <code>CreateDBCluster</code>.</p>
    /// <p>Default: 1</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must be a value from 0 to 35</p> </li>
    /// <li> <p>Cannot be set to 0 if the DB instance is a source to Read Replicas</p> </li>
    /// </ul>
    pub fn backup_retention_period(mut self, input: i32) -> Self {
        self.inner = self.inner.backup_retention_period(input);
        self
    }
    /// <p>The number of days for which automated backups are retained.</p>
    /// <p>Not applicable. The retention period for automated backups is managed by the DB cluster. For more information, see <code>CreateDBCluster</code>.</p>
    /// <p>Default: 1</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must be a value from 0 to 35</p> </li>
    /// <li> <p>Cannot be set to 0 if the DB instance is a source to Read Replicas</p> </li>
    /// </ul>
    pub fn set_backup_retention_period(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_backup_retention_period(input);
        self
    }
    /// <p> The daily time range during which automated backups are created.</p>
    /// <p>Not applicable. The daily time range for creating automated backups is managed by the DB cluster. For more information, see <code>CreateDBCluster</code>.</p>
    pub fn preferred_backup_window(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.preferred_backup_window(input.into());
        self
    }
    /// <p> The daily time range during which automated backups are created.</p>
    /// <p>Not applicable. The daily time range for creating automated backups is managed by the DB cluster. For more information, see <code>CreateDBCluster</code>.</p>
    pub fn set_preferred_backup_window(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_preferred_backup_window(input);
        self
    }
    /// <p>The port number on which the database accepts connections.</p>
    /// <p>Not applicable. The port is managed by the DB cluster. For more information, see <code>CreateDBCluster</code>.</p>
    /// <p> Default: <code>8182</code> </p>
    /// <p>Type: Integer</p>
    pub fn port(mut self, input: i32) -> Self {
        self.inner = self.inner.port(input);
        self
    }
    /// <p>The port number on which the database accepts connections.</p>
    /// <p>Not applicable. The port is managed by the DB cluster. For more information, see <code>CreateDBCluster</code>.</p>
    /// <p> Default: <code>8182</code> </p>
    /// <p>Type: Integer</p>
    pub fn set_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_port(input);
        self
    }
    /// <p>Specifies if the DB instance is a Multi-AZ deployment. You can't set the AvailabilityZone parameter if the MultiAZ parameter is set to true.</p>
    pub fn multi_az(mut self, input: bool) -> Self {
        self.inner = self.inner.multi_az(input);
        self
    }
    /// <p>Specifies if the DB instance is a Multi-AZ deployment. You can't set the AvailabilityZone parameter if the MultiAZ parameter is set to true.</p>
    pub fn set_multi_az(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_multi_az(input);
        self
    }
    /// <p>The version number of the database engine to use. Currently, setting this parameter has no effect.</p>
    pub fn engine_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.engine_version(input.into());
        self
    }
    /// <p>The version number of the database engine to use. Currently, setting this parameter has no effect.</p>
    pub fn set_engine_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_engine_version(input);
        self
    }
    /// <p>Indicates that minor engine upgrades are applied automatically to the DB instance during the maintenance window.</p>
    /// <p>Default: <code>true</code> </p>
    pub fn auto_minor_version_upgrade(mut self, input: bool) -> Self {
        self.inner = self.inner.auto_minor_version_upgrade(input);
        self
    }
    /// <p>Indicates that minor engine upgrades are applied automatically to the DB instance during the maintenance window.</p>
    /// <p>Default: <code>true</code> </p>
    pub fn set_auto_minor_version_upgrade(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_auto_minor_version_upgrade(input);
        self
    }
    /// <p>License model information for this DB instance.</p>
    /// <p> Valid values: <code>license-included</code> | <code>bring-your-own-license</code> | <code>general-public-license</code> </p>
    pub fn license_model(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.license_model(input.into());
        self
    }
    /// <p>License model information for this DB instance.</p>
    /// <p> Valid values: <code>license-included</code> | <code>bring-your-own-license</code> | <code>general-public-license</code> </p>
    pub fn set_license_model(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_license_model(input);
        self
    }
    /// <p>The amount of Provisioned IOPS (input/output operations per second) to be initially allocated for the DB instance.</p>
    pub fn iops(mut self, input: i32) -> Self {
        self.inner = self.inner.iops(input);
        self
    }
    /// <p>The amount of Provisioned IOPS (input/output operations per second) to be initially allocated for the DB instance.</p>
    pub fn set_iops(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_iops(input);
        self
    }
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub fn option_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.option_group_name(input.into());
        self
    }
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub fn set_option_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_option_group_name(input);
        self
    }
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub fn character_set_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.character_set_name(input.into());
        self
    }
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub fn set_character_set_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_character_set_name(input);
        self
    }
    /// <p>This flag should no longer be used.</p>
    #[deprecated]
    pub fn publicly_accessible(mut self, input: bool) -> Self {
        self.inner = self.inner.publicly_accessible(input);
        self
    }
    /// <p>This flag should no longer be used.</p>
    #[deprecated]
    pub fn set_publicly_accessible(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_publicly_accessible(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to assign to the new instance.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags to assign to the new instance.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The identifier of the DB cluster that the instance will belong to.</p>
    /// <p>For information on creating a DB cluster, see <code>CreateDBCluster</code>.</p>
    /// <p>Type: String</p>
    pub fn db_cluster_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.db_cluster_identifier(input.into());
        self
    }
    /// <p>The identifier of the DB cluster that the instance will belong to.</p>
    /// <p>For information on creating a DB cluster, see <code>CreateDBCluster</code>.</p>
    /// <p>Type: String</p>
    pub fn set_db_cluster_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_db_cluster_identifier(input);
        self
    }
    /// <p>Specifies the storage type to be associated with the DB instance.</p>
    /// <p>Not applicable. Storage is managed by the DB Cluster.</p>
    pub fn storage_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.storage_type(input.into());
        self
    }
    /// <p>Specifies the storage type to be associated with the DB instance.</p>
    /// <p>Not applicable. Storage is managed by the DB Cluster.</p>
    pub fn set_storage_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_storage_type(input);
        self
    }
    /// <p>The ARN from the key store with which to associate the instance for TDE encryption.</p>
    pub fn tde_credential_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.tde_credential_arn(input.into());
        self
    }
    /// <p>The ARN from the key store with which to associate the instance for TDE encryption.</p>
    pub fn set_tde_credential_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_tde_credential_arn(input);
        self
    }
    /// <p>The password for the given ARN from the key store in order to access the device.</p>
    pub fn tde_credential_password(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.tde_credential_password(input.into());
        self
    }
    /// <p>The password for the given ARN from the key store in order to access the device.</p>
    pub fn set_tde_credential_password(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_tde_credential_password(input);
        self
    }
    /// <p>Specifies whether the DB instance is encrypted.</p>
    /// <p>Not applicable. The encryption for DB instances is managed by the DB cluster. For more information, see <code>CreateDBCluster</code>.</p>
    /// <p>Default: false</p>
    pub fn storage_encrypted(mut self, input: bool) -> Self {
        self.inner = self.inner.storage_encrypted(input);
        self
    }
    /// <p>Specifies whether the DB instance is encrypted.</p>
    /// <p>Not applicable. The encryption for DB instances is managed by the DB cluster. For more information, see <code>CreateDBCluster</code>.</p>
    /// <p>Default: false</p>
    pub fn set_storage_encrypted(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_storage_encrypted(input);
        self
    }
    /// <p>The Amazon KMS key identifier for an encrypted DB instance.</p>
    /// <p>The KMS key identifier is the Amazon Resource Name (ARN) for the KMS encryption key. If you are creating a DB instance with the same Amazon account that owns the KMS encryption key used to encrypt the new DB instance, then you can use the KMS key alias instead of the ARN for the KM encryption key.</p>
    /// <p>Not applicable. The KMS key identifier is managed by the DB cluster. For more information, see <code>CreateDBCluster</code>.</p>
    /// <p>If the <code>StorageEncrypted</code> parameter is true, and you do not specify a value for the <code>KmsKeyId</code> parameter, then Amazon Neptune will use your default encryption key. Amazon KMS creates the default encryption key for your Amazon account. Your Amazon account has a different default encryption key for each Amazon Region.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_id(input.into());
        self
    }
    /// <p>The Amazon KMS key identifier for an encrypted DB instance.</p>
    /// <p>The KMS key identifier is the Amazon Resource Name (ARN) for the KMS encryption key. If you are creating a DB instance with the same Amazon account that owns the KMS encryption key used to encrypt the new DB instance, then you can use the KMS key alias instead of the ARN for the KM encryption key.</p>
    /// <p>Not applicable. The KMS key identifier is managed by the DB cluster. For more information, see <code>CreateDBCluster</code>.</p>
    /// <p>If the <code>StorageEncrypted</code> parameter is true, and you do not specify a value for the <code>KmsKeyId</code> parameter, then Amazon Neptune will use your default encryption key. Amazon KMS creates the default encryption key for your Amazon account. Your Amazon account has a different default encryption key for each Amazon Region.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_id(input);
        self
    }
    /// <p>Specify the Active Directory Domain to create the instance in.</p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain(input.into());
        self
    }
    /// <p>Specify the Active Directory Domain to create the instance in.</p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain(input);
        self
    }
    /// <p>True to copy all tags from the DB instance to snapshots of the DB instance, and otherwise false. The default is false.</p>
    pub fn copy_tags_to_snapshot(mut self, input: bool) -> Self {
        self.inner = self.inner.copy_tags_to_snapshot(input);
        self
    }
    /// <p>True to copy all tags from the DB instance to snapshots of the DB instance, and otherwise false. The default is false.</p>
    pub fn set_copy_tags_to_snapshot(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_copy_tags_to_snapshot(input);
        self
    }
    /// <p>The interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB instance. To disable collecting Enhanced Monitoring metrics, specify 0. The default is 0.</p>
    /// <p>If <code>MonitoringRoleArn</code> is specified, then you must also set <code>MonitoringInterval</code> to a value other than 0.</p>
    /// <p>Valid Values: <code>0, 1, 5, 10, 15, 30, 60</code> </p>
    pub fn monitoring_interval(mut self, input: i32) -> Self {
        self.inner = self.inner.monitoring_interval(input);
        self
    }
    /// <p>The interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB instance. To disable collecting Enhanced Monitoring metrics, specify 0. The default is 0.</p>
    /// <p>If <code>MonitoringRoleArn</code> is specified, then you must also set <code>MonitoringInterval</code> to a value other than 0.</p>
    /// <p>Valid Values: <code>0, 1, 5, 10, 15, 30, 60</code> </p>
    pub fn set_monitoring_interval(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_monitoring_interval(input);
        self
    }
    /// <p>The ARN for the IAM role that permits Neptune to send enhanced monitoring metrics to Amazon CloudWatch Logs. For example, <code>arn:aws:iam:123456789012:role/emaccess</code>.</p>
    /// <p>If <code>MonitoringInterval</code> is set to a value other than 0, then you must supply a <code>MonitoringRoleArn</code> value.</p>
    pub fn monitoring_role_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.monitoring_role_arn(input.into());
        self
    }
    /// <p>The ARN for the IAM role that permits Neptune to send enhanced monitoring metrics to Amazon CloudWatch Logs. For example, <code>arn:aws:iam:123456789012:role/emaccess</code>.</p>
    /// <p>If <code>MonitoringInterval</code> is set to a value other than 0, then you must supply a <code>MonitoringRoleArn</code> value.</p>
    pub fn set_monitoring_role_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_monitoring_role_arn(input);
        self
    }
    /// <p>Specify the name of the IAM role to be used when making API calls to the Directory Service.</p>
    pub fn domain_iam_role_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.domain_iam_role_name(input.into());
        self
    }
    /// <p>Specify the name of the IAM role to be used when making API calls to the Directory Service.</p>
    pub fn set_domain_iam_role_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_domain_iam_role_name(input);
        self
    }
    /// <p>A value that specifies the order in which an Read Replica is promoted to the primary instance after a failure of the existing primary instance. </p>
    /// <p>Default: 1</p>
    /// <p>Valid Values: 0 - 15</p>
    pub fn promotion_tier(mut self, input: i32) -> Self {
        self.inner = self.inner.promotion_tier(input);
        self
    }
    /// <p>A value that specifies the order in which an Read Replica is promoted to the primary instance after a failure of the existing primary instance. </p>
    /// <p>Default: 1</p>
    /// <p>Valid Values: 0 - 15</p>
    pub fn set_promotion_tier(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_promotion_tier(input);
        self
    }
    /// <p>The time zone of the DB instance.</p>
    pub fn timezone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.timezone(input.into());
        self
    }
    /// <p>The time zone of the DB instance.</p>
    pub fn set_timezone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_timezone(input);
        self
    }
    /// <p>Not supported by Neptune (ignored).</p>
    pub fn enable_iam_database_authentication(mut self, input: bool) -> Self {
        self.inner = self.inner.enable_iam_database_authentication(input);
        self
    }
    /// <p>Not supported by Neptune (ignored).</p>
    pub fn set_enable_iam_database_authentication(
        mut self,
        input: ::std::option::Option<bool>,
    ) -> Self {
        self.inner = self.inner.set_enable_iam_database_authentication(input);
        self
    }
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub fn enable_performance_insights(mut self, input: bool) -> Self {
        self.inner = self.inner.enable_performance_insights(input);
        self
    }
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub fn set_enable_performance_insights(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enable_performance_insights(input);
        self
    }
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub fn performance_insights_kms_key_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.performance_insights_kms_key_id(input.into());
        self
    }
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub fn set_performance_insights_kms_key_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_performance_insights_kms_key_id(input);
        self
    }
    /// Appends an item to `EnableCloudwatchLogsExports`.
    ///
    /// To override the contents of this collection use [`set_enable_cloudwatch_logs_exports`](Self::set_enable_cloudwatch_logs_exports).
    ///
    /// <p>The list of log types that need to be enabled for exporting to CloudWatch Logs.</p>
    pub fn enable_cloudwatch_logs_exports(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.enable_cloudwatch_logs_exports(input.into());
        self
    }
    /// <p>The list of log types that need to be enabled for exporting to CloudWatch Logs.</p>
    pub fn set_enable_cloudwatch_logs_exports(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_enable_cloudwatch_logs_exports(input);
        self
    }
    /// <p>A value that indicates whether the DB instance has deletion protection enabled. The database can't be deleted when deletion protection is enabled. By default, deletion protection is disabled. See <a href="https://docs.aws.amazon.com/neptune/latest/userguide/manage-console-instances-delete.html">Deleting a DB Instance</a>.</p>
    /// <p>DB instances in a DB cluster can be deleted even when deletion protection is enabled in their parent DB cluster.</p>
    pub fn deletion_protection(mut self, input: bool) -> Self {
        self.inner = self.inner.deletion_protection(input);
        self
    }
    /// <p>A value that indicates whether the DB instance has deletion protection enabled. The database can't be deleted when deletion protection is enabled. By default, deletion protection is disabled. See <a href="https://docs.aws.amazon.com/neptune/latest/userguide/manage-console-instances-delete.html">Deleting a DB Instance</a>.</p>
    /// <p>DB instances in a DB cluster can be deleted even when deletion protection is enabled in their parent DB cluster.</p>
    pub fn set_deletion_protection(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_deletion_protection(input);
        self
    }
}
