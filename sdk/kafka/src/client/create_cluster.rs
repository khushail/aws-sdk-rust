// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateCluster`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`broker_node_group_info(BrokerNodeGroupInfo)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::broker_node_group_info) / [`set_broker_node_group_info(Option<BrokerNodeGroupInfo>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::set_broker_node_group_info): <p>Information about the broker nodes in the cluster.</p>
    ///   - [`client_authentication(ClientAuthentication)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::client_authentication) / [`set_client_authentication(Option<ClientAuthentication>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::set_client_authentication): <p>Includes all client authentication related information.</p>
    ///   - [`cluster_name(impl ::std::convert::Into<String>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::cluster_name) / [`set_cluster_name(Option<String>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::set_cluster_name): <p>The name of the cluster.</p>
    ///   - [`configuration_info(ConfigurationInfo)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::configuration_info) / [`set_configuration_info(Option<ConfigurationInfo>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::set_configuration_info): <p>Represents the configuration that you want MSK to use for the brokers in a cluster.</p>
    ///   - [`encryption_info(EncryptionInfo)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::encryption_info) / [`set_encryption_info(Option<EncryptionInfo>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::set_encryption_info): <p>Includes all encryption-related information.</p>
    ///   - [`enhanced_monitoring(EnhancedMonitoring)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::enhanced_monitoring) / [`set_enhanced_monitoring(Option<EnhancedMonitoring>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::set_enhanced_monitoring): <p>Specifies the level of monitoring for the MSK cluster. The possible values are DEFAULT, PER_BROKER, PER_TOPIC_PER_BROKER, and PER_TOPIC_PER_PARTITION.</p>
    ///   - [`open_monitoring(OpenMonitoringInfo)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::open_monitoring) / [`set_open_monitoring(Option<OpenMonitoringInfo>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::set_open_monitoring): <p>The settings for open monitoring.</p>
    ///   - [`kafka_version(impl ::std::convert::Into<String>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::kafka_version) / [`set_kafka_version(Option<String>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::set_kafka_version): <p>The version of Apache Kafka.</p>
    ///   - [`logging_info(LoggingInfo)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::logging_info) / [`set_logging_info(Option<LoggingInfo>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::set_logging_info): (undocumented)
    ///   - [`number_of_broker_nodes(i32)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::number_of_broker_nodes) / [`set_number_of_broker_nodes(Option<i32>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::set_number_of_broker_nodes): <p>The number of broker nodes in the cluster.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::set_tags): <p>Create tags when creating the cluster.</p>
    ///   - [`storage_mode(StorageMode)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::storage_mode) / [`set_storage_mode(Option<StorageMode>)`](crate::operation::create_cluster::builders::CreateClusterFluentBuilder::set_storage_mode): <p>This controls storage mode for supported storage tiers.</p>
    /// - On success, responds with [`CreateClusterOutput`](crate::operation::create_cluster::CreateClusterOutput) with field(s):
    ///   - [`cluster_arn(Option<String>)`](crate::operation::create_cluster::CreateClusterOutput::cluster_arn): <p>The Amazon Resource Name (ARN) of the cluster.</p>
    ///   - [`cluster_name(Option<String>)`](crate::operation::create_cluster::CreateClusterOutput::cluster_name): <p>The name of the MSK cluster.</p>
    ///   - [`state(Option<ClusterState>)`](crate::operation::create_cluster::CreateClusterOutput::state): <p>The state of the cluster. The possible states are ACTIVE, CREATING, DELETING, FAILED, HEALING, MAINTENANCE, REBOOTING_BROKER, and UPDATING.</p>
    /// - On failure, responds with [`SdkError<CreateClusterError>`](crate::operation::create_cluster::CreateClusterError)
    pub fn create_cluster(
        &self,
    ) -> crate::operation::create_cluster::builders::CreateClusterFluentBuilder {
        crate::operation::create_cluster::builders::CreateClusterFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
