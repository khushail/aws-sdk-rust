// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_policy_version_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_policy_version::GetPolicyVersionOutput,
    crate::operation::get_policy_version::GetPolicyVersionError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::get_policy_version::GetPolicyVersionError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::get_policy_version::GetPolicyVersionError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInput" => {
            crate::operation::get_policy_version::GetPolicyVersionError::InvalidInputException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidInputExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_xml_err(_response_body, output).map_err(crate::operation::get_policy_version::GetPolicyVersionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NoSuchEntity" => {
            crate::operation::get_policy_version::GetPolicyVersionError::NoSuchEntityException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(_response_body, output).map_err(crate::operation::get_policy_version::GetPolicyVersionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServiceFailure" => {
            crate::operation::get_policy_version::GetPolicyVersionError::ServiceFailureException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(_response_body, output).map_err(crate::operation::get_policy_version::GetPolicyVersionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_policy_version::GetPolicyVersionError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_policy_version_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_policy_version::GetPolicyVersionOutput,
    crate::operation::get_policy_version::GetPolicyVersionError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::get_policy_version::builders::GetPolicyVersionOutputBuilder::default(
            );
        output = crate::protocol_serde::shape_get_policy_version::de_get_policy_version(
            _response_body,
            output,
        )
        .map_err(crate::operation::get_policy_version::GetPolicyVersionError::unhandled)?;
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_policy_version(
    inp: &[u8],
    mut builder: crate::operation::get_policy_version::builders::GetPolicyVersionOutputBuilder,
) -> Result<
    crate::operation::get_policy_version::builders::GetPolicyVersionOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("GetPolicyVersionResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected GetPolicyVersionResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("GetPolicyVersionResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected GetPolicyVersionResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("PolicyVersion") /* PolicyVersion com.amazonaws.iam.synthetic#GetPolicyVersionOutput$PolicyVersion */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_policy_version::de_policy_version(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_policy_version(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected GetPolicyVersionResult tag",
        ));
    };
    Ok(builder)
}
