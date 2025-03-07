// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_automation_execution<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::AutomationExecution>,
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
            let mut builder = crate::types::builders::AutomationExecutionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "AutomationExecutionId" => {
                                builder = builder.set_automation_execution_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "DocumentName" => {
                                builder = builder.set_document_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "DocumentVersion" => {
                                builder = builder.set_document_version(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "ExecutionStartTime" => {
                                builder = builder.set_execution_start_time(
                                    ::aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), ::aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "ExecutionEndTime" => {
                                builder = builder.set_execution_end_time(
                                    ::aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), ::aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "AutomationExecutionStatus" => {
                                builder = builder.set_automation_execution_status(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::AutomationExecutionStatus::from(
                                                u.as_ref(),
                                            )
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "StepExecutions" => {
                                builder = builder.set_step_executions(
                                    crate::protocol_serde::shape_step_execution_list::de_step_execution_list(tokens)?
                                );
                            }
                            "StepExecutionsTruncated" => {
                                builder = builder.set_step_executions_truncated(
                                    ::aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "Parameters" => {
                                builder = builder.set_parameters(
                                    crate::protocol_serde::shape_automation_parameter_map::de_automation_parameter_map(tokens)?
                                );
                            }
                            "Outputs" => {
                                builder = builder.set_outputs(
                                    crate::protocol_serde::shape_automation_parameter_map::de_automation_parameter_map(tokens)?
                                );
                            }
                            "FailureMessage" => {
                                builder = builder.set_failure_message(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Mode" => {
                                builder = builder.set_mode(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::ExecutionMode::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "ParentAutomationExecutionId" => {
                                builder = builder.set_parent_automation_execution_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "ExecutedBy" => {
                                builder = builder.set_executed_by(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "CurrentStepName" => {
                                builder = builder.set_current_step_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "CurrentAction" => {
                                builder = builder.set_current_action(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "TargetParameterName" => {
                                builder = builder.set_target_parameter_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Targets" => {
                                builder = builder.set_targets(
                                    crate::protocol_serde::shape_targets::de_targets(tokens)?,
                                );
                            }
                            "TargetMaps" => {
                                builder = builder.set_target_maps(
                                    crate::protocol_serde::shape_target_maps::de_target_maps(
                                        tokens,
                                    )?,
                                );
                            }
                            "ResolvedTargets" => {
                                builder = builder.set_resolved_targets(
                                    crate::protocol_serde::shape_resolved_targets::de_resolved_targets(tokens)?
                                );
                            }
                            "MaxConcurrency" => {
                                builder = builder.set_max_concurrency(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "MaxErrors" => {
                                builder = builder.set_max_errors(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Target" => {
                                builder = builder.set_target(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "TargetLocations" => {
                                builder = builder.set_target_locations(
                                    crate::protocol_serde::shape_target_locations::de_target_locations(tokens)?
                                );
                            }
                            "ProgressCounters" => {
                                builder = builder.set_progress_counters(
                                    crate::protocol_serde::shape_progress_counters::de_progress_counters(tokens)?
                                );
                            }
                            "AlarmConfiguration" => {
                                builder = builder.set_alarm_configuration(
                                    crate::protocol_serde::shape_alarm_configuration::de_alarm_configuration(tokens)?
                                );
                            }
                            "TriggeredAlarms" => {
                                builder = builder.set_triggered_alarms(
                                    crate::protocol_serde::shape_alarm_state_information_list::de_alarm_state_information_list(tokens)?
                                );
                            }
                            "AutomationSubtype" => {
                                builder = builder.set_automation_subtype(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::AutomationSubtype::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "ScheduledTime" => {
                                builder = builder.set_scheduled_time(
                                    ::aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), ::aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "Runbooks" => {
                                builder = builder.set_runbooks(
                                    crate::protocol_serde::shape_runbooks::de_runbooks(tokens)?,
                                );
                            }
                            "OpsItemId" => {
                                builder = builder.set_ops_item_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "AssociationId" => {
                                builder = builder.set_association_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "ChangeRequestName" => {
                                builder = builder.set_change_request_name(
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
