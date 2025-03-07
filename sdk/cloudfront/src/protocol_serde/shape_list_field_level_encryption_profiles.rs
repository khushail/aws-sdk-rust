// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_field_level_encryption_profiles_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_field_level_encryption_profiles::ListFieldLevelEncryptionProfilesOutput,
    crate::operation::list_field_level_encryption_profiles::ListFieldLevelEncryptionProfilesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::list_field_level_encryption_profiles::ListFieldLevelEncryptionProfilesError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::list_field_level_encryption_profiles::ListFieldLevelEncryptionProfilesError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidArgument" => crate::operation::list_field_level_encryption_profiles::ListFieldLevelEncryptionProfilesError::InvalidArgument({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidArgumentBuilder::default();
                    output = crate::protocol_serde::shape_invalid_argument::de_invalid_argument_xml_err(_response_body, output).map_err(crate::operation::list_field_level_encryption_profiles::ListFieldLevelEncryptionProfilesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::list_field_level_encryption_profiles::ListFieldLevelEncryptionProfilesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_field_level_encryption_profiles_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_field_level_encryption_profiles::ListFieldLevelEncryptionProfilesOutput,
    crate::operation::list_field_level_encryption_profiles::ListFieldLevelEncryptionProfilesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_field_level_encryption_profiles::builders::ListFieldLevelEncryptionProfilesOutputBuilder::default();
        output = output.set_field_level_encryption_profile_list(
            crate::protocol_serde::shape_list_field_level_encryption_profiles_output::de_field_level_encryption_profile_list_payload(_response_body)?
        );
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
