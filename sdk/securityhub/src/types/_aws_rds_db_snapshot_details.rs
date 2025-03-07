// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides details about an Amazon RDS DB cluster snapshot.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsRdsDbSnapshotDetails {
    /// <p>The name or ARN of the DB snapshot that is used to restore the DB instance.</p>
    #[doc(hidden)]
    pub db_snapshot_identifier: ::std::option::Option<::std::string::String>,
    /// <p>A name for the DB instance.</p>
    #[doc(hidden)]
    pub db_instance_identifier: ::std::option::Option<::std::string::String>,
    /// <p>When the snapshot was taken in Coordinated Universal Time (UTC).</p>
    #[doc(hidden)]
    pub snapshot_create_time: ::std::option::Option<::std::string::String>,
    /// <p>The name of the database engine to use for this DB instance. Valid values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>aurora</code> </p> </li>
    /// <li> <p> <code>aurora-mysql</code> </p> </li>
    /// <li> <p> <code>aurora-postgresql</code> </p> </li>
    /// <li> <p> <code>c</code> </p> </li>
    /// <li> <p> <code>mariadb</code> </p> </li>
    /// <li> <p> <code>mysql</code> </p> </li>
    /// <li> <p> <code>oracle-ee</code> </p> </li>
    /// <li> <p> <code>oracle-se</code> </p> </li>
    /// <li> <p> <code>oracle-se1</code> </p> </li>
    /// <li> <p> <code>oracle-se2</code> </p> </li>
    /// <li> <p> <code>sqlserver-ee</code> </p> </li>
    /// <li> <p> <code>sqlserver-ex</code> </p> </li>
    /// <li> <p> <code>sqlserver-se</code> </p> </li>
    /// <li> <p> <code>sqlserver-web</code> </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub engine: ::std::option::Option<::std::string::String>,
    /// <p>The amount of storage (in gigabytes) to be initially allocated for the database instance.</p>
    #[doc(hidden)]
    pub allocated_storage: i32,
    /// <p>The status of this DB snapshot.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<::std::string::String>,
    /// <p>The port that the database engine was listening on at the time of the snapshot.</p>
    #[doc(hidden)]
    pub port: i32,
    /// <p>Specifies the name of the Availability Zone in which the DB instance was located at the time of the DB snapshot.</p>
    #[doc(hidden)]
    pub availability_zone: ::std::option::Option<::std::string::String>,
    /// <p>The VPC ID associated with the DB snapshot.</p>
    #[doc(hidden)]
    pub vpc_id: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the time in Coordinated Universal Time (UTC) when the DB instance, from which the snapshot was taken, was created.</p>
    #[doc(hidden)]
    pub instance_create_time: ::std::option::Option<::std::string::String>,
    /// <p>The master user name for the DB snapshot.</p>
    #[doc(hidden)]
    pub master_username: ::std::option::Option<::std::string::String>,
    /// <p>The version of the database engine.</p>
    #[doc(hidden)]
    pub engine_version: ::std::option::Option<::std::string::String>,
    /// <p>License model information for the restored DB instance.</p>
    #[doc(hidden)]
    pub license_model: ::std::option::Option<::std::string::String>,
    /// <p>The type of the DB snapshot.</p>
    #[doc(hidden)]
    pub snapshot_type: ::std::option::Option<::std::string::String>,
    /// <p>The provisioned IOPS (I/O operations per second) value of the DB instance at the time of the snapshot.</p>
    #[doc(hidden)]
    pub iops: i32,
    /// <p>The option group name for the DB snapshot.</p>
    #[doc(hidden)]
    pub option_group_name: ::std::option::Option<::std::string::String>,
    /// <p>The percentage of the estimated data that has been transferred.</p>
    #[doc(hidden)]
    pub percent_progress: i32,
    /// <p>The Amazon Web Services Region that the DB snapshot was created in or copied from.</p>
    #[doc(hidden)]
    pub source_region: ::std::option::Option<::std::string::String>,
    /// <p>The DB snapshot ARN that the DB snapshot was copied from.</p>
    #[doc(hidden)]
    pub source_db_snapshot_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The storage type associated with the DB snapshot. Valid values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>gp2</code> </p> </li>
    /// <li> <p> <code>io1</code> </p> </li>
    /// <li> <p> <code>standard</code> </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub storage_type: ::std::option::Option<::std::string::String>,
    /// <p>The ARN from the key store with which to associate the instance for TDE encryption.</p>
    #[doc(hidden)]
    pub tde_credential_arn: ::std::option::Option<::std::string::String>,
    /// <p>Whether the DB snapshot is encrypted.</p>
    #[doc(hidden)]
    pub encrypted: bool,
    /// <p>If <code>Encrypted</code> is <code>true</code>, the KMS key identifier for the encrypted DB snapshot.</p>
    #[doc(hidden)]
    pub kms_key_id: ::std::option::Option<::std::string::String>,
    /// <p>The time zone of the DB snapshot.</p>
    #[doc(hidden)]
    pub timezone: ::std::option::Option<::std::string::String>,
    /// <p>Whether mapping of IAM accounts to database accounts is enabled.</p>
    #[doc(hidden)]
    pub iam_database_authentication_enabled: bool,
    /// <p>The number of CPU cores and the number of threads per core for the DB instance class of the DB instance.</p>
    #[doc(hidden)]
    pub processor_features:
        ::std::option::Option<::std::vec::Vec<crate::types::AwsRdsDbProcessorFeature>>,
    /// <p>The identifier for the source DB instance.</p>
    #[doc(hidden)]
    pub dbi_resource_id: ::std::option::Option<::std::string::String>,
}
impl AwsRdsDbSnapshotDetails {
    /// <p>The name or ARN of the DB snapshot that is used to restore the DB instance.</p>
    pub fn db_snapshot_identifier(&self) -> ::std::option::Option<&str> {
        self.db_snapshot_identifier.as_deref()
    }
    /// <p>A name for the DB instance.</p>
    pub fn db_instance_identifier(&self) -> ::std::option::Option<&str> {
        self.db_instance_identifier.as_deref()
    }
    /// <p>When the snapshot was taken in Coordinated Universal Time (UTC).</p>
    pub fn snapshot_create_time(&self) -> ::std::option::Option<&str> {
        self.snapshot_create_time.as_deref()
    }
    /// <p>The name of the database engine to use for this DB instance. Valid values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>aurora</code> </p> </li>
    /// <li> <p> <code>aurora-mysql</code> </p> </li>
    /// <li> <p> <code>aurora-postgresql</code> </p> </li>
    /// <li> <p> <code>c</code> </p> </li>
    /// <li> <p> <code>mariadb</code> </p> </li>
    /// <li> <p> <code>mysql</code> </p> </li>
    /// <li> <p> <code>oracle-ee</code> </p> </li>
    /// <li> <p> <code>oracle-se</code> </p> </li>
    /// <li> <p> <code>oracle-se1</code> </p> </li>
    /// <li> <p> <code>oracle-se2</code> </p> </li>
    /// <li> <p> <code>sqlserver-ee</code> </p> </li>
    /// <li> <p> <code>sqlserver-ex</code> </p> </li>
    /// <li> <p> <code>sqlserver-se</code> </p> </li>
    /// <li> <p> <code>sqlserver-web</code> </p> </li>
    /// </ul>
    pub fn engine(&self) -> ::std::option::Option<&str> {
        self.engine.as_deref()
    }
    /// <p>The amount of storage (in gigabytes) to be initially allocated for the database instance.</p>
    pub fn allocated_storage(&self) -> i32 {
        self.allocated_storage
    }
    /// <p>The status of this DB snapshot.</p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>The port that the database engine was listening on at the time of the snapshot.</p>
    pub fn port(&self) -> i32 {
        self.port
    }
    /// <p>Specifies the name of the Availability Zone in which the DB instance was located at the time of the DB snapshot.</p>
    pub fn availability_zone(&self) -> ::std::option::Option<&str> {
        self.availability_zone.as_deref()
    }
    /// <p>The VPC ID associated with the DB snapshot.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>Specifies the time in Coordinated Universal Time (UTC) when the DB instance, from which the snapshot was taken, was created.</p>
    pub fn instance_create_time(&self) -> ::std::option::Option<&str> {
        self.instance_create_time.as_deref()
    }
    /// <p>The master user name for the DB snapshot.</p>
    pub fn master_username(&self) -> ::std::option::Option<&str> {
        self.master_username.as_deref()
    }
    /// <p>The version of the database engine.</p>
    pub fn engine_version(&self) -> ::std::option::Option<&str> {
        self.engine_version.as_deref()
    }
    /// <p>License model information for the restored DB instance.</p>
    pub fn license_model(&self) -> ::std::option::Option<&str> {
        self.license_model.as_deref()
    }
    /// <p>The type of the DB snapshot.</p>
    pub fn snapshot_type(&self) -> ::std::option::Option<&str> {
        self.snapshot_type.as_deref()
    }
    /// <p>The provisioned IOPS (I/O operations per second) value of the DB instance at the time of the snapshot.</p>
    pub fn iops(&self) -> i32 {
        self.iops
    }
    /// <p>The option group name for the DB snapshot.</p>
    pub fn option_group_name(&self) -> ::std::option::Option<&str> {
        self.option_group_name.as_deref()
    }
    /// <p>The percentage of the estimated data that has been transferred.</p>
    pub fn percent_progress(&self) -> i32 {
        self.percent_progress
    }
    /// <p>The Amazon Web Services Region that the DB snapshot was created in or copied from.</p>
    pub fn source_region(&self) -> ::std::option::Option<&str> {
        self.source_region.as_deref()
    }
    /// <p>The DB snapshot ARN that the DB snapshot was copied from.</p>
    pub fn source_db_snapshot_identifier(&self) -> ::std::option::Option<&str> {
        self.source_db_snapshot_identifier.as_deref()
    }
    /// <p>The storage type associated with the DB snapshot. Valid values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>gp2</code> </p> </li>
    /// <li> <p> <code>io1</code> </p> </li>
    /// <li> <p> <code>standard</code> </p> </li>
    /// </ul>
    pub fn storage_type(&self) -> ::std::option::Option<&str> {
        self.storage_type.as_deref()
    }
    /// <p>The ARN from the key store with which to associate the instance for TDE encryption.</p>
    pub fn tde_credential_arn(&self) -> ::std::option::Option<&str> {
        self.tde_credential_arn.as_deref()
    }
    /// <p>Whether the DB snapshot is encrypted.</p>
    pub fn encrypted(&self) -> bool {
        self.encrypted
    }
    /// <p>If <code>Encrypted</code> is <code>true</code>, the KMS key identifier for the encrypted DB snapshot.</p>
    pub fn kms_key_id(&self) -> ::std::option::Option<&str> {
        self.kms_key_id.as_deref()
    }
    /// <p>The time zone of the DB snapshot.</p>
    pub fn timezone(&self) -> ::std::option::Option<&str> {
        self.timezone.as_deref()
    }
    /// <p>Whether mapping of IAM accounts to database accounts is enabled.</p>
    pub fn iam_database_authentication_enabled(&self) -> bool {
        self.iam_database_authentication_enabled
    }
    /// <p>The number of CPU cores and the number of threads per core for the DB instance class of the DB instance.</p>
    pub fn processor_features(
        &self,
    ) -> ::std::option::Option<&[crate::types::AwsRdsDbProcessorFeature]> {
        self.processor_features.as_deref()
    }
    /// <p>The identifier for the source DB instance.</p>
    pub fn dbi_resource_id(&self) -> ::std::option::Option<&str> {
        self.dbi_resource_id.as_deref()
    }
}
impl AwsRdsDbSnapshotDetails {
    /// Creates a new builder-style object to manufacture [`AwsRdsDbSnapshotDetails`](crate::types::AwsRdsDbSnapshotDetails).
    pub fn builder() -> crate::types::builders::AwsRdsDbSnapshotDetailsBuilder {
        crate::types::builders::AwsRdsDbSnapshotDetailsBuilder::default()
    }
}

