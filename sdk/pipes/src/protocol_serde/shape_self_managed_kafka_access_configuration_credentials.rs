// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_self_managed_kafka_access_configuration_credentials(
    object_10: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SelfManagedKafkaAccessConfigurationCredentials,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::types::SelfManagedKafkaAccessConfigurationCredentials::BasicAuth(inner) => {
            object_10.key("BasicAuth").string(inner.as_str());
        }
        crate::types::SelfManagedKafkaAccessConfigurationCredentials::SaslScram512Auth(inner) => {
            object_10.key("SaslScram512Auth").string(inner.as_str());
        }
        crate::types::SelfManagedKafkaAccessConfigurationCredentials::SaslScram256Auth(inner) => {
            object_10.key("SaslScram256Auth").string(inner.as_str());
        }
        crate::types::SelfManagedKafkaAccessConfigurationCredentials::ClientCertificateTlsAuth(
            inner,
        ) => {
            object_10
                .key("ClientCertificateTlsAuth")
                .string(inner.as_str());
        }
        crate::types::SelfManagedKafkaAccessConfigurationCredentials::Unknown => {
            return Err(
                ::aws_smithy_http::operation::error::SerializationError::unknown_variant(
                    "SelfManagedKafkaAccessConfigurationCredentials",
                ),
            )
        }
    }
    Ok(())
}

pub(crate) fn de_self_managed_kafka_access_configuration_credentials<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::SelfManagedKafkaAccessConfigurationCredentials>,
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
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => loop {
            match tokens.next().transpose()? {
                Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                    if variant.is_some() {
                        return Err(
                            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                "encountered mixed variants in union",
                            ),
                        );
                    }
                    variant = match key.to_unescaped()?.as_ref() {
                            "BasicAuth" => {
                                Some(crate::types::SelfManagedKafkaAccessConfigurationCredentials::BasicAuth(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                    .unwrap_or_default()
                                ))
                            }
                            "SaslScram512Auth" => {
                                Some(crate::types::SelfManagedKafkaAccessConfigurationCredentials::SaslScram512Auth(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                    .unwrap_or_default()
                                ))
                            }
                            "SaslScram256Auth" => {
                                Some(crate::types::SelfManagedKafkaAccessConfigurationCredentials::SaslScram256Auth(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                    .unwrap_or_default()
                                ))
                            }
                            "ClientCertificateTlsAuth" => {
                                Some(crate::types::SelfManagedKafkaAccessConfigurationCredentials::ClientCertificateTlsAuth(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                    .unwrap_or_default()
                                ))
                            }
                            _ => {
                                                                      ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                                                                      Some(crate::types::SelfManagedKafkaAccessConfigurationCredentials::Unknown)
                                                                    }
                        };
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
        },
        _ => {
            return Err(
                ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                    "expected start object or null",
                ),
            )
        }
    }
    Ok(variant)
}
