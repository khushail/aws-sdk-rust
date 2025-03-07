// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateVPCFromHostedZone`](crate::operation::disassociate_vpc_from_hosted_zone::builders::DisassociateVPCFromHostedZoneFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`hosted_zone_id(impl ::std::convert::Into<String>)`](crate::operation::disassociate_vpc_from_hosted_zone::builders::DisassociateVPCFromHostedZoneFluentBuilder::hosted_zone_id) / [`set_hosted_zone_id(Option<String>)`](crate::operation::disassociate_vpc_from_hosted_zone::builders::DisassociateVPCFromHostedZoneFluentBuilder::set_hosted_zone_id): <p>The ID of the private hosted zone that you want to disassociate a VPC from.</p>
    ///   - [`vpc(Vpc)`](crate::operation::disassociate_vpc_from_hosted_zone::builders::DisassociateVPCFromHostedZoneFluentBuilder::vpc) / [`set_vpc(Option<Vpc>)`](crate::operation::disassociate_vpc_from_hosted_zone::builders::DisassociateVPCFromHostedZoneFluentBuilder::set_vpc): <p>A complex type that contains information about the VPC that you're disassociating from the specified hosted zone.</p>
    ///   - [`comment(impl ::std::convert::Into<String>)`](crate::operation::disassociate_vpc_from_hosted_zone::builders::DisassociateVPCFromHostedZoneFluentBuilder::comment) / [`set_comment(Option<String>)`](crate::operation::disassociate_vpc_from_hosted_zone::builders::DisassociateVPCFromHostedZoneFluentBuilder::set_comment): <p> <i>Optional:</i> A comment about the disassociation request.</p>
    /// - On success, responds with [`DisassociateVpcFromHostedZoneOutput`](crate::operation::disassociate_vpc_from_hosted_zone::DisassociateVpcFromHostedZoneOutput) with field(s):
    ///   - [`change_info(Option<ChangeInfo>)`](crate::operation::disassociate_vpc_from_hosted_zone::DisassociateVpcFromHostedZoneOutput::change_info): <p>A complex type that describes the changes made to the specified private hosted zone.</p>
    /// - On failure, responds with [`SdkError<DisassociateVPCFromHostedZoneError>`](crate::operation::disassociate_vpc_from_hosted_zone::DisassociateVPCFromHostedZoneError)
    pub fn disassociate_vpc_from_hosted_zone(&self) -> crate::operation::disassociate_vpc_from_hosted_zone::builders::DisassociateVPCFromHostedZoneFluentBuilder{
        crate::operation::disassociate_vpc_from_hosted_zone::builders::DisassociateVPCFromHostedZoneFluentBuilder::new(self.handle.clone())
    }
}
