// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_video_selector_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::VideoSelectorSettings,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.video_selector_pid {
        #[allow(unused_mut)]
        let mut object_2 = object.key("videoSelectorPid").start_object();
        crate::protocol_serde::shape_video_selector_pid::ser_video_selector_pid(
            &mut object_2,
            var_1,
        )?;
        object_2.finish();
    }
    if let Some(var_3) = &input.video_selector_program_id {
        #[allow(unused_mut)]
        let mut object_4 = object.key("videoSelectorProgramId").start_object();
        crate::protocol_serde::shape_video_selector_program_id::ser_video_selector_program_id(
            &mut object_4,
            var_3,
        )?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_video_selector_settings<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::VideoSelectorSettings>,
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
            let mut builder = crate::types::builders::VideoSelectorSettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "videoSelectorPid" => {
                                builder = builder.set_video_selector_pid(
                                    crate::protocol_serde::shape_video_selector_pid::de_video_selector_pid(tokens)?
                                );
                            }
                            "videoSelectorProgramId" => {
                                builder = builder.set_video_selector_program_id(
                                    crate::protocol_serde::shape_video_selector_program_id::de_video_selector_program_id(tokens)?
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
