// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Information about a line item shipment. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ShipmentInformation {
    /// <p> The tracking number of the shipment. </p>
    #[doc(hidden)]
    pub shipment_tracking_number: ::std::option::Option<::std::string::String>,
    /// <p> The carrier of the shipment. </p>
    #[doc(hidden)]
    pub shipment_carrier: ::std::option::Option<crate::types::ShipmentCarrier>,
}
impl ShipmentInformation {
    /// <p> The tracking number of the shipment. </p>
    pub fn shipment_tracking_number(&self) -> ::std::option::Option<&str> {
        self.shipment_tracking_number.as_deref()
    }
    /// <p> The carrier of the shipment. </p>
    pub fn shipment_carrier(&self) -> ::std::option::Option<&crate::types::ShipmentCarrier> {
        self.shipment_carrier.as_ref()
    }
}
impl ShipmentInformation {
    /// Creates a new builder-style object to manufacture [`ShipmentInformation`](crate::types::ShipmentInformation).
    pub fn builder() -> crate::types::builders::ShipmentInformationBuilder {
        crate::types::builders::ShipmentInformationBuilder::default()
    }
}

/// A builder for [`ShipmentInformation`](crate::types::ShipmentInformation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ShipmentInformationBuilder {
    pub(crate) shipment_tracking_number: ::std::option::Option<::std::string::String>,
    pub(crate) shipment_carrier: ::std::option::Option<crate::types::ShipmentCarrier>,
}
impl ShipmentInformationBuilder {
    /// <p> The tracking number of the shipment. </p>
    pub fn shipment_tracking_number(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.shipment_tracking_number = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The tracking number of the shipment. </p>
    pub fn set_shipment_tracking_number(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.shipment_tracking_number = input;
        self
    }
    /// <p> The carrier of the shipment. </p>
    pub fn shipment_carrier(mut self, input: crate::types::ShipmentCarrier) -> Self {
        self.shipment_carrier = ::std::option::Option::Some(input);
        self
    }
    /// <p> The carrier of the shipment. </p>
    pub fn set_shipment_carrier(
        mut self,
        input: ::std::option::Option<crate::types::ShipmentCarrier>,
    ) -> Self {
        self.shipment_carrier = input;
        self
    }
    /// Consumes the builder and constructs a [`ShipmentInformation`](crate::types::ShipmentInformation).
    pub fn build(self) -> crate::types::ShipmentInformation {
        crate::types::ShipmentInformation {
            shipment_tracking_number: self.shipment_tracking_number,
            shipment_carrier: self.shipment_carrier,
        }
    }
}
