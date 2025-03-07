// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_pricing_rules::_list_pricing_rules_output::ListPricingRulesOutputBuilder;

pub use crate::operation::list_pricing_rules::_list_pricing_rules_input::ListPricingRulesInputBuilder;

/// Fluent builder constructing a request to `ListPricingRules`.
///
/// <p> Describes a pricing rule that can be associated to a pricing plan, or set of pricing plans. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListPricingRulesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_pricing_rules::builders::ListPricingRulesInputBuilder,
}
impl ListPricingRulesFluentBuilder {
    /// Creates a new `ListPricingRules`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_pricing_rules::ListPricingRules,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_pricing_rules::ListPricingRulesError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_pricing_rules::ListPricingRulesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_pricing_rules::ListPricingRulesError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_pricing_rules::ListPricingRulesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_pricing_rules::ListPricingRulesError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_pricing_rules::ListPricingRules,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_pricing_rules::ListPricingRulesError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_pricing_rules::paginator::ListPricingRulesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_pricing_rules::paginator::ListPricingRulesPaginator {
        crate::operation::list_pricing_rules::paginator::ListPricingRulesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p> The preferred billing period to get the pricing plan. </p>
    pub fn billing_period(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.billing_period(input.into());
        self
    }
    /// <p> The preferred billing period to get the pricing plan. </p>
    pub fn set_billing_period(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_billing_period(input);
        self
    }
    /// <p> A <code>DescribePricingRuleFilter</code> that specifies the Amazon Resource Name (ARNs) of pricing rules to retrieve pricing rules information. </p>
    pub fn filters(mut self, input: crate::types::ListPricingRulesFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p> A <code>DescribePricingRuleFilter</code> that specifies the Amazon Resource Name (ARNs) of pricing rules to retrieve pricing rules information. </p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<crate::types::ListPricingRulesFilter>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p> The maximum number of pricing rules to retrieve. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p> The maximum number of pricing rules to retrieve. </p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p> The pagination token that's used on subsequent call to get pricing rules. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p> The pagination token that's used on subsequent call to get pricing rules. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
