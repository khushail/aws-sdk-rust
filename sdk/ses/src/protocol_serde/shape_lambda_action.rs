// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_lambda_action(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::LambdaAction,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TopicArn");
    if let Some(var_2) = &input.topic_arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("FunctionArn");
    if let Some(var_4) = &input.function_arn {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("InvocationType");
    if let Some(var_6) = &input.invocation_type {
        scope_5.string(var_6.as_str());
    }
    Ok(())
}

pub fn de_lambda_action(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::LambdaAction, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LambdaAction::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("TopicArn") /* TopicArn com.amazonaws.ses#LambdaAction$TopicArn */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_topic_arn(var_7);
            }
            ,
            s if s.matches("FunctionArn") /* FunctionArn com.amazonaws.ses#LambdaAction$FunctionArn */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_function_arn(var_8);
            }
            ,
            s if s.matches("InvocationType") /* InvocationType com.amazonaws.ses#LambdaAction$InvocationType */ =>  {
                let var_9 =
                    Some(
                        Result::<crate::types::InvocationType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InvocationType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_invocation_type(var_9);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
