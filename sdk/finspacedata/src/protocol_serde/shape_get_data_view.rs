// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_data_view_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_data_view::GetDataViewOutput,
    crate::operation::get_data_view::GetDataViewError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::get_data_view::GetDataViewError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_data_view::GetDataViewError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConflictException" => {
            crate::operation::get_data_view::GetDataViewError::ConflictException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ConflictExceptionBuilder::default();
                    output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output).map_err(crate::operation::get_data_view::GetDataViewError::unhandled)?;
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
            crate::operation::get_data_view::GetDataViewError::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalServerExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output).map_err(crate::operation::get_data_view::GetDataViewError::unhandled)?;
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
            crate::operation::get_data_view::GetDataViewError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output).map_err(crate::operation::get_data_view::GetDataViewError::unhandled)?;
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
            crate::operation::get_data_view::GetDataViewError::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output).map_err(crate::operation::get_data_view::GetDataViewError::unhandled)?;
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
            crate::operation::get_data_view::GetDataViewError::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ValidationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(crate::operation::get_data_view::GetDataViewError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_data_view::GetDataViewError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_data_view_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_data_view::GetDataViewOutput,
    crate::operation::get_data_view::GetDataViewError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::get_data_view::builders::GetDataViewOutputBuilder::default();
        output =
            crate::protocol_serde::shape_get_data_view::de_get_data_view(_response_body, output)
                .map_err(crate::operation::get_data_view::GetDataViewError::unhandled)?;
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_get_data_view(
    value: &[u8],
    mut builder: crate::operation::get_data_view::builders::GetDataViewOutputBuilder,
) -> Result<
    crate::operation::get_data_view::builders::GetDataViewOutputBuilder,
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
                    "asOfTimestamp" => {
                        builder = builder.set_as_of_timestamp(
                            ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                tokens.next(),
                            )?
                            .map(i64::try_from)
                            .transpose()?,
                        );
                    }
                    "autoUpdate" => {
                        builder = builder.set_auto_update(
                            ::aws_smithy_json::deserialize::token::expect_bool_or_null(
                                tokens.next(),
                            )?,
                        );
                    }
                    "createTime" => {
                        builder = builder.set_create_time(
                            ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                tokens.next(),
                            )?
                            .map(i64::try_from)
                            .transpose()?,
                        );
                    }
                    "dataViewArn" => {
                        builder = builder.set_data_view_arn(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "dataViewId" => {
                        builder = builder.set_data_view_id(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "datasetId" => {
                        builder = builder.set_dataset_id(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "destinationTypeParams" => {
                        builder = builder.set_destination_type_params(
                            crate::protocol_serde::shape_data_view_destination_type_params::de_data_view_destination_type_params(tokens)?
                        );
                    }
                    "errorInfo" => {
                        builder = builder.set_error_info(
                            crate::protocol_serde::shape_data_view_error_info::de_data_view_error_info(tokens)?
                        );
                    }
                    "lastModifiedTime" => {
                        builder = builder.set_last_modified_time(
                            ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                tokens.next(),
                            )?
                            .map(i64::try_from)
                            .transpose()?,
                        );
                    }
                    "partitionColumns" => {
                        builder = builder.set_partition_columns(
                            crate::protocol_serde::shape_partition_column_list::de_partition_column_list(tokens)?
                        );
                    }
                    "sortColumns" => {
                        builder = builder.set_sort_columns(
                            crate::protocol_serde::shape_sort_column_list::de_sort_column_list(
                                tokens,
                            )?,
                        );
                    }
                    "status" => {
                        builder = builder.set_status(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| {
                                s.to_unescaped()
                                    .map(|u| crate::types::DataViewStatus::from(u.as_ref()))
                            })
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
