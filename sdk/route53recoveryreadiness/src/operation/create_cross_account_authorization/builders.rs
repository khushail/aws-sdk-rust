// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_cross_account_authorization::_create_cross_account_authorization_output::CreateCrossAccountAuthorizationOutputBuilder;

pub use crate::operation::create_cross_account_authorization::_create_cross_account_authorization_input::CreateCrossAccountAuthorizationInputBuilder;

/// Fluent builder constructing a request to `CreateCrossAccountAuthorization`.
///
/// <p>Creates a cross-account readiness authorization. This lets you authorize another account to work with Route 53 Application Recovery Controller, for example, to check the readiness status of resources in a separate account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateCrossAccountAuthorizationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::create_cross_account_authorization::builders::CreateCrossAccountAuthorizationInputBuilder,
}
impl CreateCrossAccountAuthorizationFluentBuilder {
    /// Creates a new `CreateCrossAccountAuthorization`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::create_cross_account_authorization::CreateCrossAccountAuthorization, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::create_cross_account_authorization::CreateCrossAccountAuthorizationError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::create_cross_account_authorization::CreateCrossAccountAuthorizationOutput, ::aws_smithy_http::result::SdkError<crate::operation::create_cross_account_authorization::CreateCrossAccountAuthorizationError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::create_cross_account_authorization::CreateCrossAccountAuthorizationOutput, ::aws_smithy_http::result::SdkError<crate::operation::create_cross_account_authorization::CreateCrossAccountAuthorizationError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::create_cross_account_authorization::CreateCrossAccountAuthorization, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::create_cross_account_authorization::CreateCrossAccountAuthorizationError>
    >{
        self.customize_middleware().await
    }
    /// <p>The cross-account authorization.</p>
    pub fn cross_account_authorization(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.cross_account_authorization(input.into());
        self
    }
    /// <p>The cross-account authorization.</p>
    pub fn set_cross_account_authorization(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_cross_account_authorization(input);
        self
    }
}
