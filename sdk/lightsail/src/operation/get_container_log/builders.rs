// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_container_log::_get_container_log_output::GetContainerLogOutputBuilder;

pub use crate::operation::get_container_log::_get_container_log_input::GetContainerLogInputBuilder;

/// Fluent builder constructing a request to `GetContainerLog`.
///
/// <p>Returns the log events of a container of your Amazon Lightsail container service.</p>
/// <p>If your container service has more than one node (i.e., a scale greater than 1), then the log events that are returned for the specified container are merged from all nodes on your container service.</p> <note>
/// <p>Container logs are retained for a certain amount of time. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/lightsail.html">Amazon Lightsail endpoints and quotas</a> in the <i>Amazon Web Services General Reference</i>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetContainerLogFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_container_log::builders::GetContainerLogInputBuilder,
}
impl GetContainerLogFluentBuilder {
    /// Creates a new `GetContainerLog`.
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
            crate::operation::get_container_log::GetContainerLog,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_container_log::GetContainerLogError,
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
        crate::operation::get_container_log::GetContainerLogOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_container_log::GetContainerLogError,
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
        crate::operation::get_container_log::GetContainerLogOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_container_log::GetContainerLogError,
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
            crate::operation::get_container_log::GetContainerLog,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_container_log::GetContainerLogError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the container service for which to get a container log.</p>
    pub fn service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_name(input.into());
        self
    }
    /// <p>The name of the container service for which to get a container log.</p>
    pub fn set_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_name(input);
        self
    }
    /// <p>The name of the container that is either running or previously ran on the container service for which to return a log.</p>
    pub fn container_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.container_name(input.into());
        self
    }
    /// <p>The name of the container that is either running or previously ran on the container service for which to return a log.</p>
    pub fn set_container_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_container_name(input);
        self
    }
    /// <p>The start of the time interval for which to get log data.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li>
    /// <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use a start time of October 1, 2018, at 8 PM UTC, specify <code>1538424000</code> as the start time.</p> </li>
    /// </ul>
    /// <p>You can convert a human-friendly time to Unix time format using a converter like <a href="https://www.epochconverter.com/">Epoch converter</a>.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The start of the time interval for which to get log data.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li>
    /// <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use a start time of October 1, 2018, at 8 PM UTC, specify <code>1538424000</code> as the start time.</p> </li>
    /// </ul>
    /// <p>You can convert a human-friendly time to Unix time format using a converter like <a href="https://www.epochconverter.com/">Epoch converter</a>.</p>
    pub fn set_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The end of the time interval for which to get log data.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li>
    /// <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use an end time of October 1, 2018, at 9 PM UTC, specify <code>1538427600</code> as the end time.</p> </li>
    /// </ul>
    /// <p>You can convert a human-friendly time to Unix time format using a converter like <a href="https://www.epochconverter.com/">Epoch converter</a>.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>The end of the time interval for which to get log data.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li>
    /// <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use an end time of October 1, 2018, at 9 PM UTC, specify <code>1538427600</code> as the end time.</p> </li>
    /// </ul>
    /// <p>You can convert a human-friendly time to Unix time format using a converter like <a href="https://www.epochconverter.com/">Epoch converter</a>.</p>
    pub fn set_end_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>The pattern to use to filter the returned log events to a specific term.</p>
    /// <p>The following are a few examples of filter patterns that you can specify:</p>
    /// <ul>
    /// <li> <p>To return all log events, specify a filter pattern of <code>""</code>.</p> </li>
    /// <li> <p>To exclude log events that contain the <code>ERROR</code> term, and return all other log events, specify a filter pattern of <code>"-ERROR"</code>.</p> </li>
    /// <li> <p>To return log events that contain the <code>ERROR</code> term, specify a filter pattern of <code>"ERROR"</code>.</p> </li>
    /// <li> <p>To return log events that contain both the <code>ERROR</code> and <code>Exception</code> terms, specify a filter pattern of <code>"ERROR Exception"</code>.</p> </li>
    /// <li> <p>To return log events that contain the <code>ERROR</code> <i>or</i> the <code>Exception</code> term, specify a filter pattern of <code>"?ERROR ?Exception"</code>.</p> </li>
    /// </ul>
    pub fn filter_pattern(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.filter_pattern(input.into());
        self
    }
    /// <p>The pattern to use to filter the returned log events to a specific term.</p>
    /// <p>The following are a few examples of filter patterns that you can specify:</p>
    /// <ul>
    /// <li> <p>To return all log events, specify a filter pattern of <code>""</code>.</p> </li>
    /// <li> <p>To exclude log events that contain the <code>ERROR</code> term, and return all other log events, specify a filter pattern of <code>"-ERROR"</code>.</p> </li>
    /// <li> <p>To return log events that contain the <code>ERROR</code> term, specify a filter pattern of <code>"ERROR"</code>.</p> </li>
    /// <li> <p>To return log events that contain both the <code>ERROR</code> and <code>Exception</code> terms, specify a filter pattern of <code>"ERROR Exception"</code>.</p> </li>
    /// <li> <p>To return log events that contain the <code>ERROR</code> <i>or</i> the <code>Exception</code> term, specify a filter pattern of <code>"?ERROR ?Exception"</code>.</p> </li>
    /// </ul>
    pub fn set_filter_pattern(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_filter_pattern(input);
        self
    }
    /// <p>The token to advance to the next page of results from your request.</p>
    /// <p>To get a page token, perform an initial <code>GetContainerLog</code> request. If your results are paginated, the response will return a next page token that you can specify as the page token in a subsequent request.</p>
    pub fn page_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.page_token(input.into());
        self
    }
    /// <p>The token to advance to the next page of results from your request.</p>
    /// <p>To get a page token, perform an initial <code>GetContainerLog</code> request. If your results are paginated, the response will return a next page token that you can specify as the page token in a subsequent request.</p>
    pub fn set_page_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_page_token(input);
        self
    }
}
