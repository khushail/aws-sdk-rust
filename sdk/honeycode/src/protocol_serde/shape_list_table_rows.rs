// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_table_rows_input(
    input: &crate::operation::list_table_rows::ListTableRowsInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_table_rows_input::ser_list_table_rows_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_table_rows_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_table_rows::ListTableRowsOutput,
    crate::operation::list_table_rows::ListTableRowsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::list_table_rows::ListTableRowsError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::list_table_rows::ListTableRowsError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::list_table_rows::ListTableRowsError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output).map_err(crate::operation::list_table_rows::ListTableRowsError::unhandled)?;
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
            crate::operation::list_table_rows::ListTableRowsError::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalServerExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output).map_err(crate::operation::list_table_rows::ListTableRowsError::unhandled)?;
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
            crate::operation::list_table_rows::ListTableRowsError::RequestTimeoutException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::RequestTimeoutExceptionBuilder::default();
                    output = crate::protocol_serde::shape_request_timeout_exception::de_request_timeout_exception_json_err(_response_body, output).map_err(crate::operation::list_table_rows::ListTableRowsError::unhandled)?;
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
            crate::operation::list_table_rows::ListTableRowsError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output).map_err(crate::operation::list_table_rows::ListTableRowsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServiceUnavailableException" => {
            crate::operation::list_table_rows::ListTableRowsError::ServiceUnavailableException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ServiceUnavailableExceptionBuilder::default(
                        );
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(_response_body, output).map_err(crate::operation::list_table_rows::ListTableRowsError::unhandled)?;
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
            crate::operation::list_table_rows::ListTableRowsError::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output).map_err(crate::operation::list_table_rows::ListTableRowsError::unhandled)?;
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
            crate::operation::list_table_rows::ListTableRowsError::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ValidationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(crate::operation::list_table_rows::ListTableRowsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::list_table_rows::ListTableRowsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_table_rows_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_table_rows::ListTableRowsOutput,
    crate::operation::list_table_rows::ListTableRowsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::list_table_rows::builders::ListTableRowsOutputBuilder::default();
        output = crate::protocol_serde::shape_list_table_rows::de_list_table_rows(
            _response_body,
            output,
        )
        .map_err(crate::operation::list_table_rows::ListTableRowsError::unhandled)?;
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_list_table_rows(
    value: &[u8],
    mut builder: crate::operation::list_table_rows::builders::ListTableRowsOutputBuilder,
) -> Result<
    crate::operation::list_table_rows::builders::ListTableRowsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned =
        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "columnIds" => {
                        builder = builder.set_column_ids(
                            crate::protocol_serde::shape_resource_ids::de_resource_ids(tokens)?,
                        );
                    }
                    "nextToken" => {
                        builder = builder.set_next_token(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "rowIdsNotFound" => {
                        builder = builder.set_row_ids_not_found(
                            crate::protocol_serde::shape_row_id_list::de_row_id_list(tokens)?,
                        );
                    }
                    "rows" => {
                        builder = builder.set_rows(
                            crate::protocol_serde::shape_table_rows::de_table_rows(tokens)?,
                        );
                    }
                    "workbookCursor" => {
                        builder = builder.set_workbook_cursor(
                            ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                tokens.next(),
                            )?
                            .map(i64::try_from)
                            .transpose()?,
                        );
                    }
                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(
                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )),
                )
            }
        }
    }
    if tokens.next().is_some() {
        return Err(
            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "found more JSON tokens after completing parsing",
            ),
        );
    }
    Ok(builder)
}
