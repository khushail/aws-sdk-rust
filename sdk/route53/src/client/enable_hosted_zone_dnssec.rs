// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`EnableHostedZoneDNSSEC`](crate::operation::enable_hosted_zone_dnssec::builders::EnableHostedZoneDNSSECFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`hosted_zone_id(impl ::std::convert::Into<String>)`](crate::operation::enable_hosted_zone_dnssec::builders::EnableHostedZoneDNSSECFluentBuilder::hosted_zone_id) / [`set_hosted_zone_id(Option<String>)`](crate::operation::enable_hosted_zone_dnssec::builders::EnableHostedZoneDNSSECFluentBuilder::set_hosted_zone_id): <p>A unique string used to identify a hosted zone.</p>
    /// - On success, responds with [`EnableHostedZoneDnssecOutput`](crate::operation::enable_hosted_zone_dnssec::EnableHostedZoneDnssecOutput) with field(s):
    ///   - [`change_info(Option<ChangeInfo>)`](crate::operation::enable_hosted_zone_dnssec::EnableHostedZoneDnssecOutput::change_info): <p>A complex type that describes change information about changes made to your hosted zone.</p>
    /// - On failure, responds with [`SdkError<EnableHostedZoneDNSSECError>`](crate::operation::enable_hosted_zone_dnssec::EnableHostedZoneDNSSECError)
    pub fn enable_hosted_zone_dnssec(
        &self,
    ) -> crate::operation::enable_hosted_zone_dnssec::builders::EnableHostedZoneDNSSECFluentBuilder
    {
        crate::operation::enable_hosted_zone_dnssec::builders::EnableHostedZoneDNSSECFluentBuilder::new(self.handle.clone())
    }
}
