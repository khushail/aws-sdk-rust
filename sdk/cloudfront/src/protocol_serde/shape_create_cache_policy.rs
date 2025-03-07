// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_cache_policy_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_cache_policy::CreateCachePolicyOutput,
    crate::operation::create_cache_policy::CreateCachePolicyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::create_cache_policy::CreateCachePolicyError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::create_cache_policy::CreateCachePolicyError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDenied" => crate::operation::create_cache_policy::CreateCachePolicyError::AccessDenied({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedBuilder::default();
                    output = crate::protocol_serde::shape_access_denied::de_access_denied_xml_err(_response_body, output).map_err(crate::operation::create_cache_policy::CreateCachePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CachePolicyAlreadyExists" => crate::operation::create_cache_policy::CreateCachePolicyError::CachePolicyAlreadyExists({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::CachePolicyAlreadyExistsBuilder::default();
                    output = crate::protocol_serde::shape_cache_policy_already_exists::de_cache_policy_already_exists_xml_err(_response_body, output).map_err(crate::operation::create_cache_policy::CreateCachePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InconsistentQuantities" => crate::operation::create_cache_policy::CreateCachePolicyError::InconsistentQuantities({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InconsistentQuantitiesBuilder::default();
                    output = crate::protocol_serde::shape_inconsistent_quantities::de_inconsistent_quantities_xml_err(_response_body, output).map_err(crate::operation::create_cache_policy::CreateCachePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArgument" => crate::operation::create_cache_policy::CreateCachePolicyError::InvalidArgument({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidArgumentBuilder::default();
                    output = crate::protocol_serde::shape_invalid_argument::de_invalid_argument_xml_err(_response_body, output).map_err(crate::operation::create_cache_policy::CreateCachePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyCachePolicies" => crate::operation::create_cache_policy::CreateCachePolicyError::TooManyCachePolicies({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyCachePoliciesBuilder::default();
                    output = crate::protocol_serde::shape_too_many_cache_policies::de_too_many_cache_policies_xml_err(_response_body, output).map_err(crate::operation::create_cache_policy::CreateCachePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyCookiesInCachePolicy" => crate::operation::create_cache_policy::CreateCachePolicyError::TooManyCookiesInCachePolicy({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyCookiesInCachePolicyBuilder::default();
                    output = crate::protocol_serde::shape_too_many_cookies_in_cache_policy::de_too_many_cookies_in_cache_policy_xml_err(_response_body, output).map_err(crate::operation::create_cache_policy::CreateCachePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyHeadersInCachePolicy" => crate::operation::create_cache_policy::CreateCachePolicyError::TooManyHeadersInCachePolicy({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyHeadersInCachePolicyBuilder::default();
                    output = crate::protocol_serde::shape_too_many_headers_in_cache_policy::de_too_many_headers_in_cache_policy_xml_err(_response_body, output).map_err(crate::operation::create_cache_policy::CreateCachePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyQueryStringsInCachePolicy" => crate::operation::create_cache_policy::CreateCachePolicyError::TooManyQueryStringsInCachePolicy({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyQueryStringsInCachePolicyBuilder::default();
                    output = crate::protocol_serde::shape_too_many_query_strings_in_cache_policy::de_too_many_query_strings_in_cache_policy_xml_err(_response_body, output).map_err(crate::operation::create_cache_policy::CreateCachePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::create_cache_policy::CreateCachePolicyError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_cache_policy_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_cache_policy::CreateCachePolicyOutput,
    crate::operation::create_cache_policy::CreateCachePolicyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_cache_policy::builders::CreateCachePolicyOutputBuilder::default();
        output = output.set_cache_policy(
            crate::protocol_serde::shape_create_cache_policy_output::de_cache_policy_payload(
                _response_body,
            )?,
        );
        output = output.set_e_tag(
            crate::protocol_serde::shape_create_cache_policy_output::de_e_tag_header(
                _response_headers,
            )
            .map_err(|_| {
                crate::operation::create_cache_policy::CreateCachePolicyError::unhandled(
                    "Failed to parse ETag from header `ETag",
                )
            })?,
        );
        output = output.set_location(
            crate::protocol_serde::shape_create_cache_policy_output::de_location_header(
                _response_headers,
            )
            .map_err(|_| {
                crate::operation::create_cache_policy::CreateCachePolicyError::unhandled(
                    "Failed to parse Location from header `Location",
                )
            })?,
        );
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
