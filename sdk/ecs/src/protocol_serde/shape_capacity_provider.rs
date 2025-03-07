// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_capacity_provider<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::CapacityProvider>,
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
            let mut builder = crate::types::builders::CapacityProviderBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key
                        .to_unescaped()?
                        .as_ref()
                    {
                        "capacityProviderArn" => {
                            builder = builder.set_capacity_provider_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "status" => {
                            builder = builder.set_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| {
                                    s.to_unescaped().map(|u| {
                                        crate::types::CapacityProviderStatus::from(u.as_ref())
                                    })
                                })
                                .transpose()?,
                            );
                        }
                        "autoScalingGroupProvider" => {
                            builder = builder.set_auto_scaling_group_provider(
                                    crate::protocol_serde::shape_auto_scaling_group_provider::de_auto_scaling_group_provider(tokens)?
                                );
                        }
                        "updateStatus" => {
                            builder = builder.set_update_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| {
                                    s.to_unescaped().map(|u| {
                                        crate::types::CapacityProviderUpdateStatus::from(u.as_ref())
                                    })
                                })
                                .transpose()?,
                            );
                        }
                        "updateStatusReason" => {
                            builder = builder.set_update_status_reason(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "tags" => {
                            builder = builder
                                .set_tags(crate::protocol_serde::shape_tags::de_tags(tokens)?);
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
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
