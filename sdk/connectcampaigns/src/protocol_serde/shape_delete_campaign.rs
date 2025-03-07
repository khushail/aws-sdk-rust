// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_campaign_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_campaign::DeleteCampaignOutput,
    crate::operation::delete_campaign::DeleteCampaignError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::delete_campaign::DeleteCampaignError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::delete_campaign::DeleteCampaignError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::delete_campaign::DeleteCampaignError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output).map_err(crate::operation::delete_campaign::DeleteCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_access_denied_exception::de_x_amz_error_type_header(_response_headers)
                                                .map_err(|_|crate::operation::delete_campaign::DeleteCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
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
            crate::operation::delete_campaign::DeleteCampaignError::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalServerExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output).map_err(crate::operation::delete_campaign::DeleteCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_internal_server_exception::de_x_amz_error_type_header(_response_headers)
                                                .map_err(|_|crate::operation::delete_campaign::DeleteCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
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
            crate::operation::delete_campaign::DeleteCampaignError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output).map_err(crate::operation::delete_campaign::DeleteCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_resource_not_found_exception::de_x_amz_error_type_header(_response_headers)
                                                .map_err(|_|crate::operation::delete_campaign::DeleteCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
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
            crate::operation::delete_campaign::DeleteCampaignError::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ValidationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(crate::operation::delete_campaign::DeleteCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_validation_exception::de_x_amz_error_type_header(_response_headers)
                                                .map_err(|_|crate::operation::delete_campaign::DeleteCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::delete_campaign::DeleteCampaignError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_campaign_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_campaign::DeleteCampaignOutput,
    crate::operation::delete_campaign::DeleteCampaignError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::delete_campaign::builders::DeleteCampaignOutputBuilder::default();
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
