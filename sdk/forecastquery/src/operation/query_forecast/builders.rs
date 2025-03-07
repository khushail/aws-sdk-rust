// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::query_forecast::_query_forecast_output::QueryForecastOutputBuilder;

pub use crate::operation::query_forecast::_query_forecast_input::QueryForecastInputBuilder;

/// Fluent builder constructing a request to `QueryForecast`.
///
/// <p>Retrieves a forecast for a single item, filtered by the supplied criteria.</p>
/// <p>The criteria is a key-value pair. The key is either <code>item_id</code> (or the equivalent non-timestamp, non-target field) from the <code>TARGET_TIME_SERIES</code> dataset, or one of the forecast dimensions specified as part of the <code>FeaturizationConfig</code> object.</p>
/// <p>By default, <code>QueryForecast</code> returns the complete date range for the filtered forecast. You can request a specific date range.</p>
/// <p>To get the full forecast, use the <a href="https://docs.aws.amazon.com/en_us/forecast/latest/dg/API_CreateForecastExportJob.html">CreateForecastExportJob</a> operation.</p> <note>
/// <p>The forecasts generated by Amazon Forecast are in the same timezone as the dataset that was used to create the predictor.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct QueryForecastFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::query_forecast::builders::QueryForecastInputBuilder,
}
impl QueryForecastFluentBuilder {
    /// Creates a new `QueryForecast`.
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
            crate::operation::query_forecast::QueryForecast,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::query_forecast::QueryForecastError>,
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
        crate::operation::query_forecast::QueryForecastOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::query_forecast::QueryForecastError>,
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
        crate::operation::query_forecast::QueryForecastOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::query_forecast::QueryForecastError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::query_forecast::QueryForecast,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::query_forecast::QueryForecastError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Resource Name (ARN) of the forecast to query.</p>
    pub fn forecast_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.forecast_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the forecast to query.</p>
    pub fn set_forecast_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_forecast_arn(input);
        self
    }
    /// <p>The start date for the forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T08:00:00.</p>
    pub fn start_date(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.start_date(input.into());
        self
    }
    /// <p>The start date for the forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T08:00:00.</p>
    pub fn set_start_date(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_start_date(input);
        self
    }
    /// <p>The end date for the forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T20:00:00. </p>
    pub fn end_date(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.end_date(input.into());
        self
    }
    /// <p>The end date for the forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T20:00:00. </p>
    pub fn set_end_date(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_end_date(input);
        self
    }
    /// Adds a key-value pair to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filtering criteria to apply when retrieving the forecast. For example, to get the forecast for <code>client_21</code> in the electricity usage dataset, specify the following:</p>
    /// <p> <code>{"item_id" : "client_21"}</code> </p>
    /// <p>To get the full forecast, use the <a href="https://docs.aws.amazon.com/en_us/forecast/latest/dg/API_CreateForecastExportJob.html">CreateForecastExportJob</a> operation.</p>
    pub fn filters(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.filters(k.into(), v.into());
        self
    }
    /// <p>The filtering criteria to apply when retrieving the forecast. For example, to get the forecast for <code>client_21</code> in the electricity usage dataset, specify the following:</p>
    /// <p> <code>{"item_id" : "client_21"}</code> </p>
    /// <p>To get the full forecast, use the <a href="https://docs.aws.amazon.com/en_us/forecast/latest/dg/API_CreateForecastExportJob.html">CreateForecastExportJob</a> operation.</p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
