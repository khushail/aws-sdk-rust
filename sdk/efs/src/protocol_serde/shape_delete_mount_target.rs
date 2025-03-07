// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_mount_target_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_mount_target::DeleteMountTargetOutput,
    crate::operation::delete_mount_target::DeleteMountTargetError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::delete_mount_target::DeleteMountTargetError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::delete_mount_target::DeleteMountTargetError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequest" => {
            crate::operation::delete_mount_target::DeleteMountTargetError::BadRequest({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::BadRequestBuilder::default();
                    output = crate::protocol_serde::shape_bad_request::de_bad_request_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(
                        crate::operation::delete_mount_target::DeleteMountTargetError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "DependencyTimeout" => {
            crate::operation::delete_mount_target::DeleteMountTargetError::DependencyTimeout({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::DependencyTimeoutBuilder::default();
                    output = crate::protocol_serde::shape_dependency_timeout::de_dependency_timeout_json_err(_response_body, output).map_err(crate::operation::delete_mount_target::DeleteMountTargetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InternalServerError" => {
            crate::operation::delete_mount_target::DeleteMountTargetError::InternalServerError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalServerErrorBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(_response_body, output).map_err(crate::operation::delete_mount_target::DeleteMountTargetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "MountTargetNotFound" => {
            crate::operation::delete_mount_target::DeleteMountTargetError::MountTargetNotFound({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::MountTargetNotFoundBuilder::default();
                    output = crate::protocol_serde::shape_mount_target_not_found::de_mount_target_not_found_json_err(_response_body, output).map_err(crate::operation::delete_mount_target::DeleteMountTargetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::delete_mount_target::DeleteMountTargetError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_mount_target_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_mount_target::DeleteMountTargetOutput,
    crate::operation::delete_mount_target::DeleteMountTargetError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_mount_target::builders::DeleteMountTargetOutputBuilder::default();
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
