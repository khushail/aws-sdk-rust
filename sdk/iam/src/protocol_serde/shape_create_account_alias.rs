// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_account_alias_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_account_alias::CreateAccountAliasOutput,
    crate::operation::create_account_alias::CreateAccountAliasError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::create_account_alias::CreateAccountAliasError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::create_account_alias::CreateAccountAliasError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "EntityAlreadyExists" => crate::operation::create_account_alias::CreateAccountAliasError::EntityAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::EntityAlreadyExistsExceptionBuilder::default();
                    output = crate::protocol_serde::shape_entity_already_exists_exception::de_entity_already_exists_exception_xml_err(_response_body, output).map_err(crate::operation::create_account_alias::CreateAccountAliasError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceeded" => crate::operation::create_account_alias::CreateAccountAliasError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(_response_body, output).map_err(crate::operation::create_account_alias::CreateAccountAliasError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceFailure" => crate::operation::create_account_alias::CreateAccountAliasError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(_response_body, output).map_err(crate::operation::create_account_alias::CreateAccountAliasError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::create_account_alias::CreateAccountAliasError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_account_alias_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_account_alias::CreateAccountAliasOutput,
    crate::operation::create_account_alias::CreateAccountAliasError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_account_alias::builders::CreateAccountAliasOutputBuilder::default();
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
