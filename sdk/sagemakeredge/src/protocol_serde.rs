// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_get_deployments;

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

pub(crate) mod shape_get_device_registration;

pub(crate) mod shape_send_heartbeat;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_get_deployments_input;

pub(crate) mod shape_get_device_registration_input;

pub(crate) mod shape_internal_service_exception;

pub(crate) mod shape_send_heartbeat_input;

pub(crate) mod shape_deployment_result;

pub(crate) mod shape_edge_deployments;

pub(crate) mod shape_edge_metric;

pub(crate) mod shape_model;

pub(crate) mod shape_deployment_model;

pub(crate) mod shape_edge_deployment;

pub(crate) mod shape_definitions;

pub(crate) mod shape_definition;

pub(crate) mod shape_checksum;
