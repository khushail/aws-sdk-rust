// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_receive_message_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::receive_message::ReceiveMessageOutput,
    crate::operation::receive_message::ReceiveMessageError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::receive_message::ReceiveMessageError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::receive_message::ReceiveMessageError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "OverLimit" => crate::operation::receive_message::ReceiveMessageError::OverLimit({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::OverLimitBuilder::default();
                output = crate::protocol_serde::shape_over_limit::de_over_limit_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::receive_message::ReceiveMessageError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::receive_message::ReceiveMessageError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_receive_message_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::receive_message::ReceiveMessageOutput,
    crate::operation::receive_message::ReceiveMessageError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::receive_message::builders::ReceiveMessageOutputBuilder::default();
        output = crate::protocol_serde::shape_receive_message::de_receive_message(
            _response_body,
            output,
        )
        .map_err(crate::operation::receive_message::ReceiveMessageError::unhandled)?;
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_receive_message(
    inp: &[u8],
    mut builder: crate::operation::receive_message::builders::ReceiveMessageOutputBuilder,
) -> Result<
    crate::operation::receive_message::builders::ReceiveMessageOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ReceiveMessageResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ReceiveMessageResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("ReceiveMessageResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected ReceiveMessageResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("Message") /* Messages com.amazonaws.sqs.synthetic#ReceiveMessageOutput$Messages */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::vec::Vec<crate::types::Message>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_2 = builder.messages.take().unwrap_or_default();
                            list_2.push(
                                crate::protocol_serde::shape_message::de_message(&mut tag)
                                ?
                            );
                            list_2
                        })
                        ?
                    )
                ;
                builder = builder.set_messages(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected ReceiveMessageResult tag",
        ));
    };
    Ok(builder)
}
