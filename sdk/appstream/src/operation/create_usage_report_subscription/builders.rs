// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_usage_report_subscription::_create_usage_report_subscription_output::CreateUsageReportSubscriptionOutputBuilder;

pub use crate::operation::create_usage_report_subscription::_create_usage_report_subscription_input::CreateUsageReportSubscriptionInputBuilder;

/// Fluent builder constructing a request to `CreateUsageReportSubscription`.
///
/// <p>Creates a usage report subscription. Usage reports are generated daily.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateUsageReportSubscriptionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::create_usage_report_subscription::builders::CreateUsageReportSubscriptionInputBuilder,
}
impl CreateUsageReportSubscriptionFluentBuilder {
    /// Creates a new `CreateUsageReportSubscription`.
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
            crate::operation::create_usage_report_subscription::CreateUsageReportSubscription,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_usage_report_subscription::CreateUsageReportSubscriptionError,
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
        crate::operation::create_usage_report_subscription::CreateUsageReportSubscriptionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_usage_report_subscription::CreateUsageReportSubscriptionError,
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
        crate::operation::create_usage_report_subscription::CreateUsageReportSubscriptionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_usage_report_subscription::CreateUsageReportSubscriptionError,
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
            crate::operation::create_usage_report_subscription::CreateUsageReportSubscription,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_usage_report_subscription::CreateUsageReportSubscriptionError,
        >,
    > {
        self.customize_middleware().await
    }
}
