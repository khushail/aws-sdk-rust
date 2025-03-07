// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_create_connector;

pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &::http::HeaderMap,
    response_body: &[u8],
) -> Result<
    ::aws_smithy_types::error::metadata::Builder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_create_custom_plugin;

pub(crate) mod shape_create_worker_configuration;

pub(crate) mod shape_delete_connector;

pub(crate) mod shape_delete_custom_plugin;

pub(crate) mod shape_describe_connector;

pub(crate) mod shape_describe_custom_plugin;

pub(crate) mod shape_describe_worker_configuration;

pub(crate) mod shape_list_connectors;

pub(crate) mod shape_list_custom_plugins;

pub(crate) mod shape_list_worker_configurations;

pub(crate) mod shape_update_connector;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_bad_request_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_connector_input;

pub(crate) mod shape_create_custom_plugin_input;

pub(crate) mod shape_create_worker_configuration_input;

pub(crate) mod shape_forbidden_exception;

pub(crate) mod shape_internal_server_error_exception;

pub(crate) mod shape_not_found_exception;

pub(crate) mod shape_service_unavailable_exception;

pub(crate) mod shape_too_many_requests_exception;

pub(crate) mod shape_unauthorized_exception;

pub(crate) mod shape_update_connector_input;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_connector_summary;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_custom_plugin_summary;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_plugin_description;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_worker_configuration_summary;

#[allow(non_snake_case)]
pub(crate) mod shape___sensitive__map_of__string;

pub(crate) mod shape_capacity;

pub(crate) mod shape_capacity_description;

pub(crate) mod shape_capacity_update;

pub(crate) mod shape_custom_plugin_location;

pub(crate) mod shape_custom_plugin_revision_summary;

pub(crate) mod shape_kafka_cluster;

pub(crate) mod shape_kafka_cluster_client_authentication;

pub(crate) mod shape_kafka_cluster_client_authentication_description;

pub(crate) mod shape_kafka_cluster_description;

pub(crate) mod shape_kafka_cluster_encryption_in_transit;

pub(crate) mod shape_kafka_cluster_encryption_in_transit_description;

pub(crate) mod shape_log_delivery;

pub(crate) mod shape_log_delivery_description;

pub(crate) mod shape_plugin;

pub(crate) mod shape_state_description;

pub(crate) mod shape_worker_configuration;

pub(crate) mod shape_worker_configuration_description;

pub(crate) mod shape_worker_configuration_revision_description;

pub(crate) mod shape_worker_configuration_revision_summary;

pub(crate) mod shape_apache_kafka_cluster;

pub(crate) mod shape_apache_kafka_cluster_description;

pub(crate) mod shape_auto_scaling;

pub(crate) mod shape_auto_scaling_description;

pub(crate) mod shape_auto_scaling_update;

pub(crate) mod shape_connector_summary;

pub(crate) mod shape_custom_plugin;

pub(crate) mod shape_custom_plugin_file_description;

pub(crate) mod shape_custom_plugin_location_description;

pub(crate) mod shape_custom_plugin_summary;

pub(crate) mod shape_plugin_description;

pub(crate) mod shape_provisioned_capacity;

pub(crate) mod shape_provisioned_capacity_description;

pub(crate) mod shape_provisioned_capacity_update;

pub(crate) mod shape_s3_location;

pub(crate) mod shape_worker_configuration_summary;

pub(crate) mod shape_worker_log_delivery;

pub(crate) mod shape_worker_log_delivery_description;

pub(crate) mod shape_cloud_watch_logs_log_delivery;

pub(crate) mod shape_cloud_watch_logs_log_delivery_description;

pub(crate) mod shape_custom_plugin_description;

pub(crate) mod shape_firehose_log_delivery;

pub(crate) mod shape_firehose_log_delivery_description;

pub(crate) mod shape_s3_location_description;

pub(crate) mod shape_s3_log_delivery;

pub(crate) mod shape_s3_log_delivery_description;

pub(crate) mod shape_scale_in_policy;

pub(crate) mod shape_scale_in_policy_description;

pub(crate) mod shape_scale_in_policy_update;

pub(crate) mod shape_scale_out_policy;

pub(crate) mod shape_scale_out_policy_description;

pub(crate) mod shape_scale_out_policy_update;

pub(crate) mod shape_vpc;

pub(crate) mod shape_vpc_description;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of__string;
