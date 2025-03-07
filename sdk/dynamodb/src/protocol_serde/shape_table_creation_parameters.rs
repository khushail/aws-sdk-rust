// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_table_creation_parameters(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TableCreationParameters,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.table_name {
        object.key("TableName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.attribute_definitions {
        let mut array_3 = object.key("AttributeDefinitions").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_attribute_definition::ser_attribute_definition(
                    &mut object_5,
                    item_4,
                )?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.key_schema {
        let mut array_7 = object.key("KeySchema").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_key_schema_element::ser_key_schema_element(
                    &mut object_9,
                    item_8,
                )?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.billing_mode {
        object.key("BillingMode").string(var_10.as_str());
    }
    if let Some(var_11) = &input.provisioned_throughput {
        #[allow(unused_mut)]
        let mut object_12 = object.key("ProvisionedThroughput").start_object();
        crate::protocol_serde::shape_provisioned_throughput::ser_provisioned_throughput(
            &mut object_12,
            var_11,
        )?;
        object_12.finish();
    }
    if let Some(var_13) = &input.sse_specification {
        #[allow(unused_mut)]
        let mut object_14 = object.key("SSESpecification").start_object();
        crate::protocol_serde::shape_sse_specification::ser_sse_specification(
            &mut object_14,
            var_13,
        )?;
        object_14.finish();
    }
    if let Some(var_15) = &input.global_secondary_indexes {
        let mut array_16 = object.key("GlobalSecondaryIndexes").start_array();
        for item_17 in var_15 {
            {
                #[allow(unused_mut)]
                let mut object_18 = array_16.value().start_object();
                crate::protocol_serde::shape_global_secondary_index::ser_global_secondary_index(
                    &mut object_18,
                    item_17,
                )?;
                object_18.finish();
            }
        }
        array_16.finish();
    }
    Ok(())
}

pub(crate) fn de_table_creation_parameters<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::TableCreationParameters>,
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
            let mut builder = crate::types::builders::TableCreationParametersBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "TableName" => {
                                builder = builder.set_table_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "AttributeDefinitions" => {
                                builder = builder.set_attribute_definitions(
                                    crate::protocol_serde::shape_attribute_definitions::de_attribute_definitions(tokens)?
                                );
                            }
                            "KeySchema" => {
                                builder = builder.set_key_schema(
                                    crate::protocol_serde::shape_key_schema::de_key_schema(tokens)?,
                                );
                            }
                            "BillingMode" => {
                                builder = builder.set_billing_mode(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::BillingMode::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "ProvisionedThroughput" => {
                                builder = builder.set_provisioned_throughput(
                                    crate::protocol_serde::shape_provisioned_throughput::de_provisioned_throughput(tokens)?
                                );
                            }
                            "SSESpecification" => {
                                builder = builder.set_sse_specification(
                                    crate::protocol_serde::shape_sse_specification::de_sse_specification(tokens)?
                                );
                            }
                            "GlobalSecondaryIndexes" => {
                                builder = builder.set_global_secondary_indexes(
                                    crate::protocol_serde::shape_global_secondary_index_list::de_global_secondary_index_list(tokens)?
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
