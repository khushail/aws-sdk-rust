// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_route_table_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_route_table::DeleteRouteTableOutput,
    crate::operation::delete_route_table::DeleteRouteTableError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::delete_route_table::DeleteRouteTableError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::delete_route_table::DeleteRouteTableError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_route_table_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_route_table::DeleteRouteTableOutput,
    crate::operation::delete_route_table::DeleteRouteTableError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::delete_route_table::builders::DeleteRouteTableOutputBuilder::default(
            );
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
