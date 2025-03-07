// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_auto_scaling_policy_input(
    input: &crate::operation::put_auto_scaling_policy::PutAutoScalingPolicyInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_put_auto_scaling_policy_input::ser_put_auto_scaling_policy_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_auto_scaling_policy_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::put_auto_scaling_policy::PutAutoScalingPolicyOutput,
    crate::operation::put_auto_scaling_policy::PutAutoScalingPolicyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::put_auto_scaling_policy::PutAutoScalingPolicyError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::put_auto_scaling_policy::PutAutoScalingPolicyError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_auto_scaling_policy_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::put_auto_scaling_policy::PutAutoScalingPolicyOutput,
    crate::operation::put_auto_scaling_policy::PutAutoScalingPolicyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_auto_scaling_policy::builders::PutAutoScalingPolicyOutputBuilder::default();
        output = crate::protocol_serde::shape_put_auto_scaling_policy::de_put_auto_scaling_policy(
            _response_body,
            output,
        )
        .map_err(crate::operation::put_auto_scaling_policy::PutAutoScalingPolicyError::unhandled)?;
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_put_auto_scaling_policy(
    value: &[u8],
    mut builder: crate::operation::put_auto_scaling_policy::builders::PutAutoScalingPolicyOutputBuilder,
) -> Result<
    crate::operation::put_auto_scaling_policy::builders::PutAutoScalingPolicyOutputBuilder,
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
                    "ClusterId" => {
                        builder = builder.set_cluster_id(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "InstanceGroupId" => {
                        builder = builder.set_instance_group_id(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "AutoScalingPolicy" => {
                        builder = builder.set_auto_scaling_policy(
                            crate::protocol_serde::shape_auto_scaling_policy_description::de_auto_scaling_policy_description(tokens)?
                        );
                    }
                    "ClusterArn" => {
                        builder = builder.set_cluster_arn(
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
