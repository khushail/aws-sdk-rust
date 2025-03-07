// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_pricing_plan::_update_pricing_plan_output::UpdatePricingPlanOutputBuilder;

pub use crate::operation::update_pricing_plan::_update_pricing_plan_input::UpdatePricingPlanInputBuilder;

/// Fluent builder constructing a request to `UpdatePricingPlan`.
///
/// <p>Update the pricing plan.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdatePricingPlanFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_pricing_plan::builders::UpdatePricingPlanInputBuilder,
}
impl UpdatePricingPlanFluentBuilder {
    /// Creates a new `UpdatePricingPlan`.
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
            crate::operation::update_pricing_plan::UpdatePricingPlan,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_pricing_plan::UpdatePricingPlanError,
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
        crate::operation::update_pricing_plan::UpdatePricingPlanOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_pricing_plan::UpdatePricingPlanError,
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
        crate::operation::update_pricing_plan::UpdatePricingPlanOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_pricing_plan::UpdatePricingPlanError,
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
            crate::operation::update_pricing_plan::UpdatePricingPlan,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_pricing_plan::UpdatePricingPlanError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The pricing mode.</p>
    pub fn pricing_mode(mut self, input: crate::types::PricingMode) -> Self {
        self.inner = self.inner.pricing_mode(input);
        self
    }
    /// <p>The pricing mode.</p>
    pub fn set_pricing_mode(
        mut self,
        input: ::std::option::Option<crate::types::PricingMode>,
    ) -> Self {
        self.inner = self.inner.set_pricing_mode(input);
        self
    }
    /// Appends an item to `bundleNames`.
    ///
    /// To override the contents of this collection use [`set_bundle_names`](Self::set_bundle_names).
    ///
    /// <p>The bundle names.</p>
    pub fn bundle_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bundle_names(input.into());
        self
    }
    /// <p>The bundle names.</p>
    pub fn set_bundle_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_bundle_names(input);
        self
    }
}
