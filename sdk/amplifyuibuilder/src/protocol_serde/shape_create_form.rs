// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_form_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_form::CreateFormOutput,
    crate::operation::create_form::CreateFormError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::create_form::CreateFormError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::create_form::CreateFormError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => {
            crate::operation::create_form::CreateFormError::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalServerExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output).map_err(crate::operation::create_form::CreateFormError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidParameterException" => {
            crate::operation::create_form::CreateFormError::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output).map_err(crate::operation::create_form::CreateFormError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceConflictException" => {
            crate::operation::create_form::CreateFormError::ResourceConflictException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ResourceConflictExceptionBuilder::default();
                    output = crate::protocol_serde::shape_resource_conflict_exception::de_resource_conflict_exception_json_err(_response_body, output).map_err(crate::operation::create_form::CreateFormError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServiceQuotaExceededException" => {
            crate::operation::create_form::CreateFormError::ServiceQuotaExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceQuotaExceededExceptionBuilder::default();
                    output = crate::protocol_serde::shape_service_quota_exceeded_exception::de_service_quota_exceeded_exception_json_err(_response_body, output).map_err(crate::operation::create_form::CreateFormError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::create_form::CreateFormError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_form_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_form::CreateFormOutput,
    crate::operation::create_form::CreateFormError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::create_form::builders::CreateFormOutputBuilder::default();
        output = output.set_entity(
            crate::protocol_serde::shape_create_form_output::de_entity_payload(_response_body)?,
        );
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
