// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_topic_column(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TopicColumn,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.column_name {
        object.key("ColumnName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.column_friendly_name {
        object.key("ColumnFriendlyName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.column_description {
        object.key("ColumnDescription").string(var_3.as_str());
    }
    if let Some(var_4) = &input.column_synonyms {
        let mut array_5 = object.key("ColumnSynonyms").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.column_data_role {
        object.key("ColumnDataRole").string(var_7.as_str());
    }
    if let Some(var_8) = &input.aggregation {
        object.key("Aggregation").string(var_8.as_str());
    }
    if input.is_included_in_topic {
        object
            .key("IsIncludedInTopic")
            .boolean(input.is_included_in_topic);
    }
    if let Some(var_9) = &input.disable_indexing {
        object.key("DisableIndexing").boolean(*var_9);
    }
    if let Some(var_10) = &input.comparative_order {
        #[allow(unused_mut)]
        let mut object_11 = object.key("ComparativeOrder").start_object();
        crate::protocol_serde::shape_comparative_order::ser_comparative_order(
            &mut object_11,
            var_10,
        )?;
        object_11.finish();
    }
    if let Some(var_12) = &input.semantic_type {
        #[allow(unused_mut)]
        let mut object_13 = object.key("SemanticType").start_object();
        crate::protocol_serde::shape_semantic_type::ser_semantic_type(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.time_granularity {
        object.key("TimeGranularity").string(var_14.as_str());
    }
    if let Some(var_15) = &input.allowed_aggregations {
        let mut array_16 = object.key("AllowedAggregations").start_array();
        for item_17 in var_15 {
            {
                array_16.value().string(item_17.as_str());
            }
        }
        array_16.finish();
    }
    if let Some(var_18) = &input.not_allowed_aggregations {
        let mut array_19 = object.key("NotAllowedAggregations").start_array();
        for item_20 in var_18 {
            {
                array_19.value().string(item_20.as_str());
            }
        }
        array_19.finish();
    }
    if let Some(var_21) = &input.default_formatting {
        #[allow(unused_mut)]
        let mut object_22 = object.key("DefaultFormatting").start_object();
        crate::protocol_serde::shape_default_formatting::ser_default_formatting(
            &mut object_22,
            var_21,
        )?;
        object_22.finish();
    }
    if input.never_aggregate_in_filter {
        object
            .key("NeverAggregateInFilter")
            .boolean(input.never_aggregate_in_filter);
    }
    if let Some(var_23) = &input.cell_value_synonyms {
        let mut array_24 = object.key("CellValueSynonyms").start_array();
        for item_25 in var_23 {
            {
                #[allow(unused_mut)]
                let mut object_26 = array_24.value().start_object();
                crate::protocol_serde::shape_cell_value_synonym::ser_cell_value_synonym(
                    &mut object_26,
                    item_25,
                )?;
                object_26.finish();
            }
        }
        array_24.finish();
    }
    Ok(())
}

pub(crate) fn de_topic_column<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::TopicColumn>,
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
            let mut builder = crate::types::builders::TopicColumnBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ColumnName" => {
                                builder = builder.set_column_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "ColumnFriendlyName" => {
                                builder = builder.set_column_friendly_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "ColumnDescription" => {
                                builder = builder.set_column_description(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "ColumnSynonyms" => {
                                builder = builder.set_column_synonyms(
                                    crate::protocol_serde::shape_synonyms::de_synonyms(tokens)?,
                                );
                            }
                            "ColumnDataRole" => {
                                builder = builder.set_column_data_role(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::ColumnDataRole::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "Aggregation" => {
                                builder = builder.set_aggregation(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::DefaultAggregation::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "IsIncludedInTopic" => {
                                builder = builder.set_is_included_in_topic(
                                    ::aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "DisableIndexing" => {
                                builder = builder.set_disable_indexing(
                                    ::aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "ComparativeOrder" => {
                                builder = builder.set_comparative_order(
                                    crate::protocol_serde::shape_comparative_order::de_comparative_order(tokens)?
                                );
                            }
                            "SemanticType" => {
                                builder = builder.set_semantic_type(
                                    crate::protocol_serde::shape_semantic_type::de_semantic_type(
                                        tokens,
                                    )?,
                                );
                            }
                            "TimeGranularity" => {
                                builder = builder.set_time_granularity(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::TopicTimeGranularity::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "AllowedAggregations" => {
                                builder = builder.set_allowed_aggregations(
                                    crate::protocol_serde::shape_author_specified_aggregations::de_author_specified_aggregations(tokens)?
                                );
                            }
                            "NotAllowedAggregations" => {
                                builder = builder.set_not_allowed_aggregations(
                                    crate::protocol_serde::shape_author_specified_aggregations::de_author_specified_aggregations(tokens)?
                                );
                            }
                            "DefaultFormatting" => {
                                builder = builder.set_default_formatting(
                                    crate::protocol_serde::shape_default_formatting::de_default_formatting(tokens)?
                                );
                            }
                            "NeverAggregateInFilter" => {
                                builder = builder.set_never_aggregate_in_filter(
                                    ::aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "CellValueSynonyms" => {
                                builder = builder.set_cell_value_synonyms(
                                    crate::protocol_serde::shape_cell_value_synonyms::de_cell_value_synonyms(tokens)?
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
