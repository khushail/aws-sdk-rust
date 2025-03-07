// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_filter_operation_selected_fields_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::FilterOperationSelectedFieldsConfiguration,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.selected_fields {
        let mut array_2 = object.key("SelectedFields").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.selected_field_options {
        object.key("SelectedFieldOptions").string(var_4.as_str());
    }
    if let Some(var_5) = &input.selected_columns {
        let mut array_6 = object.key("SelectedColumns").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_column_identifier::ser_column_identifier(
                    &mut object_8,
                    item_7,
                )?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    Ok(())
}

pub(crate) fn de_filter_operation_selected_fields_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::FilterOperationSelectedFieldsConfiguration>,
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
                crate::types::builders::FilterOperationSelectedFieldsConfigurationBuilder::default(
                );
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "SelectedFields" => {
                                builder = builder.set_selected_fields(
                                    crate::protocol_serde::shape_selected_field_list::de_selected_field_list(tokens)?
                                );
                            }
                            "SelectedFieldOptions" => {
                                builder = builder.set_selected_field_options(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::SelectedFieldOptions::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "SelectedColumns" => {
                                builder = builder.set_selected_columns(
                                    crate::protocol_serde::shape_custom_action_column_list::de_custom_action_column_list(tokens)?
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
