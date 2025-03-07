// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_create_account_status::_describe_create_account_status_output::DescribeCreateAccountStatusOutputBuilder;

pub use crate::operation::describe_create_account_status::_describe_create_account_status_input::DescribeCreateAccountStatusInputBuilder;

/// Fluent builder constructing a request to `DescribeCreateAccountStatus`.
///
/// <p>Retrieves the current status of an asynchronous request to create an account.</p>
/// <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an Amazon Web Services service.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeCreateAccountStatusFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_create_account_status::builders::DescribeCreateAccountStatusInputBuilder,
}
impl DescribeCreateAccountStatusFluentBuilder {
    /// Creates a new `DescribeCreateAccountStatus`.
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
            crate::operation::describe_create_account_status::DescribeCreateAccountStatus,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_create_account_status::DescribeCreateAccountStatusError,
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
        crate::operation::describe_create_account_status::DescribeCreateAccountStatusOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_create_account_status::DescribeCreateAccountStatusError,
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
        crate::operation::describe_create_account_status::DescribeCreateAccountStatusOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_create_account_status::DescribeCreateAccountStatusError,
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
            crate::operation::describe_create_account_status::DescribeCreateAccountStatus,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_create_account_status::DescribeCreateAccountStatusError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Specifies the <code>Id</code> value that uniquely identifies the <code>CreateAccount</code> request. You can get the value from the <code>CreateAccountStatus.Id</code> response in an earlier <code>CreateAccount</code> request, or from the <code>ListCreateAccountStatus</code> operation.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a create account request ID string requires "car-" followed by from 8 to 32 lowercase letters or digits.</p>
    pub fn create_account_request_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.create_account_request_id(input.into());
        self
    }
    /// <p>Specifies the <code>Id</code> value that uniquely identifies the <code>CreateAccount</code> request. You can get the value from the <code>CreateAccountStatus.Id</code> response in an earlier <code>CreateAccount</code> request, or from the <code>ListCreateAccountStatus</code> operation.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a create account request ID string requires "car-" followed by from 8 to 32 lowercase letters or digits.</p>
    pub fn set_create_account_request_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_create_account_request_id(input);
        self
    }
}
