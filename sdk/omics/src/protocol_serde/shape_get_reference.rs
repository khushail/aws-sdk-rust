// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_reference_headers(
    input: &crate::operation::get_reference::GetReferenceInput,
    mut builder: ::http::request::Builder,
) -> std::result::Result<::http::request::Builder, ::aws_smithy_http::operation::error::BuildError>
{
    if let ::std::option::Option::Some(inner_1) = &input.range {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_http::operation::error::BuildError::invalid_field(
                    "range",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("Range", header_value);
        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_reference_op_response(
    op_response: &mut ::aws_smithy_http::operation::Response,
) -> ::std::result::Result<
    crate::operation::get_reference::GetReferenceOutput,
    crate::operation::get_reference::GetReferenceError,
> {
    #[allow(unused_variables)]
    let (response, properties) = op_response.parts_mut();
    crate::protocol_serde::shape_get_reference::de_get_reference_http_response_with_props(
        response,
        &properties,
    )
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_reference_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_reference::GetReferenceOutput,
    crate::operation::get_reference::GetReferenceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::get_reference::GetReferenceError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_reference::GetReferenceError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::get_reference::GetReferenceError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output).map_err(crate::operation::get_reference::GetReferenceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InternalServerException" => {
            crate::operation::get_reference::GetReferenceError::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalServerExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output).map_err(crate::operation::get_reference::GetReferenceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "RangeNotSatisfiableException" => {
            crate::operation::get_reference::GetReferenceError::RangeNotSatisfiableException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::RangeNotSatisfiableExceptionBuilder::default(
                        );
                    output = crate::protocol_serde::shape_range_not_satisfiable_exception::de_range_not_satisfiable_exception_json_err(_response_body, output).map_err(crate::operation::get_reference::GetReferenceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "RequestTimeoutException" => {
            crate::operation::get_reference::GetReferenceError::RequestTimeoutException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::RequestTimeoutExceptionBuilder::default();
                    output = crate::protocol_serde::shape_request_timeout_exception::de_request_timeout_exception_json_err(_response_body, output).map_err(crate::operation::get_reference::GetReferenceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceNotFoundException" => {
            crate::operation::get_reference::GetReferenceError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output).map_err(crate::operation::get_reference::GetReferenceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ThrottlingException" => {
            crate::operation::get_reference::GetReferenceError::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output).map_err(crate::operation::get_reference::GetReferenceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ValidationException" => {
            crate::operation::get_reference::GetReferenceError::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ValidationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(crate::operation::get_reference::GetReferenceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_reference::GetReferenceError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
#[allow(unused_variables)]
pub fn de_get_reference_http_response_with_props(
    response: &mut ::http::Response<::aws_smithy_http::body::SdkBody>,
    properties: &::aws_smithy_http::property_bag::PropertyBag,
) -> std::result::Result<
    crate::operation::get_reference::GetReferenceOutput,
    crate::operation::get_reference::GetReferenceError,
> {
    let mut _response_body = ::aws_smithy_http::body::SdkBody::taken();
    std::mem::swap(&mut _response_body, response.body_mut());
    let _response_body = &mut _response_body;

    let _response_status = response.status().as_u16();
    let _response_headers = response.headers();
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::get_reference::builders::GetReferenceOutputBuilder::default();
        output = output.set_payload(Some(
            crate::protocol_serde::shape_get_reference_output::de_payload_payload(_response_body)?,
        ));
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
