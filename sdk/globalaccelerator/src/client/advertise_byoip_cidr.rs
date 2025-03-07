// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AdvertiseByoipCidr`](crate::operation::advertise_byoip_cidr::builders::AdvertiseByoipCidrFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cidr(impl ::std::convert::Into<String>)`](crate::operation::advertise_byoip_cidr::builders::AdvertiseByoipCidrFluentBuilder::cidr) / [`set_cidr(Option<String>)`](crate::operation::advertise_byoip_cidr::builders::AdvertiseByoipCidrFluentBuilder::set_cidr): <p>The address range, in CIDR notation. This must be the exact range that you provisioned. You can't advertise only a portion of the provisioned range.</p>
    /// - On success, responds with [`AdvertiseByoipCidrOutput`](crate::operation::advertise_byoip_cidr::AdvertiseByoipCidrOutput) with field(s):
    ///   - [`byoip_cidr(Option<ByoipCidr>)`](crate::operation::advertise_byoip_cidr::AdvertiseByoipCidrOutput::byoip_cidr): <p>Information about the address range.</p>
    /// - On failure, responds with [`SdkError<AdvertiseByoipCidrError>`](crate::operation::advertise_byoip_cidr::AdvertiseByoipCidrError)
    pub fn advertise_byoip_cidr(
        &self,
    ) -> crate::operation::advertise_byoip_cidr::builders::AdvertiseByoipCidrFluentBuilder {
        crate::operation::advertise_byoip_cidr::builders::AdvertiseByoipCidrFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
