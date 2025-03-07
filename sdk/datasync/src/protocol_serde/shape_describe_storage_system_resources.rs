// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_storage_system_resources_input(
    input: &crate::operation::describe_storage_system_resources::DescribeStorageSystemResourcesInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_storage_system_resources_input::ser_describe_storage_system_resources_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_storage_system_resources_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_storage_system_resources::DescribeStorageSystemResourcesOutput,
    crate::operation::describe_storage_system_resources::DescribeStorageSystemResourcesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::describe_storage_system_resources::DescribeStorageSystemResourcesError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::describe_storage_system_resources::DescribeStorageSystemResourcesError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalException" => crate::operation::describe_storage_system_resources::DescribeStorageSystemResourcesError::InternalException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_exception::de_internal_exception_json_err(_response_body, output).map_err(crate::operation::describe_storage_system_resources::DescribeStorageSystemResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRequestException" => crate::operation::describe_storage_system_resources::DescribeStorageSystemResourcesError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output).map_err(crate::operation::describe_storage_system_resources::DescribeStorageSystemResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::describe_storage_system_resources::DescribeStorageSystemResourcesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_storage_system_resources_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_storage_system_resources::DescribeStorageSystemResourcesOutput,
    crate::operation::describe_storage_system_resources::DescribeStorageSystemResourcesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_storage_system_resources::builders::DescribeStorageSystemResourcesOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_storage_system_resources::de_describe_storage_system_resources(_response_body, output).map_err(crate::operation::describe_storage_system_resources::DescribeStorageSystemResourcesError::unhandled)?;
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_describe_storage_system_resources(value: &[u8], mut builder: crate::operation::describe_storage_system_resources::builders::DescribeStorageSystemResourcesOutputBuilder) -> Result<crate::operation::describe_storage_system_resources::builders::DescribeStorageSystemResourcesOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>{
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
                    "ResourceDetails" => {
                        builder = builder.set_resource_details(
                            crate::protocol_serde::shape_resource_details::de_resource_details(
                                tokens,
                            )?,
                        );
                    }
                    "NextToken" => {
                        builder = builder.set_next_token(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
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
