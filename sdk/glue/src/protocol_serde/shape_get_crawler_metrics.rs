// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_crawler_metrics_input(
    input: &crate::operation::get_crawler_metrics::GetCrawlerMetricsInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_crawler_metrics_input::ser_get_crawler_metrics_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_crawler_metrics_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_crawler_metrics::GetCrawlerMetricsOutput,
    crate::operation::get_crawler_metrics::GetCrawlerMetricsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::get_crawler_metrics::GetCrawlerMetricsError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::get_crawler_metrics::GetCrawlerMetricsError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "OperationTimeoutException" => {
            crate::operation::get_crawler_metrics::GetCrawlerMetricsError::OperationTimeoutException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::OperationTimeoutExceptionBuilder::default();
                        output = crate::protocol_serde::shape_operation_timeout_exception::de_operation_timeout_exception_json_err(_response_body, output).map_err(crate::operation::get_crawler_metrics::GetCrawlerMetricsError::unhandled)?;
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
        _ => crate::operation::get_crawler_metrics::GetCrawlerMetricsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_crawler_metrics_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_crawler_metrics::GetCrawlerMetricsOutput,
    crate::operation::get_crawler_metrics::GetCrawlerMetricsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_crawler_metrics::builders::GetCrawlerMetricsOutputBuilder::default();
        output = crate::protocol_serde::shape_get_crawler_metrics::de_get_crawler_metrics(
            _response_body,
            output,
        )
        .map_err(crate::operation::get_crawler_metrics::GetCrawlerMetricsError::unhandled)?;
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_get_crawler_metrics(
    value: &[u8],
    mut builder: crate::operation::get_crawler_metrics::builders::GetCrawlerMetricsOutputBuilder,
) -> Result<
    crate::operation::get_crawler_metrics::builders::GetCrawlerMetricsOutputBuilder,
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
                    "CrawlerMetricsList" => {
                        builder = builder.set_crawler_metrics_list(
                            crate::protocol_serde::shape_crawler_metrics_list::de_crawler_metrics_list(tokens)?
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
