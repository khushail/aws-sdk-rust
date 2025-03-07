// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> A representation of a custom line item. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CustomLineItemListElement {
    /// <p> The Amazon Resource Names (ARNs) for custom line items. </p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p> The custom line item's name. </p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p> A <code>ListCustomLineItemChargeDetails</code> that describes the charge details of a custom line item. </p>
    #[doc(hidden)]
    pub charge_details: ::std::option::Option<crate::types::ListCustomLineItemChargeDetails>,
    /// <p> The custom line item's charge value currency. Only one of the valid values can be used. </p>
    #[doc(hidden)]
    pub currency_code: ::std::option::Option<crate::types::CurrencyCode>,
    /// <p> The custom line item's description. This is shown on the Bills page in association with the charge value. </p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p> The product code that's associated with the custom line item. </p>
    #[doc(hidden)]
    pub product_code: ::std::option::Option<::std::string::String>,
    /// <p> The Amazon Resource Name (ARN) that references the billing group where the custom line item applies to. </p>
    #[doc(hidden)]
    pub billing_group_arn: ::std::option::Option<::std::string::String>,
    /// <p> The time created. </p>
    #[doc(hidden)]
    pub creation_time: i64,
    /// <p> The most recent time when the custom line item was modified. </p>
    #[doc(hidden)]
    pub last_modified_time: i64,
    /// <p> The number of resources that are associated to the custom line item. </p>
    #[doc(hidden)]
    pub association_size: i64,
}
impl CustomLineItemListElement {
    /// <p> The Amazon Resource Names (ARNs) for custom line items. </p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p> The custom line item's name. </p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p> A <code>ListCustomLineItemChargeDetails</code> that describes the charge details of a custom line item. </p>
    pub fn charge_details(
        &self,
    ) -> ::std::option::Option<&crate::types::ListCustomLineItemChargeDetails> {
        self.charge_details.as_ref()
    }
    /// <p> The custom line item's charge value currency. Only one of the valid values can be used. </p>
    pub fn currency_code(&self) -> ::std::option::Option<&crate::types::CurrencyCode> {
        self.currency_code.as_ref()
    }
    /// <p> The custom line item's description. This is shown on the Bills page in association with the charge value. </p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p> The product code that's associated with the custom line item. </p>
    pub fn product_code(&self) -> ::std::option::Option<&str> {
        self.product_code.as_deref()
    }
    /// <p> The Amazon Resource Name (ARN) that references the billing group where the custom line item applies to. </p>
    pub fn billing_group_arn(&self) -> ::std::option::Option<&str> {
        self.billing_group_arn.as_deref()
    }
    /// <p> The time created. </p>
    pub fn creation_time(&self) -> i64 {
        self.creation_time
    }
    /// <p> The most recent time when the custom line item was modified. </p>
    pub fn last_modified_time(&self) -> i64 {
        self.last_modified_time
    }
    /// <p> The number of resources that are associated to the custom line item. </p>
    pub fn association_size(&self) -> i64 {
        self.association_size
    }
}
impl ::std::fmt::Debug for CustomLineItemListElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CustomLineItemListElement");
        formatter.field("arn", &self.arn);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("charge_details", &self.charge_details);
        formatter.field("currency_code", &self.currency_code);
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("product_code", &self.product_code);
        formatter.field("billing_group_arn", &self.billing_group_arn);
        formatter.field("creation_time", &self.creation_time);
        formatter.field("last_modified_time", &self.last_modified_time);
        formatter.field("association_size", &self.association_size);
        formatter.finish()
    }
}
impl CustomLineItemListElement {
    /// Creates a new builder-style object to manufacture [`CustomLineItemListElement`](crate::types::CustomLineItemListElement).
    pub fn builder() -> crate::types::builders::CustomLineItemListElementBuilder {
        crate::types::builders::CustomLineItemListElementBuilder::default()
    }
}

