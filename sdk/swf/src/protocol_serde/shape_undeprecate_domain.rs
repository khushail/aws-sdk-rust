// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_undeprecate_domain_input(
    input: &crate::operation::undeprecate_domain::UndeprecateDomainInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_undeprecate_domain_input::ser_undeprecate_domain_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_undeprecate_domain_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::undeprecate_domain::UndeprecateDomainOutput,
    crate::operation::undeprecate_domain::UndeprecateDomainError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::undeprecate_domain::UndeprecateDomainError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::undeprecate_domain::UndeprecateDomainError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DomainAlreadyExistsFault" => {
            crate::operation::undeprecate_domain::UndeprecateDomainError::DomainAlreadyExistsFault(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::types::error::builders::DomainAlreadyExistsFaultBuilder::default(
                            );
                        output = crate::protocol_serde::shape_domain_already_exists_fault::de_domain_already_exists_fault_json_err(_response_body, output).map_err(crate::operation::undeprecate_domain::UndeprecateDomainError::unhandled)?;
                        let output = output.meta(generic);
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            )
        }
        "OperationNotPermittedFault" => {
            crate::operation::undeprecate_domain::UndeprecateDomainError::OperationNotPermittedFault(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::OperationNotPermittedFaultBuilder::default();
                        output = crate::protocol_serde::shape_operation_not_permitted_fault::de_operation_not_permitted_fault_json_err(_response_body, output).map_err(crate::operation::undeprecate_domain::UndeprecateDomainError::unhandled)?;
                        let output = output.meta(generic);
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            )
        }
        "UnknownResourceFault" => {
            crate::operation::undeprecate_domain::UndeprecateDomainError::UnknownResourceFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::UnknownResourceFaultBuilder::default();
                    output = crate::protocol_serde::shape_unknown_resource_fault::de_unknown_resource_fault_json_err(_response_body, output).map_err(crate::operation::undeprecate_domain::UndeprecateDomainError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::undeprecate_domain::UndeprecateDomainError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_undeprecate_domain_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::undeprecate_domain::UndeprecateDomainOutput,
    crate::operation::undeprecate_domain::UndeprecateDomainError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::undeprecate_domain::builders::UndeprecateDomainOutputBuilder::default(
            );
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
