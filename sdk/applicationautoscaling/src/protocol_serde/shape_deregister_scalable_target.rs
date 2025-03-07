// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_deregister_scalable_target_input(
    input: &crate::operation::deregister_scalable_target::DeregisterScalableTargetInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_deregister_scalable_target_input::ser_deregister_scalable_target_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deregister_scalable_target_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::deregister_scalable_target::DeregisterScalableTargetOutput,
    crate::operation::deregister_scalable_target::DeregisterScalableTargetError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(
        crate::operation::deregister_scalable_target::DeregisterScalableTargetError::unhandled,
    )?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::deregister_scalable_target::DeregisterScalableTargetError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentUpdateException" => crate::operation::deregister_scalable_target::DeregisterScalableTargetError::ConcurrentUpdateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConcurrentUpdateExceptionBuilder::default();
                    output = crate::protocol_serde::shape_concurrent_update_exception::de_concurrent_update_exception_json_err(_response_body, output).map_err(crate::operation::deregister_scalable_target::DeregisterScalableTargetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServiceException" => crate::operation::deregister_scalable_target::DeregisterScalableTargetError::InternalServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServiceExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_json_err(_response_body, output).map_err(crate::operation::deregister_scalable_target::DeregisterScalableTargetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ObjectNotFoundException" => crate::operation::deregister_scalable_target::DeregisterScalableTargetError::ObjectNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ObjectNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_object_not_found_exception::de_object_not_found_exception_json_err(_response_body, output).map_err(crate::operation::deregister_scalable_target::DeregisterScalableTargetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::operation::deregister_scalable_target::DeregisterScalableTargetError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(crate::operation::deregister_scalable_target::DeregisterScalableTargetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::deregister_scalable_target::DeregisterScalableTargetError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deregister_scalable_target_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::deregister_scalable_target::DeregisterScalableTargetOutput,
    crate::operation::deregister_scalable_target::DeregisterScalableTargetError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::deregister_scalable_target::builders::DeregisterScalableTargetOutputBuilder::default();
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