/// A builder for [`CustomLineItemListElement`](crate::types::CustomLineItemListElement).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct CustomLineItemListElementBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) charge_details: ::std::option::Option<crate::types::ListCustomLineItemChargeDetails>,
    pub(crate) currency_code: ::std::option::Option<crate::types::CurrencyCode>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) product_code: ::std::option::Option<::std::string::String>,
    pub(crate) billing_group_arn: ::std::option::Option<::std::string::String>,
    pub(crate) creation_time: ::std::option::Option<i64>,
    pub(crate) last_modified_time: ::std::option::Option<i64>,
    pub(crate) association_size: ::std::option::Option<i64>,
}
impl CustomLineItemListElementBuilder {
    /// <p> The Amazon Resource Names (ARNs) for custom line items. </p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The Amazon Resource Names (ARNs) for custom line items. </p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p> The custom line item's name. </p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The custom line item's name. </p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p> A <code>ListCustomLineItemChargeDetails</code> that describes the charge details of a custom line item. </p>
    pub fn charge_details(mut self, input: crate::types::ListCustomLineItemChargeDetails) -> Self {
        self.charge_details = ::std::option::Option::Some(input);
        self
    }
    /// <p> A <code>ListCustomLineItemChargeDetails</code> that describes the charge details of a custom line item. </p>
    pub fn set_charge_details(
        mut self,
        input: ::std::option::Option<crate::types::ListCustomLineItemChargeDetails>,
    ) -> Self {
        self.charge_details = input;
        self
    }
    /// <p> The custom line item's charge value currency. Only one of the valid values can be used. </p>
    pub fn currency_code(mut self, input: crate::types::CurrencyCode) -> Self {
        self.currency_code = ::std::option::Option::Some(input);
        self
    }
    /// <p> The custom line item's charge value currency. Only one of the valid values can be used. </p>
    pub fn set_currency_code(
        mut self,
        input: ::std::option::Option<crate::types::CurrencyCode>,
    ) -> Self {
        self.currency_code = input;
        self
    }
    /// <p> The custom line item's description. This is shown on the Bills page in association with the charge value. </p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The custom line item's description. This is shown on the Bills page in association with the charge value. </p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p> The product code that's associated with the custom line item. </p>
    pub fn product_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.product_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The product code that's associated with the custom line item. </p>
    pub fn set_product_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.product_code = input;
        self
    }
    /// <p> The Amazon Resource Name (ARN) that references the billing group where the custom line item applies to. </p>
    pub fn billing_group_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.billing_group_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The Amazon Resource Name (ARN) that references the billing group where the custom line item applies to. </p>
    pub fn set_billing_group_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.billing_group_arn = input;
        self
    }
    /// <p> The time created. </p>
    pub fn creation_time(mut self, input: i64) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> The time created. </p>
    pub fn set_creation_time(mut self, input: ::std::option::Option<i64>) -> Self {
        self.creation_time = input;
        self
    }
    /// <p> The most recent time when the custom line item was modified. </p>
    pub fn last_modified_time(mut self, input: i64) -> Self {
        self.last_modified_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> The most recent time when the custom line item was modified. </p>
    pub fn set_last_modified_time(mut self, input: ::std::option::Option<i64>) -> Self {
        self.last_modified_time = input;
        self
    }
    /// <p> The number of resources that are associated to the custom line item. </p>
    pub fn association_size(mut self, input: i64) -> Self {
        self.association_size = ::std::option::Option::Some(input);
        self
    }
    /// <p> The number of resources that are associated to the custom line item. </p>
    pub fn set_association_size(mut self, input: ::std::option::Option<i64>) -> Self {
        self.association_size = input;
        self
    }
    /// Consumes the builder and constructs a [`CustomLineItemListElement`](crate::types::CustomLineItemListElement).
    pub fn build(self) -> crate::types::CustomLineItemListElement {
        crate::types::CustomLineItemListElement {
            arn: self.arn,
            name: self.name,
            charge_details: self.charge_details,
            currency_code: self.currency_code,
            description: self.description,
            product_code: self.product_code,
            billing_group_arn: self.billing_group_arn,
            creation_time: self.creation_time.unwrap_or_default(),
            last_modified_time: self.last_modified_time.unwrap_or_default(),
            association_size: self.association_size.unwrap_or_default(),
        }
    }
}
impl ::std::fmt::Debug for CustomLineItemListElementBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CustomLineItemListElementBuilder");
        formatter.field("arn", &self.arn);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("charge_details", &self.charge_details);
        formatter.field("currency_code", &self.currency_code);
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("product_code", &self.product_code);
        formatter.field("billing_group_arn", &self.billing_group_arn);
        formatter.field("creation_time", &self.creation_time);
        formatter.field("last_modified_time", &self.last_modified_time);
        formatter.field("association_size", &self.association_size);
        formatter.finish()
    }
}
