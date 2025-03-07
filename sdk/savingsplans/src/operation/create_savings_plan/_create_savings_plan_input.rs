// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateSavingsPlanInput {
    /// <p>The ID of the offering.</p>
    #[doc(hidden)]
    pub savings_plan_offering_id: ::std::option::Option<::std::string::String>,
    /// <p>The hourly commitment, in USD. This is a value between 0.001 and 1 million. You cannot specify more than five digits after the decimal point.</p>
    #[doc(hidden)]
    pub commitment: ::std::option::Option<::std::string::String>,
    /// <p>The up-front payment amount. This is a whole number between 50 and 99 percent of the total value of the Savings Plan. This parameter is supported only if the payment option is <code>Partial Upfront</code>.</p>
    #[doc(hidden)]
    pub upfront_payment_amount: ::std::option::Option<::std::string::String>,
    /// <p>The time at which to purchase the Savings Plan, in UTC format (YYYY-MM-DDTHH:MM:SSZ).</p>
    #[doc(hidden)]
    pub purchase_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>One or more tags.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateSavingsPlanInput {
    /// <p>The ID of the offering.</p>
    pub fn savings_plan_offering_id(&self) -> ::std::option::Option<&str> {
        self.savings_plan_offering_id.as_deref()
    }
    /// <p>The hourly commitment, in USD. This is a value between 0.001 and 1 million. You cannot specify more than five digits after the decimal point.</p>
    pub fn commitment(&self) -> ::std::option::Option<&str> {
        self.commitment.as_deref()
    }
    /// <p>The up-front payment amount. This is a whole number between 50 and 99 percent of the total value of the Savings Plan. This parameter is supported only if the payment option is <code>Partial Upfront</code>.</p>
    pub fn upfront_payment_amount(&self) -> ::std::option::Option<&str> {
        self.upfront_payment_amount.as_deref()
    }
    /// <p>The time at which to purchase the Savings Plan, in UTC format (YYYY-MM-DDTHH:MM:SSZ).</p>
    pub fn purchase_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.purchase_time.as_ref()
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>One or more tags.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
}
impl CreateSavingsPlanInput {
    /// Creates a new builder-style object to manufacture [`CreateSavingsPlanInput`](crate::operation::create_savings_plan::CreateSavingsPlanInput).
    pub fn builder(
    ) -> crate::operation::create_savings_plan::builders::CreateSavingsPlanInputBuilder {
        crate::operation::create_savings_plan::builders::CreateSavingsPlanInputBuilder::default()
    }
}

/// A builder for [`CreateSavingsPlanInput`](crate::operation::create_savings_plan::CreateSavingsPlanInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateSavingsPlanInputBuilder {
    pub(crate) savings_plan_offering_id: ::std::option::Option<::std::string::String>,
    pub(crate) commitment: ::std::option::Option<::std::string::String>,
    pub(crate) upfront_payment_amount: ::std::option::Option<::std::string::String>,
    pub(crate) purchase_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateSavingsPlanInputBuilder {
    /// <p>The ID of the offering.</p>
    pub fn savings_plan_offering_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.savings_plan_offering_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the offering.</p>
    pub fn set_savings_plan_offering_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.savings_plan_offering_id = input;
        self
    }
    /// <p>The hourly commitment, in USD. This is a value between 0.001 and 1 million. You cannot specify more than five digits after the decimal point.</p>
    pub fn commitment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.commitment = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The hourly commitment, in USD. This is a value between 0.001 and 1 million. You cannot specify more than five digits after the decimal point.</p>
    pub fn set_commitment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.commitment = input;
        self
    }
    /// <p>The up-front payment amount. This is a whole number between 50 and 99 percent of the total value of the Savings Plan. This parameter is supported only if the payment option is <code>Partial Upfront</code>.</p>
    pub fn upfront_payment_amount(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.upfront_payment_amount = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The up-front payment amount. This is a whole number between 50 and 99 percent of the total value of the Savings Plan. This parameter is supported only if the payment option is <code>Partial Upfront</code>.</p>
    pub fn set_upfront_payment_amount(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.upfront_payment_amount = input;
        self
    }
    /// <p>The time at which to purchase the Savings Plan, in UTC format (YYYY-MM-DDTHH:MM:SSZ).</p>
    pub fn purchase_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.purchase_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which to purchase the Savings Plan, in UTC format (YYYY-MM-DDTHH:MM:SSZ).</p>
    pub fn set_purchase_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.purchase_time = input;
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>One or more tags.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>One or more tags.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateSavingsPlanInput`](crate::operation::create_savings_plan::CreateSavingsPlanInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_savings_plan::CreateSavingsPlanInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_savings_plan::CreateSavingsPlanInput {
                savings_plan_offering_id: self.savings_plan_offering_id,
                commitment: self.commitment,
                upfront_payment_amount: self.upfront_payment_amount,
                purchase_time: self.purchase_time,
                client_token: self.client_token,
                tags: self.tags,
            },
        )
    }
}
