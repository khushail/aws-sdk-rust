// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_copy_backup_to_region;

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

pub(crate) mod shape_create_cluster;

pub(crate) mod shape_create_hsm;

pub(crate) mod shape_delete_backup;

pub(crate) mod shape_delete_cluster;

pub(crate) mod shape_delete_hsm;

pub(crate) mod shape_describe_backups;

pub(crate) mod shape_describe_clusters;

pub(crate) mod shape_initialize_cluster;

pub(crate) mod shape_list_tags;

pub(crate) mod shape_modify_backup_attributes;

pub(crate) mod shape_modify_cluster;

pub(crate) mod shape_restore_backup;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_cloud_hsm_access_denied_exception;

pub(crate) mod shape_cloud_hsm_internal_failure_exception;

pub(crate) mod shape_cloud_hsm_invalid_request_exception;

pub(crate) mod shape_cloud_hsm_resource_not_found_exception;

pub(crate) mod shape_cloud_hsm_service_exception;

pub(crate) mod shape_cloud_hsm_tag_exception;

pub(crate) mod shape_copy_backup_to_region_input;

pub(crate) mod shape_create_cluster_input;

pub(crate) mod shape_create_hsm_input;

pub(crate) mod shape_delete_backup_input;

pub(crate) mod shape_delete_cluster_input;

pub(crate) mod shape_delete_hsm_input;

pub(crate) mod shape_describe_backups_input;

pub(crate) mod shape_describe_clusters_input;

pub(crate) mod shape_initialize_cluster_input;

pub(crate) mod shape_list_tags_input;

pub(crate) mod shape_modify_backup_attributes_input;

pub(crate) mod shape_modify_cluster_input;

pub(crate) mod shape_restore_backup_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_backup;

pub(crate) mod shape_backup_retention_policy;

pub(crate) mod shape_backups;

pub(crate) mod shape_cluster;

pub(crate) mod shape_clusters;

pub(crate) mod shape_destination_backup;

pub(crate) mod shape_hsm;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_certificates;

pub(crate) mod shape_external_subnet_mapping;

pub(crate) mod shape_hsms;
