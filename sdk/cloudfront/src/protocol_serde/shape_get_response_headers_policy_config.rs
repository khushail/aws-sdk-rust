// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_response_headers_policy_config_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_response_headers_policy_config::GetResponseHeadersPolicyConfigOutput,
    crate::operation::get_response_headers_policy_config::GetResponseHeadersPolicyConfigError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::get_response_headers_policy_config::GetResponseHeadersPolicyConfigError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::get_response_headers_policy_config::GetResponseHeadersPolicyConfigError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDenied" => crate::operation::get_response_headers_policy_config::GetResponseHeadersPolicyConfigError::AccessDenied({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedBuilder::default();
                    output = crate::protocol_serde::shape_access_denied::de_access_denied_xml_err(_response_body, output).map_err(crate::operation::get_response_headers_policy_config::GetResponseHeadersPolicyConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchResponseHeadersPolicy" => crate::operation::get_response_headers_policy_config::GetResponseHeadersPolicyConfigError::NoSuchResponseHeadersPolicy({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchResponseHeadersPolicyBuilder::default();
                    output = crate::protocol_serde::shape_no_such_response_headers_policy::de_no_such_response_headers_policy_xml_err(_response_body, output).map_err(crate::operation::get_response_headers_policy_config::GetResponseHeadersPolicyConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::get_response_headers_policy_config::GetResponseHeadersPolicyConfigError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_response_headers_policy_config_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_response_headers_policy_config::GetResponseHeadersPolicyConfigOutput,
    crate::operation::get_response_headers_policy_config::GetResponseHeadersPolicyConfigError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_response_headers_policy_config::builders::GetResponseHeadersPolicyConfigOutputBuilder::default();
        output = output.set_e_tag(
            crate::protocol_serde::shape_get_response_headers_policy_config_output::de_e_tag_header(_response_headers)
                                    .map_err(|_|crate::operation::get_response_headers_policy_config::GetResponseHeadersPolicyConfigError::unhandled("Failed to parse ETag from header `ETag"))?
        );
        output = output.set_response_headers_policy_config(
            crate::protocol_serde::shape_get_response_headers_policy_config_output::de_response_headers_policy_config_payload(_response_body)?
        );
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
