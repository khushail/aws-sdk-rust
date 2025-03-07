// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_environment_managed_action_history_http_error(_response_status: u16, _response_headers: &::http::header::HeaderMap, _response_body: &[u8]) -> std::result::Result<crate::operation::describe_environment_managed_action_history::DescribeEnvironmentManagedActionHistoryOutput, crate::operation::describe_environment_managed_action_history::DescribeEnvironmentManagedActionHistoryError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::describe_environment_managed_action_history::DescribeEnvironmentManagedActionHistoryError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::describe_environment_managed_action_history::DescribeEnvironmentManagedActionHistoryError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ElasticBeanstalkServiceException" => crate::operation::describe_environment_managed_action_history::DescribeEnvironmentManagedActionHistoryError::ElasticBeanstalkServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ElasticBeanstalkServiceExceptionBuilder::default();
                    output = crate::protocol_serde::shape_elastic_beanstalk_service_exception::de_elastic_beanstalk_service_exception_xml_err(_response_body, output).map_err(crate::operation::describe_environment_managed_action_history::DescribeEnvironmentManagedActionHistoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::describe_environment_managed_action_history::DescribeEnvironmentManagedActionHistoryError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_environment_managed_action_history_http_response_with_props(_response_status: u16, _response_headers: &::http::header::HeaderMap, _response_body: &[u8]) -> std::result::Result<crate::operation::describe_environment_managed_action_history::DescribeEnvironmentManagedActionHistoryOutput, crate::operation::describe_environment_managed_action_history::DescribeEnvironmentManagedActionHistoryError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_environment_managed_action_history::builders::DescribeEnvironmentManagedActionHistoryOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_environment_managed_action_history::de_describe_environment_managed_action_history(_response_body, output).map_err(crate::operation::describe_environment_managed_action_history::DescribeEnvironmentManagedActionHistoryError::unhandled)?;
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_environment_managed_action_history(inp: &[u8], mut builder: crate::operation::describe_environment_managed_action_history::builders::DescribeEnvironmentManagedActionHistoryOutputBuilder) -> Result<crate::operation::describe_environment_managed_action_history::builders::DescribeEnvironmentManagedActionHistoryOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeEnvironmentManagedActionHistoryResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeEnvironmentManagedActionHistoryResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("DescribeEnvironmentManagedActionHistoryResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected DescribeEnvironmentManagedActionHistoryResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("ManagedActionHistoryItems") /* ManagedActionHistoryItems com.amazonaws.elasticbeanstalk.synthetic#DescribeEnvironmentManagedActionHistoryOutput$ManagedActionHistoryItems */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_managed_action_history_items::de_managed_action_history_items(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_managed_action_history_items(var_1);
            }
            ,
            s if s.matches("NextToken") /* NextToken com.amazonaws.elasticbeanstalk.synthetic#DescribeEnvironmentManagedActionHistoryOutput$NextToken */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_2);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected DescribeEnvironmentManagedActionHistoryResult tag",
        ));
    };
    Ok(builder)
}
