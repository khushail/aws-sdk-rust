// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_ec2_launch_template_data_block_device_mapping_set_ebs_details(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AwsEc2LaunchTemplateDataBlockDeviceMappingSetEbsDetails,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if input.delete_on_termination {
        object
            .key("DeleteOnTermination")
            .boolean(input.delete_on_termination);
    }
    if input.encrypted {
        object.key("Encrypted").boolean(input.encrypted);
    }
    if input.iops != 0 {
        object.key("Iops").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.iops).into()),
        );
    }
    if let Some(var_1) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.snapshot_id {
        object.key("SnapshotId").string(var_2.as_str());
    }
    if input.throughput != 0 {
        object.key("Throughput").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.throughput).into()),
        );
    }
    if input.volume_size != 0 {
        object.key("VolumeSize").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.volume_size).into()),
        );
    }
    if let Some(var_3) = &input.volume_type {
        object.key("VolumeType").string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_ec2_launch_template_data_block_device_mapping_set_ebs_details<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::AwsEc2LaunchTemplateDataBlockDeviceMappingSetEbsDetails>,
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
            let mut builder = crate::types::builders::AwsEc2LaunchTemplateDataBlockDeviceMappingSetEbsDetailsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "DeleteOnTermination" => {
                                builder = builder.set_delete_on_termination(
                                    ::aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "Encrypted" => {
                                builder = builder.set_encrypted(
                                    ::aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "Iops" => {
                                builder = builder.set_iops(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "KmsKeyId" => {
                                builder = builder.set_kms_key_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "SnapshotId" => {
                                builder = builder.set_snapshot_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Throughput" => {
                                builder = builder.set_throughput(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "VolumeSize" => {
                                builder = builder.set_volume_size(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "VolumeType" => {
                                builder = builder.set_volume_type(
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
