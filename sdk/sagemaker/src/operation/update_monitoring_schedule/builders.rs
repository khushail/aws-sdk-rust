// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_monitoring_schedule::_update_monitoring_schedule_output::UpdateMonitoringScheduleOutputBuilder;

pub use crate::operation::update_monitoring_schedule::_update_monitoring_schedule_input::UpdateMonitoringScheduleInputBuilder;

/// Fluent builder constructing a request to `UpdateMonitoringSchedule`.
///
/// <p>Updates a previously created schedule.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateMonitoringScheduleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_monitoring_schedule::builders::UpdateMonitoringScheduleInputBuilder,
}
impl UpdateMonitoringScheduleFluentBuilder {
    /// Creates a new `UpdateMonitoringSchedule`.
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
            crate::operation::update_monitoring_schedule::UpdateMonitoringSchedule,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_monitoring_schedule::UpdateMonitoringScheduleError,
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
        crate::operation::update_monitoring_schedule::UpdateMonitoringScheduleOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_monitoring_schedule::UpdateMonitoringScheduleError,
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
        crate::operation::update_monitoring_schedule::UpdateMonitoringScheduleOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_monitoring_schedule::UpdateMonitoringScheduleError,
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
            crate::operation::update_monitoring_schedule::UpdateMonitoringSchedule,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_monitoring_schedule::UpdateMonitoringScheduleError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the monitoring schedule. The name must be unique within an Amazon Web Services Region within an Amazon Web Services account.</p>
    pub fn monitoring_schedule_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.monitoring_schedule_name(input.into());
        self
    }
    /// <p>The name of the monitoring schedule. The name must be unique within an Amazon Web Services Region within an Amazon Web Services account.</p>
    pub fn set_monitoring_schedule_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_monitoring_schedule_name(input);
        self
    }
    /// <p>The configuration object that specifies the monitoring schedule and defines the monitoring job.</p>
    pub fn monitoring_schedule_config(
        mut self,
        input: crate::types::MonitoringScheduleConfig,
    ) -> Self {
        self.inner = self.inner.monitoring_schedule_config(input);
        self
    }
    /// <p>The configuration object that specifies the monitoring schedule and defines the monitoring job.</p>
    pub fn set_monitoring_schedule_config(
        mut self,
        input: ::std::option::Option<crate::types::MonitoringScheduleConfig>,
    ) -> Self {
        self.inner = self.inner.set_monitoring_schedule_config(input);
        self
    }
}
