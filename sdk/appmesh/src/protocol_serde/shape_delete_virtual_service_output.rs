// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_virtual_service_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<crate::types::VirtualServiceData>,
    crate::operation::delete_virtual_service::DeleteVirtualServiceError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_virtual_service_data::de_virtual_service_data_payload(body)
                .map_err(
                    crate::operation::delete_virtual_service::DeleteVirtualServiceError::unhandled,
                )
        })
        .transpose()
}
