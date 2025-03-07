// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_custom_document_enrichment_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CustomDocumentEnrichmentConfiguration,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.inline_configurations {
        let mut array_2 = object.key("InlineConfigurations").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_inline_custom_document_enrichment_configuration::ser_inline_custom_document_enrichment_configuration(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.pre_extraction_hook_configuration {
        #[allow(unused_mut)]
        let mut object_6 = object.key("PreExtractionHookConfiguration").start_object();
        crate::protocol_serde::shape_hook_configuration::ser_hook_configuration(
            &mut object_6,
            var_5,
        )?;
        object_6.finish();
    }
    if let Some(var_7) = &input.post_extraction_hook_configuration {
        #[allow(unused_mut)]
        let mut object_8 = object.key("PostExtractionHookConfiguration").start_object();
        crate::protocol_serde::shape_hook_configuration::ser_hook_configuration(
            &mut object_8,
            var_7,
        )?;
        object_8.finish();
    }
    if let Some(var_9) = &input.role_arn {
        object.key("RoleArn").string(var_9.as_str());
    }
    Ok(())
}

pub(crate) fn de_custom_document_enrichment_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::CustomDocumentEnrichmentConfiguration>,
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
            let mut builder =
                crate::types::builders::CustomDocumentEnrichmentConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "InlineConfigurations" => {
                                builder = builder.set_inline_configurations(
                                    crate::protocol_serde::shape_inline_custom_document_enrichment_configuration_list::de_inline_custom_document_enrichment_configuration_list(tokens)?
                                );
                            }
                            "PreExtractionHookConfiguration" => {
                                builder = builder.set_pre_extraction_hook_configuration(
                                    crate::protocol_serde::shape_hook_configuration::de_hook_configuration(tokens)?
                                );
                            }
                            "PostExtractionHookConfiguration" => {
                                builder = builder.set_post_extraction_hook_configuration(
                                    crate::protocol_serde::shape_hook_configuration::de_hook_configuration(tokens)?
                                );
                            }
                            "RoleArn" => {
                                builder = builder.set_role_arn(
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
