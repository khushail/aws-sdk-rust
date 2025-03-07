// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_reject_input_device_transfer_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::reject_input_device_transfer::RejectInputDeviceTransferOutput,
    crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(
        crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::unhandled,
    )?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadGatewayException" => crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::BadGatewayException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::BadGatewayExceptionBuilder::default();
                    output = crate::protocol_serde::shape_bad_gateway_exception::de_bad_gateway_exception_json_err(_response_body, output).map_err(crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "BadRequestException" => crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::BadRequestExceptionBuilder::default();
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output).map_err(crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ConflictException" => crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                    output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output).map_err(crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ForbiddenException" => crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::ForbiddenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ForbiddenExceptionBuilder::default();
                    output = crate::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(_response_body, output).map_err(crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "GatewayTimeoutException" => crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::GatewayTimeoutException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::GatewayTimeoutExceptionBuilder::default();
                    output = crate::protocol_serde::shape_gateway_timeout_exception::de_gateway_timeout_exception_json_err(_response_body, output).map_err(crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerErrorException" => crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::InternalServerErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerErrorExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_error_exception::de_internal_server_error_exception_json_err(_response_body, output).map_err(crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output).map_err(crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output).map_err(crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnprocessableEntityException" => crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::UnprocessableEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnprocessableEntityExceptionBuilder::default();
                    output = crate::protocol_serde::shape_unprocessable_entity_exception::de_unprocessable_entity_exception_json_err(_response_body, output).map_err(crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_reject_input_device_transfer_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::reject_input_device_transfer::RejectInputDeviceTransferOutput,
    crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::reject_input_device_transfer::builders::RejectInputDeviceTransferOutputBuilder::default();
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
