// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the ENA Express configuration for the network interface that's attached to the instance.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AttachmentEnaSrdSpecification {
    /// <p>Indicates whether ENA Express is enabled for the network interface that's attached to the instance.</p>
    #[doc(hidden)]
    pub ena_srd_enabled: ::std::option::Option<bool>,
    /// <p>ENA Express configuration for UDP network traffic.</p>
    #[doc(hidden)]
    pub ena_srd_udp_specification:
        ::std::option::Option<crate::types::AttachmentEnaSrdUdpSpecification>,
}
impl AttachmentEnaSrdSpecification {
    /// <p>Indicates whether ENA Express is enabled for the network interface that's attached to the instance.</p>
    pub fn ena_srd_enabled(&self) -> ::std::option::Option<bool> {
        self.ena_srd_enabled
    }
    /// <p>ENA Express configuration for UDP network traffic.</p>
    pub fn ena_srd_udp_specification(
        &self,
    ) -> ::std::option::Option<&crate::types::AttachmentEnaSrdUdpSpecification> {
        self.ena_srd_udp_specification.as_ref()
    }
}
impl AttachmentEnaSrdSpecification {
    /// Creates a new builder-style object to manufacture [`AttachmentEnaSrdSpecification`](crate::types::AttachmentEnaSrdSpecification).
    pub fn builder() -> crate::types::builders::AttachmentEnaSrdSpecificationBuilder {
        crate::types::builders::AttachmentEnaSrdSpecificationBuilder::default()
    }
}

/// A builder for [`AttachmentEnaSrdSpecification`](crate::types::AttachmentEnaSrdSpecification).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AttachmentEnaSrdSpecificationBuilder {
    pub(crate) ena_srd_enabled: ::std::option::Option<bool>,
    pub(crate) ena_srd_udp_specification:
        ::std::option::Option<crate::types::AttachmentEnaSrdUdpSpecification>,
}
impl AttachmentEnaSrdSpecificationBuilder {
    /// <p>Indicates whether ENA Express is enabled for the network interface that's attached to the instance.</p>
    pub fn ena_srd_enabled(mut self, input: bool) -> Self {
        self.ena_srd_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether ENA Express is enabled for the network interface that's attached to the instance.</p>
    pub fn set_ena_srd_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.ena_srd_enabled = input;
        self
    }
    /// <p>ENA Express configuration for UDP network traffic.</p>
    pub fn ena_srd_udp_specification(
        mut self,
        input: crate::types::AttachmentEnaSrdUdpSpecification,
    ) -> Self {
        self.ena_srd_udp_specification = ::std::option::Option::Some(input);
        self
    }
    /// <p>ENA Express configuration for UDP network traffic.</p>
    pub fn set_ena_srd_udp_specification(
        mut self,
        input: ::std::option::Option<crate::types::AttachmentEnaSrdUdpSpecification>,
    ) -> Self {
        self.ena_srd_udp_specification = input;
        self
    }
    /// Consumes the builder and constructs a [`AttachmentEnaSrdSpecification`](crate::types::AttachmentEnaSrdSpecification).
    pub fn build(self) -> crate::types::AttachmentEnaSrdSpecification {
        crate::types::AttachmentEnaSrdSpecification {
            ena_srd_enabled: self.ena_srd_enabled,
            ena_srd_udp_specification: self.ena_srd_udp_specification,
        }
    }
}
