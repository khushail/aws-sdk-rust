// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_segment_response_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<crate::types::SegmentResponse>,
    crate::operation::delete_segment::DeleteSegmentError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_segment_response::de_segment_response_payload(body)
                .map_err(crate::operation::delete_segment::DeleteSegmentError::unhandled)
        })
        .transpose()
}
