// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides details in case one of the following operations fails due to an error related to KMS: <code>CreateDeliveryStream</code>, <code>DeleteDeliveryStream</code>, <code>StartDeliveryStreamEncryption</code>, <code>StopDeliveryStreamEncryption</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FailureDescription {
    /// <p>The type of error that caused the failure.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::DeliveryStreamFailureType>,
    /// <p>A message providing details about the error that caused the failure.</p>
    #[doc(hidden)]
    pub details: ::std::option::Option<::std::string::String>,
}
impl FailureDescription {
    /// <p>The type of error that caused the failure.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::DeliveryStreamFailureType> {
        self.r#type.as_ref()
    }
    /// <p>A message providing details about the error that caused the failure.</p>
    pub fn details(&self) -> ::std::option::Option<&str> {
        self.details.as_deref()
    }
}
impl FailureDescription {
    /// Creates a new builder-style object to manufacture [`FailureDescription`](crate::types::FailureDescription).
    pub fn builder() -> crate::types::builders::FailureDescriptionBuilder {
        crate::types::builders::FailureDescriptionBuilder::default()
    }
}

/// A builder for [`FailureDescription`](crate::types::FailureDescription).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FailureDescriptionBuilder {
    pub(crate) r#type: ::std::option::Option<crate::types::DeliveryStreamFailureType>,
    pub(crate) details: ::std::option::Option<::std::string::String>,
}
impl FailureDescriptionBuilder {
    /// <p>The type of error that caused the failure.</p>
    pub fn r#type(mut self, input: crate::types::DeliveryStreamFailureType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of error that caused the failure.</p>
    pub fn set_type(
        mut self,
        input: ::std::option::Option<crate::types::DeliveryStreamFailureType>,
    ) -> Self {
        self.r#type = input;
        self
    }
    /// <p>A message providing details about the error that caused the failure.</p>
    pub fn details(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.details = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A message providing details about the error that caused the failure.</p>
    pub fn set_details(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.details = input;
        self
    }
    /// Consumes the builder and constructs a [`FailureDescription`](crate::types::FailureDescription).
    pub fn build(self) -> crate::types::FailureDescription {
        crate::types::FailureDescription {
            r#type: self.r#type,
            details: self.details,
        }
    }
}
