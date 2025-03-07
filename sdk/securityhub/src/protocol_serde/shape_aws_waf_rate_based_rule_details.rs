// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_waf_rate_based_rule_details(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AwsWafRateBasedRuleDetails,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.metric_name {
        object.key("MetricName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.rate_key {
        object.key("RateKey").string(var_3.as_str());
    }
    if input.rate_limit != 0 {
        object.key("RateLimit").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.rate_limit).into()),
        );
    }
    if let Some(var_4) = &input.rule_id {
        object.key("RuleId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.match_predicates {
        let mut array_6 = object.key("MatchPredicates").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_aws_waf_rate_based_rule_match_predicate::ser_aws_waf_rate_based_rule_match_predicate(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    Ok(())
}

pub(crate) fn de_aws_waf_rate_based_rule_details<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::AwsWafRateBasedRuleDetails>,
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
            let mut builder = crate::types::builders::AwsWafRateBasedRuleDetailsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "MetricName" => {
                                builder = builder.set_metric_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Name" => {
                                builder = builder.set_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "RateKey" => {
                                builder = builder.set_rate_key(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "RateLimit" => {
                                builder = builder.set_rate_limit(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i64::try_from)
                                    .transpose()?,
                                );
                            }
                            "RuleId" => {
                                builder = builder.set_rule_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "MatchPredicates" => {
                                builder = builder.set_match_predicates(
                                    crate::protocol_serde::shape_aws_waf_rate_based_rule_match_predicate_list::de_aws_waf_rate_based_rule_match_predicate_list(tokens)?
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
