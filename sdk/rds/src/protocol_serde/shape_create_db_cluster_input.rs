// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_db_cluster_input_input(
    input: &crate::operation::create_db_cluster::CreateDbClusterInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        ::aws_smithy_query::QueryWriter::new(&mut out, "CreateDBCluster", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AvailabilityZones");
    if let Some(var_2) = &input.availability_zones {
        let mut list_4 = scope_1.start_list(false, Some("AvailabilityZone"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("BackupRetentionPeriod");
    if let Some(var_7) = &input.backup_retention_period {
        scope_6.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("CharacterSetName");
    if let Some(var_9) = &input.character_set_name {
        scope_8.string(var_9);
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("DatabaseName");
    if let Some(var_11) = &input.database_name {
        scope_10.string(var_11);
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("DBClusterIdentifier");
    if let Some(var_13) = &input.db_cluster_identifier {
        scope_12.string(var_13);
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("DBClusterParameterGroupName");
    if let Some(var_15) = &input.db_cluster_parameter_group_name {
        scope_14.string(var_15);
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("VpcSecurityGroupIds");
    if let Some(var_17) = &input.vpc_security_group_ids {
        let mut list_19 = scope_16.start_list(false, Some("VpcSecurityGroupId"));
        for item_18 in var_17 {
            #[allow(unused_mut)]
            let mut entry_20 = list_19.entry();
            entry_20.string(item_18);
        }
        list_19.finish();
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("DBSubnetGroupName");
    if let Some(var_22) = &input.db_subnet_group_name {
        scope_21.string(var_22);
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("Engine");
    if let Some(var_24) = &input.engine {
        scope_23.string(var_24);
    }
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("EngineVersion");
    if let Some(var_26) = &input.engine_version {
        scope_25.string(var_26);
    }
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("Port");
    if let Some(var_28) = &input.port {
        scope_27.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_28).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_29 = writer.prefix("MasterUsername");
    if let Some(var_30) = &input.master_username {
        scope_29.string(var_30);
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("MasterUserPassword");
    if let Some(var_32) = &input.master_user_password {
        scope_31.string(var_32);
    }
    #[allow(unused_mut)]
    let mut scope_33 = writer.prefix("OptionGroupName");
    if let Some(var_34) = &input.option_group_name {
        scope_33.string(var_34);
    }
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("PreferredBackupWindow");
    if let Some(var_36) = &input.preferred_backup_window {
        scope_35.string(var_36);
    }
    #[allow(unused_mut)]
    let mut scope_37 = writer.prefix("PreferredMaintenanceWindow");
    if let Some(var_38) = &input.preferred_maintenance_window {
        scope_37.string(var_38);
    }
    #[allow(unused_mut)]
    let mut scope_39 = writer.prefix("ReplicationSourceIdentifier");
    if let Some(var_40) = &input.replication_source_identifier {
        scope_39.string(var_40);
    }
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("Tags");
    if let Some(var_42) = &input.tags {
        let mut list_44 = scope_41.start_list(false, Some("Tag"));
        for item_43 in var_42 {
            #[allow(unused_mut)]
            let mut entry_45 = list_44.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_45, item_43)?;
        }
        list_44.finish();
    }
    #[allow(unused_mut)]
    let mut scope_46 = writer.prefix("StorageEncrypted");
    if let Some(var_47) = &input.storage_encrypted {
        scope_46.boolean(*var_47);
    }
    #[allow(unused_mut)]
    let mut scope_48 = writer.prefix("KmsKeyId");
    if let Some(var_49) = &input.kms_key_id {
        scope_48.string(var_49);
    }
    #[allow(unused_mut)]
    let mut scope_50 = writer.prefix("PreSignedUrl");
    if let Some(var_51) = &input.pre_signed_url {
        scope_50.string(var_51);
    }
    #[allow(unused_mut)]
    let mut scope_52 = writer.prefix("EnableIAMDatabaseAuthentication");
    if let Some(var_53) = &input.enable_iam_database_authentication {
        scope_52.boolean(*var_53);
    }
    #[allow(unused_mut)]
    let mut scope_54 = writer.prefix("BacktrackWindow");
    if let Some(var_55) = &input.backtrack_window {
        scope_54.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_55).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_56 = writer.prefix("EnableCloudwatchLogsExports");
    if let Some(var_57) = &input.enable_cloudwatch_logs_exports {
        let mut list_59 = scope_56.start_list(false, None);
        for item_58 in var_57 {
            #[allow(unused_mut)]
            let mut entry_60 = list_59.entry();
            entry_60.string(item_58);
        }
        list_59.finish();
    }
    #[allow(unused_mut)]
    let mut scope_61 = writer.prefix("EngineMode");
    if let Some(var_62) = &input.engine_mode {
        scope_61.string(var_62);
    }
    #[allow(unused_mut)]
    let mut scope_63 = writer.prefix("ScalingConfiguration");
    if let Some(var_64) = &input.scaling_configuration {
        crate::protocol_serde::shape_scaling_configuration::ser_scaling_configuration(
            scope_63, var_64,
        )?;
    }
    #[allow(unused_mut)]
    let mut scope_65 = writer.prefix("DeletionProtection");
    if let Some(var_66) = &input.deletion_protection {
        scope_65.boolean(*var_66);
    }
    #[allow(unused_mut)]
    let mut scope_67 = writer.prefix("GlobalClusterIdentifier");
    if let Some(var_68) = &input.global_cluster_identifier {
        scope_67.string(var_68);
    }
    #[allow(unused_mut)]
    let mut scope_69 = writer.prefix("EnableHttpEndpoint");
    if let Some(var_70) = &input.enable_http_endpoint {
        scope_69.boolean(*var_70);
    }
    #[allow(unused_mut)]
    let mut scope_71 = writer.prefix("CopyTagsToSnapshot");
    if let Some(var_72) = &input.copy_tags_to_snapshot {
        scope_71.boolean(*var_72);
    }
    #[allow(unused_mut)]
    let mut scope_73 = writer.prefix("Domain");
    if let Some(var_74) = &input.domain {
        scope_73.string(var_74);
    }
    #[allow(unused_mut)]
    let mut scope_75 = writer.prefix("DomainIAMRoleName");
    if let Some(var_76) = &input.domain_iam_role_name {
        scope_75.string(var_76);
    }
    #[allow(unused_mut)]
    let mut scope_77 = writer.prefix("EnableGlobalWriteForwarding");
    if let Some(var_78) = &input.enable_global_write_forwarding {
        scope_77.boolean(*var_78);
    }
    #[allow(unused_mut)]
    let mut scope_79 = writer.prefix("DBClusterInstanceClass");
    if let Some(var_80) = &input.db_cluster_instance_class {
        scope_79.string(var_80);
    }
    #[allow(unused_mut)]
    let mut scope_81 = writer.prefix("AllocatedStorage");
    if let Some(var_82) = &input.allocated_storage {
        scope_81.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_82).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_83 = writer.prefix("StorageType");
    if let Some(var_84) = &input.storage_type {
        scope_83.string(var_84);
    }
    #[allow(unused_mut)]
    let mut scope_85 = writer.prefix("Iops");
    if let Some(var_86) = &input.iops {
        scope_85.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_86).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_87 = writer.prefix("PubliclyAccessible");
    if let Some(var_88) = &input.publicly_accessible {
        scope_87.boolean(*var_88);
    }
    #[allow(unused_mut)]
    let mut scope_89 = writer.prefix("AutoMinorVersionUpgrade");
    if let Some(var_90) = &input.auto_minor_version_upgrade {
        scope_89.boolean(*var_90);
    }
    #[allow(unused_mut)]
    let mut scope_91 = writer.prefix("MonitoringInterval");
    if let Some(var_92) = &input.monitoring_interval {
        scope_91.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_92).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_93 = writer.prefix("MonitoringRoleArn");
    if let Some(var_94) = &input.monitoring_role_arn {
        scope_93.string(var_94);
    }
    #[allow(unused_mut)]
    let mut scope_95 = writer.prefix("EnablePerformanceInsights");
    if let Some(var_96) = &input.enable_performance_insights {
        scope_95.boolean(*var_96);
    }
    #[allow(unused_mut)]
    let mut scope_97 = writer.prefix("PerformanceInsightsKMSKeyId");
    if let Some(var_98) = &input.performance_insights_kms_key_id {
        scope_97.string(var_98);
    }
    #[allow(unused_mut)]
    let mut scope_99 = writer.prefix("PerformanceInsightsRetentionPeriod");
    if let Some(var_100) = &input.performance_insights_retention_period {
        scope_99.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_100).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_101 = writer.prefix("ServerlessV2ScalingConfiguration");
    if let Some(var_102) = &input.serverless_v2_scaling_configuration {
        crate::protocol_serde::shape_serverless_v2_scaling_configuration::ser_serverless_v2_scaling_configuration(scope_101, var_102)?;
    }
    #[allow(unused_mut)]
    let mut scope_103 = writer.prefix("NetworkType");
    if let Some(var_104) = &input.network_type {
        scope_103.string(var_104);
    }
    #[allow(unused_mut)]
    let mut scope_105 = writer.prefix("DBSystemId");
    if let Some(var_106) = &input.db_system_id {
        scope_105.string(var_106);
    }
    #[allow(unused_mut)]
    let mut scope_107 = writer.prefix("ManageMasterUserPassword");
    if let Some(var_108) = &input.manage_master_user_password {
        scope_107.boolean(*var_108);
    }
    #[allow(unused_mut)]
    let mut scope_109 = writer.prefix("MasterUserSecretKmsKeyId");
    if let Some(var_110) = &input.master_user_secret_kms_key_id {
        scope_109.string(var_110);
    }
    writer.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}
