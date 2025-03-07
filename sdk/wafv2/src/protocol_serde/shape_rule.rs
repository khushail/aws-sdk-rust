// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_rule(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Rule,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    {
        object.key("Priority").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.priority).into()),
        );
    }
    if let Some(var_2) = &input.statement {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Statement").start_object();
        crate::protocol_serde::shape_statement::ser_statement(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.action {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Action").start_object();
        crate::protocol_serde::shape_rule_action::ser_rule_action(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.override_action {
        #[allow(unused_mut)]
        let mut object_7 = object.key("OverrideAction").start_object();
        crate::protocol_serde::shape_override_action::ser_override_action(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.rule_labels {
        let mut array_9 = object.key("RuleLabels").start_array();
        for item_10 in var_8 {
            {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_label::ser_label(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.visibility_config {
        #[allow(unused_mut)]
        let mut object_13 = object.key("VisibilityConfig").start_object();
        crate::protocol_serde::shape_visibility_config::ser_visibility_config(
            &mut object_13,
            var_12,
        )?;
        object_13.finish();
    }
    if let Some(var_14) = &input.captcha_config {
        #[allow(unused_mut)]
        let mut object_15 = object.key("CaptchaConfig").start_object();
        crate::protocol_serde::shape_captcha_config::ser_captcha_config(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.challenge_config {
        #[allow(unused_mut)]
        let mut object_17 = object.key("ChallengeConfig").start_object();
        crate::protocol_serde::shape_challenge_config::ser_challenge_config(
            &mut object_17,
            var_16,
        )?;
        object_17.finish();
    }
    Ok(())
}

pub(crate) fn de_rule<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::Rule>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
            let mut builder = crate::types::builders::RuleBuilder::default();
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
                            "Priority" => {
                                builder = builder.set_priority(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "Statement" => {
                                builder = builder.set_statement(
                                    crate::protocol_serde::shape_statement::de_statement(tokens)?,
                                );
                            }
                            "Action" => {
                                builder = builder.set_action(
                                    crate::protocol_serde::shape_rule_action::de_rule_action(
                                        tokens,
                                    )?,
                                );
                            }
                            "OverrideAction" => {
                                builder = builder.set_override_action(
                                    crate::protocol_serde::shape_override_action::de_override_action(tokens)?
                                );
                            }
                            "RuleLabels" => {
                                builder = builder.set_rule_labels(
                                    crate::protocol_serde::shape_labels::de_labels(tokens)?,
                                );
                            }
                            "VisibilityConfig" => {
                                builder = builder.set_visibility_config(
                                    crate::protocol_serde::shape_visibility_config::de_visibility_config(tokens)?
                                );
                            }
                            "CaptchaConfig" => {
                                builder = builder.set_captcha_config(
                                    crate::protocol_serde::shape_captcha_config::de_captcha_config(
                                        tokens,
                                    )?,
                                );
                            }
                            "ChallengeConfig" => {
                                builder = builder.set_challenge_config(
                                    crate::protocol_serde::shape_challenge_config::de_challenge_config(tokens)?
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
