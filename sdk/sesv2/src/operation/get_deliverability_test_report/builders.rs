// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_deliverability_test_report::_get_deliverability_test_report_output::GetDeliverabilityTestReportOutputBuilder;

pub use crate::operation::get_deliverability_test_report::_get_deliverability_test_report_input::GetDeliverabilityTestReportInputBuilder;

/// Fluent builder constructing a request to `GetDeliverabilityTestReport`.
///
/// <p>Retrieve the results of a predictive inbox placement test.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetDeliverabilityTestReportFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::get_deliverability_test_report::builders::GetDeliverabilityTestReportInputBuilder,
}
impl GetDeliverabilityTestReportFluentBuilder {
    /// Creates a new `GetDeliverabilityTestReport`.
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
            crate::operation::get_deliverability_test_report::GetDeliverabilityTestReport,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_deliverability_test_report::GetDeliverabilityTestReportError,
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
        crate::operation::get_deliverability_test_report::GetDeliverabilityTestReportOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_deliverability_test_report::GetDeliverabilityTestReportError,
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
        crate::operation::get_deliverability_test_report::GetDeliverabilityTestReportOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_deliverability_test_report::GetDeliverabilityTestReportError,
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
            crate::operation::get_deliverability_test_report::GetDeliverabilityTestReport,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_deliverability_test_report::GetDeliverabilityTestReportError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>A unique string that identifies the predictive inbox placement test.</p>
    pub fn report_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.report_id(input.into());
        self
    }
    /// <p>A unique string that identifies the predictive inbox placement test.</p>
    pub fn set_report_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_report_id(input);
        self
    }
}
