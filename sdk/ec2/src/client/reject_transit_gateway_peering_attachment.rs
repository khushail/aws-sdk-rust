// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RejectTransitGatewayPeeringAttachment`](crate::operation::reject_transit_gateway_peering_attachment::builders::RejectTransitGatewayPeeringAttachmentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_attachment_id(impl ::std::convert::Into<String>)`](crate::operation::reject_transit_gateway_peering_attachment::builders::RejectTransitGatewayPeeringAttachmentFluentBuilder::transit_gateway_attachment_id) / [`set_transit_gateway_attachment_id(Option<String>)`](crate::operation::reject_transit_gateway_peering_attachment::builders::RejectTransitGatewayPeeringAttachmentFluentBuilder::set_transit_gateway_attachment_id): <p>The ID of the transit gateway peering attachment.</p>
    ///   - [`dry_run(bool)`](crate::operation::reject_transit_gateway_peering_attachment::builders::RejectTransitGatewayPeeringAttachmentFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::reject_transit_gateway_peering_attachment::builders::RejectTransitGatewayPeeringAttachmentFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`RejectTransitGatewayPeeringAttachmentOutput`](crate::operation::reject_transit_gateway_peering_attachment::RejectTransitGatewayPeeringAttachmentOutput) with field(s):
    ///   - [`transit_gateway_peering_attachment(Option<TransitGatewayPeeringAttachment>)`](crate::operation::reject_transit_gateway_peering_attachment::RejectTransitGatewayPeeringAttachmentOutput::transit_gateway_peering_attachment): <p>The transit gateway peering attachment.</p>
    /// - On failure, responds with [`SdkError<RejectTransitGatewayPeeringAttachmentError>`](crate::operation::reject_transit_gateway_peering_attachment::RejectTransitGatewayPeeringAttachmentError)
    pub fn reject_transit_gateway_peering_attachment(&self) -> crate::operation::reject_transit_gateway_peering_attachment::builders::RejectTransitGatewayPeeringAttachmentFluentBuilder{
        crate::operation::reject_transit_gateway_peering_attachment::builders::RejectTransitGatewayPeeringAttachmentFluentBuilder::new(self.handle.clone())
    }
}