/// A builder for [`AwsRdsDbSnapshotDetails`](crate::types::AwsRdsDbSnapshotDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsRdsDbSnapshotDetailsBuilder {
    pub(crate) db_snapshot_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) db_instance_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) snapshot_create_time: ::std::option::Option<::std::string::String>,
    pub(crate) engine: ::std::option::Option<::std::string::String>,
    pub(crate) allocated_storage: ::std::option::Option<i32>,
    pub(crate) status: ::std::option::Option<::std::string::String>,
    pub(crate) port: ::std::option::Option<i32>,
    pub(crate) availability_zone: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
    pub(crate) instance_create_time: ::std::option::Option<::std::string::String>,
    pub(crate) master_username: ::std::option::Option<::std::string::String>,
    pub(crate) engine_version: ::std::option::Option<::std::string::String>,
    pub(crate) license_model: ::std::option::Option<::std::string::String>,
    pub(crate) snapshot_type: ::std::option::Option<::std::string::String>,
    pub(crate) iops: ::std::option::Option<i32>,
    pub(crate) option_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) percent_progress: ::std::option::Option<i32>,
    pub(crate) source_region: ::std::option::Option<::std::string::String>,
    pub(crate) source_db_snapshot_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) storage_type: ::std::option::Option<::std::string::String>,
    pub(crate) tde_credential_arn: ::std::option::Option<::std::string::String>,
    pub(crate) encrypted: ::std::option::Option<bool>,
    pub(crate) kms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) timezone: ::std::option::Option<::std::string::String>,
    pub(crate) iam_database_authentication_enabled: ::std::option::Option<bool>,
    pub(crate) processor_features:
        ::std::option::Option<::std::vec::Vec<crate::types::AwsRdsDbProcessorFeature>>,
    pub(crate) dbi_resource_id: ::std::option::Option<::std::string::String>,
}
impl AwsRdsDbSnapshotDetailsBuilder {
    /// <p>The name or ARN of the DB snapshot that is used to restore the DB instance.</p>
    pub fn db_snapshot_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.db_snapshot_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name or ARN of the DB snapshot that is used to restore the DB instance.</p>
    pub fn set_db_snapshot_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.db_snapshot_identifier = input;
        self
    }
    /// <p>A name for the DB instance.</p>
    pub fn db_instance_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.db_instance_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A name for the DB instance.</p>
    pub fn set_db_instance_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.db_instance_identifier = input;
        self
    }
    /// <p>When the snapshot was taken in Coordinated Universal Time (UTC).</p>
    pub fn snapshot_create_time(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.snapshot_create_time = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>When the snapshot was taken in Coordinated Universal Time (UTC).</p>
    pub fn set_snapshot_create_time(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.snapshot_create_time = input;
        self
    }
    /// <p>The name of the database engine to use for this DB instance. Valid values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>aurora</code> </p> </li>
    /// <li> <p> <code>aurora-mysql</code> </p> </li>
    /// <li> <p> <code>aurora-postgresql</code> </p> </li>
    /// <li> <p> <code>c</code> </p> </li>
    /// <li> <p> <code>mariadb</code> </p> </li>
    /// <li> <p> <code>mysql</code> </p> </li>
    /// <li> <p> <code>oracle-ee</code> </p> </li>
    /// <li> <p> <code>oracle-se</code> </p> </li>
    /// <li> <p> <code>oracle-se1</code> </p> </li>
    /// <li> <p> <code>oracle-se2</code> </p> </li>
    /// <li> <p> <code>sqlserver-ee</code> </p> </li>
    /// <li> <p> <code>sqlserver-ex</code> </p> </li>
    /// <li> <p> <code>sqlserver-se</code> </p> </li>
    /// <li> <p> <code>sqlserver-web</code> </p> </li>
    /// </ul>
    pub fn engine(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.engine = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the database engine to use for this DB instance. Valid values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>aurora</code> </p> </li>
    /// <li> <p> <code>aurora-mysql</code> </p> </li>
    /// <li> <p> <code>aurora-postgresql</code> </p> </li>
    /// <li> <p> <code>c</code> </p> </li>
    /// <li> <p> <code>mariadb</code> </p> </li>
    /// <li> <p> <code>mysql</code> </p> </li>
    /// <li> <p> <code>oracle-ee</code> </p> </li>
    /// <li> <p> <code>oracle-se</code> </p> </li>
    /// <li> <p> <code>oracle-se1</code> </p> </li>
    /// <li> <p> <code>oracle-se2</code> </p> </li>
    /// <li> <p> <code>sqlserver-ee</code> </p> </li>
    /// <li> <p> <code>sqlserver-ex</code> </p> </li>
    /// <li> <p> <code>sqlserver-se</code> </p> </li>
    /// <li> <p> <code>sqlserver-web</code> </p> </li>
    /// </ul>
    pub fn set_engine(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.engine = input;
        self
    }
    /// <p>The amount of storage (in gigabytes) to be initially allocated for the database instance.</p>
    pub fn allocated_storage(mut self, input: i32) -> Self {
        self.allocated_storage = ::std::option::Option::Some(input);
        self
    }
    /// <p>The amount of storage (in gigabytes) to be initially allocated for the database instance.</p>
    pub fn set_allocated_storage(mut self, input: ::std::option::Option<i32>) -> Self {
        self.allocated_storage = input;
        self
    }
    /// <p>The status of this DB snapshot.</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The status of this DB snapshot.</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// <p>The port that the database engine was listening on at the time of the snapshot.</p>
    pub fn port(mut self, input: i32) -> Self {
        self.port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The port that the database engine was listening on at the time of the snapshot.</p>
    pub fn set_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.port = input;
        self
    }
    /// <p>Specifies the name of the Availability Zone in which the DB instance was located at the time of the DB snapshot.</p>
    pub fn availability_zone(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.availability_zone = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the name of the Availability Zone in which the DB instance was located at the time of the DB snapshot.</p>
    pub fn set_availability_zone(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.availability_zone = input;
        self
    }
    /// <p>The VPC ID associated with the DB snapshot.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The VPC ID associated with the DB snapshot.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// <p>Specifies the time in Coordinated Universal Time (UTC) when the DB instance, from which the snapshot was taken, was created.</p>
    pub fn instance_create_time(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.instance_create_time = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the time in Coordinated Universal Time (UTC) when the DB instance, from which the snapshot was taken, was created.</p>
    pub fn set_instance_create_time(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.instance_create_time = input;
        self
    }
    /// <p>The master user name for the DB snapshot.</p>
    pub fn master_username(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.master_username = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The master user name for the DB snapshot.</p>
    pub fn set_master_username(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.master_username = input;
        self
    }
    /// <p>The version of the database engine.</p>
    pub fn engine_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.engine_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the database engine.</p>
    pub fn set_engine_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.engine_version = input;
        self
    }
    /// <p>License model information for the restored DB instance.</p>
    pub fn license_model(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.license_model = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>License model information for the restored DB instance.</p>
    pub fn set_license_model(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.license_model = input;
        self
    }
    /// <p>The type of the DB snapshot.</p>
    pub fn snapshot_type(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.snapshot_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of the DB snapshot.</p>
    pub fn set_snapshot_type(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.snapshot_type = input;
        self
    }
    /// <p>The provisioned IOPS (I/O operations per second) value of the DB instance at the time of the snapshot.</p>
    pub fn iops(mut self, input: i32) -> Self {
        self.iops = ::std::option::Option::Some(input);
        self
    }
    /// <p>The provisioned IOPS (I/O operations per second) value of the DB instance at the time of the snapshot.</p>
    pub fn set_iops(mut self, input: ::std::option::Option<i32>) -> Self {
        self.iops = input;
        self
    }
    /// <p>The option group name for the DB snapshot.</p>
    pub fn option_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.option_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The option group name for the DB snapshot.</p>
    pub fn set_option_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.option_group_name = input;
        self
    }
    /// <p>The percentage of the estimated data that has been transferred.</p>
    pub fn percent_progress(mut self, input: i32) -> Self {
        self.percent_progress = ::std::option::Option::Some(input);
        self
    }
    /// <p>The percentage of the estimated data that has been transferred.</p>
    pub fn set_percent_progress(mut self, input: ::std::option::Option<i32>) -> Self {
        self.percent_progress = input;
        self
    }
    /// <p>The Amazon Web Services Region that the DB snapshot was created in or copied from.</p>
    pub fn source_region(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.source_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services Region that the DB snapshot was created in or copied from.</p>
    pub fn set_source_region(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.source_region = input;
        self
    }
    /// <p>The DB snapshot ARN that the DB snapshot was copied from.</p>
    pub fn source_db_snapshot_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.source_db_snapshot_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The DB snapshot ARN that the DB snapshot was copied from.</p>
    pub fn set_source_db_snapshot_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.source_db_snapshot_identifier = input;
        self
    }
    /// <p>The storage type associated with the DB snapshot. Valid values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>gp2</code> </p> </li>
    /// <li> <p> <code>io1</code> </p> </li>
    /// <li> <p> <code>standard</code> </p> </li>
    /// </ul>
    pub fn storage_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.storage_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The storage type associated with the DB snapshot. Valid values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>gp2</code> </p> </li>
    /// <li> <p> <code>io1</code> </p> </li>
    /// <li> <p> <code>standard</code> </p> </li>
    /// </ul>
    pub fn set_storage_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.storage_type = input;
        self
    }
    /// <p>The ARN from the key store with which to associate the instance for TDE encryption.</p>
    pub fn tde_credential_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.tde_credential_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN from the key store with which to associate the instance for TDE encryption.</p>
    pub fn set_tde_credential_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.tde_credential_arn = input;
        self
    }
    /// <p>Whether the DB snapshot is encrypted.</p>
    pub fn encrypted(mut self, input: bool) -> Self {
        self.encrypted = ::std::option::Option::Some(input);
        self
    }
    /// <p>Whether the DB snapshot is encrypted.</p>
    pub fn set_encrypted(mut self, input: ::std::option::Option<bool>) -> Self {
        self.encrypted = input;
        self
    }
    /// <p>If <code>Encrypted</code> is <code>true</code>, the KMS key identifier for the encrypted DB snapshot.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If <code>Encrypted</code> is <code>true</code>, the KMS key identifier for the encrypted DB snapshot.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_id = input;
        self
    }
    /// <p>The time zone of the DB snapshot.</p>
    pub fn timezone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.timezone = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The time zone of the DB snapshot.</p>
    pub fn set_timezone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.timezone = input;
        self
    }
    /// <p>Whether mapping of IAM accounts to database accounts is enabled.</p>
    pub fn iam_database_authentication_enabled(mut self, input: bool) -> Self {
        self.iam_database_authentication_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Whether mapping of IAM accounts to database accounts is enabled.</p>
    pub fn set_iam_database_authentication_enabled(
        mut self,
        input: ::std::option::Option<bool>,
    ) -> Self {
        self.iam_database_authentication_enabled = input;
        self
    }
    /// Appends an item to `processor_features`.
    ///
    /// To override the contents of this collection use [`set_processor_features`](Self::set_processor_features).
    ///
    /// <p>The number of CPU cores and the number of threads per core for the DB instance class of the DB instance.</p>
    pub fn processor_features(mut self, input: crate::types::AwsRdsDbProcessorFeature) -> Self {
        let mut v = self.processor_features.unwrap_or_default();
        v.push(input);
        self.processor_features = ::std::option::Option::Some(v);
        self
    }
    /// <p>The number of CPU cores and the number of threads per core for the DB instance class of the DB instance.</p>
    pub fn set_processor_features(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AwsRdsDbProcessorFeature>>,
    ) -> Self {
        self.processor_features = input;
        self
    }
    /// <p>The identifier for the source DB instance.</p>
    pub fn dbi_resource_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.dbi_resource_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the source DB instance.</p>
    pub fn set_dbi_resource_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.dbi_resource_id = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsRdsDbSnapshotDetails`](crate::types::AwsRdsDbSnapshotDetails).
    pub fn build(self) -> crate::types::AwsRdsDbSnapshotDetails {
        crate::types::AwsRdsDbSnapshotDetails {
            db_snapshot_identifier: self.db_snapshot_identifier,
            db_instance_identifier: self.db_instance_identifier,
            snapshot_create_time: self.snapshot_create_time,
            engine: self.engine,
            allocated_storage: self.allocated_storage.unwrap_or_default(),
            status: self.status,
            port: self.port.unwrap_or_default(),
            availability_zone: self.availability_zone,
            vpc_id: self.vpc_id,
            instance_create_time: self.instance_create_time,
            master_username: self.master_username,
            engine_version: self.engine_version,
            license_model: self.license_model,
            snapshot_type: self.snapshot_type,
            iops: self.iops.unwrap_or_default(),
            option_group_name: self.option_group_name,
            percent_progress: self.percent_progress.unwrap_or_default(),
            source_region: self.source_region,
            source_db_snapshot_identifier: self.source_db_snapshot_identifier,
            storage_type: self.storage_type,
            tde_credential_arn: self.tde_credential_arn,
            encrypted: self.encrypted.unwrap_or_default(),
            kms_key_id: self.kms_key_id,
            timezone: self.timezone,
            iam_database_authentication_enabled: self
                .iam_database_authentication_enabled
                .unwrap_or_default(),
            processor_features: self.processor_features,
            dbi_resource_id: self.dbi_resource_id,
        }
    }
}
