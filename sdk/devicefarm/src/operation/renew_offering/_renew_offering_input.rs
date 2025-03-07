// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A request that represents an offering renewal.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RenewOfferingInput {
    /// <p>The ID of a request to renew an offering.</p>
    #[doc(hidden)]
    pub offering_id: ::std::option::Option<::std::string::String>,
    /// <p>The quantity requested in an offering renewal.</p>
    #[doc(hidden)]
    pub quantity: ::std::option::Option<i32>,
}
impl RenewOfferingInput {
    /// <p>The ID of a request to renew an offering.</p>
    pub fn offering_id(&self) -> ::std::option::Option<&str> {
        self.offering_id.as_deref()
    }
    /// <p>The quantity requested in an offering renewal.</p>
    pub fn quantity(&self) -> ::std::option::Option<i32> {
        self.quantity
    }
}
impl RenewOfferingInput {
    /// Creates a new builder-style object to manufacture [`RenewOfferingInput`](crate::operation::renew_offering::RenewOfferingInput).
    pub fn builder() -> crate::operation::renew_offering::builders::RenewOfferingInputBuilder {
        crate::operation::renew_offering::builders::RenewOfferingInputBuilder::default()
    }
}

/// A builder for [`RenewOfferingInput`](crate::operation::renew_offering::RenewOfferingInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RenewOfferingInputBuilder {
    pub(crate) offering_id: ::std::option::Option<::std::string::String>,
    pub(crate) quantity: ::std::option::Option<i32>,
}
impl RenewOfferingInputBuilder {
    /// <p>The ID of a request to renew an offering.</p>
    pub fn offering_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.offering_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of a request to renew an offering.</p>
    pub fn set_offering_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.offering_id = input;
        self
    }
    /// <p>The quantity requested in an offering renewal.</p>
    pub fn quantity(mut self, input: i32) -> Self {
        self.quantity = ::std::option::Option::Some(input);
        self
    }
    /// <p>The quantity requested in an offering renewal.</p>
    pub fn set_quantity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.quantity = input;
        self
    }
    /// Consumes the builder and constructs a [`RenewOfferingInput`](crate::operation::renew_offering::RenewOfferingInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::renew_offering::RenewOfferingInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::renew_offering::RenewOfferingInput {
            offering_id: self.offering_id,
            quantity: self.quantity,
        })
    }
}
