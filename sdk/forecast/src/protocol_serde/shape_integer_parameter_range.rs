// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_integer_parameter_range(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::IntegerParameterRange,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.max_value {
        object.key("MaxValue").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.min_value {
        object.key("MinValue").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.scaling_type {
        object.key("ScalingType").string(var_4.as_str());
    }
    Ok(())
}

pub(crate) fn de_integer_parameter_range<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::IntegerParameterRange>,
    ::aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<
        Item = Result<
            ::aws_smithy_json::deserialize::Token<'a>,
            ::aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::IntegerParameterRangeBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Name" => {
                                builder = builder.set_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "MaxValue" => {
                                builder = builder.set_max_value(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "MinValue" => {
                                builder = builder.set_min_value(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "ScalingType" => {
                                builder = builder.set_scaling_type(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::ScalingType::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    other => {
                        return Err(
                            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                format!("expected object key or end object, found: {:?}", other),
                            ),
                        )
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(
            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ),
        ),
    }
}
