// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_geo_json_payload_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<::aws_smithy_types::Blob>,
    crate::operation::get_position_estimate::GetPositionEstimateError,
> {
    (!body.is_empty())
        .then(|| Ok(::aws_smithy_types::Blob::new(body)))
        .transpose()
}
