// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_field_level_encryption_config_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigOutput,
    crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "FieldLevelEncryptionConfigAlreadyExists" => crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::FieldLevelEncryptionConfigAlreadyExists({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::FieldLevelEncryptionConfigAlreadyExistsBuilder::default();
                    output = crate::protocol_serde::shape_field_level_encryption_config_already_exists::de_field_level_encryption_config_already_exists_xml_err(_response_body, output).map_err(crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InconsistentQuantities" => crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::InconsistentQuantities({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InconsistentQuantitiesBuilder::default();
                    output = crate::protocol_serde::shape_inconsistent_quantities::de_inconsistent_quantities_xml_err(_response_body, output).map_err(crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArgument" => crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::InvalidArgument({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidArgumentBuilder::default();
                    output = crate::protocol_serde::shape_invalid_argument::de_invalid_argument_xml_err(_response_body, output).map_err(crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchFieldLevelEncryptionProfile" => crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::NoSuchFieldLevelEncryptionProfile({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchFieldLevelEncryptionProfileBuilder::default();
                    output = crate::protocol_serde::shape_no_such_field_level_encryption_profile::de_no_such_field_level_encryption_profile_xml_err(_response_body, output).map_err(crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "QueryArgProfileEmpty" => crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::QueryArgProfileEmpty({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::QueryArgProfileEmptyBuilder::default();
                    output = crate::protocol_serde::shape_query_arg_profile_empty::de_query_arg_profile_empty_xml_err(_response_body, output).map_err(crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyFieldLevelEncryptionConfigs" => crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::TooManyFieldLevelEncryptionConfigs({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyFieldLevelEncryptionConfigsBuilder::default();
                    output = crate::protocol_serde::shape_too_many_field_level_encryption_configs::de_too_many_field_level_encryption_configs_xml_err(_response_body, output).map_err(crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyFieldLevelEncryptionContentTypeProfiles" => crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::TooManyFieldLevelEncryptionContentTypeProfiles({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyFieldLevelEncryptionContentTypeProfilesBuilder::default();
                    output = crate::protocol_serde::shape_too_many_field_level_encryption_content_type_profiles::de_too_many_field_level_encryption_content_type_profiles_xml_err(_response_body, output).map_err(crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyFieldLevelEncryptionQueryArgProfiles" => crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::TooManyFieldLevelEncryptionQueryArgProfiles({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyFieldLevelEncryptionQueryArgProfilesBuilder::default();
                    output = crate::protocol_serde::shape_too_many_field_level_encryption_query_arg_profiles::de_too_many_field_level_encryption_query_arg_profiles_xml_err(_response_body, output).map_err(crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_field_level_encryption_config_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigOutput,
    crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_field_level_encryption_config::builders::CreateFieldLevelEncryptionConfigOutputBuilder::default();
        output = output.set_e_tag(
            crate::protocol_serde::shape_create_field_level_encryption_config_output::de_e_tag_header(_response_headers)
                                    .map_err(|_|crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::unhandled("Failed to parse ETag from header `ETag"))?
        );
        output = output.set_field_level_encryption(
            crate::protocol_serde::shape_create_field_level_encryption_config_output::de_field_level_encryption_payload(_response_body)?
        );
        output = output.set_location(
            crate::protocol_serde::shape_create_field_level_encryption_config_output::de_location_header(_response_headers)
                                    .map_err(|_|crate::operation::create_field_level_encryption_config::CreateFieldLevelEncryptionConfigError::unhandled("Failed to parse Location from header `Location"))?
        );
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
