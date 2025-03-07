// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::add_application_input::_add_application_input_output::AddApplicationInputOutputBuilder;

pub use crate::operation::add_application_input::_add_application_input_input::AddApplicationInputInputBuilder;

/// Fluent builder constructing a request to `AddApplicationInput`.
///
/// <p> Adds a streaming source to your SQL-based Kinesis Data Analytics application. </p>
/// <p>You can add a streaming source when you create an application, or you can use this operation to add a streaming source after you create an application. For more information, see <code>CreateApplication</code>.</p>
/// <p>Any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <code>DescribeApplication</code> operation to find the current application version. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AddApplicationInputFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::add_application_input::builders::AddApplicationInputInputBuilder,
}
impl AddApplicationInputFluentBuilder {
    /// Creates a new `AddApplicationInput`.
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
            crate::operation::add_application_input::AddApplicationInput,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::add_application_input::AddApplicationInputError,
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
        crate::operation::add_application_input::AddApplicationInputOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::add_application_input::AddApplicationInputError,
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
        crate::operation::add_application_input::AddApplicationInputOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::add_application_input::AddApplicationInputError,
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
            crate::operation::add_application_input::AddApplicationInput,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::add_application_input::AddApplicationInputError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of your existing application to which you want to add the streaming source.</p>
    pub fn application_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.application_name(input.into());
        self
    }
    /// <p>The name of your existing application to which you want to add the streaming source.</p>
    pub fn set_application_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_application_name(input);
        self
    }
    /// <p>The current version of your application. You must provide the <code>ApplicationVersionID</code> or the <code>ConditionalToken</code>.You can use the <code>DescribeApplication</code> operation to find the current application version.</p>
    pub fn current_application_version_id(mut self, input: i64) -> Self {
        self.inner = self.inner.current_application_version_id(input);
        self
    }
    /// <p>The current version of your application. You must provide the <code>ApplicationVersionID</code> or the <code>ConditionalToken</code>.You can use the <code>DescribeApplication</code> operation to find the current application version.</p>
    pub fn set_current_application_version_id(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_current_application_version_id(input);
        self
    }
    /// <p>The <code>Input</code> to add.</p>
    pub fn input(mut self, input: crate::types::Input) -> Self {
        self.inner = self.inner.input(input);
        self
    }
    /// <p>The <code>Input</code> to add.</p>
    pub fn set_input(mut self, input: ::std::option::Option<crate::types::Input>) -> Self {
        self.inner = self.inner.set_input(input);
        self
    }
}
