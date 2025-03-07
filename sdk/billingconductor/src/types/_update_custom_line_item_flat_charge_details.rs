// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> A representation of the new charge details that are associated with a flat custom line item. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateCustomLineItemFlatChargeDetails {
    /// <p> The custom line item's new fixed charge value in USD. </p>
    #[doc(hidden)]
    pub charge_value: ::std::option::Option<f64>,
}
impl UpdateCustomLineItemFlatChargeDetails {
    /// <p> The custom line item's new fixed charge value in USD. </p>
    pub fn charge_value(&self) -> ::std::option::Option<f64> {
        self.charge_value
    }
}
impl UpdateCustomLineItemFlatChargeDetails {
    /// Creates a new builder-style object to manufacture [`UpdateCustomLineItemFlatChargeDetails`](crate::types::UpdateCustomLineItemFlatChargeDetails).
    pub fn builder() -> crate::types::builders::UpdateCustomLineItemFlatChargeDetailsBuilder {
        crate::types::builders::UpdateCustomLineItemFlatChargeDetailsBuilder::default()
    }
}

/// A builder for [`UpdateCustomLineItemFlatChargeDetails`](crate::types::UpdateCustomLineItemFlatChargeDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateCustomLineItemFlatChargeDetailsBuilder {
    pub(crate) charge_value: ::std::option::Option<f64>,
}
impl UpdateCustomLineItemFlatChargeDetailsBuilder {
    /// <p> The custom line item's new fixed charge value in USD. </p>
    pub fn charge_value(mut self, input: f64) -> Self {
        self.charge_value = ::std::option::Option::Some(input);
        self
    }
    /// <p> The custom line item's new fixed charge value in USD. </p>
    pub fn set_charge_value(mut self, input: ::std::option::Option<f64>) -> Self {
        self.charge_value = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateCustomLineItemFlatChargeDetails`](crate::types::UpdateCustomLineItemFlatChargeDetails).
    pub fn build(self) -> crate::types::UpdateCustomLineItemFlatChargeDetails {
        crate::types::UpdateCustomLineItemFlatChargeDetails {
            charge_value: self.charge_value,
        }
    }
}
