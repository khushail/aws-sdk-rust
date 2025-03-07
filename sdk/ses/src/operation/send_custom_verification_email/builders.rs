// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::send_custom_verification_email::_send_custom_verification_email_output::SendCustomVerificationEmailOutputBuilder;

pub use crate::operation::send_custom_verification_email::_send_custom_verification_email_input::SendCustomVerificationEmailInputBuilder;

/// Fluent builder constructing a request to `SendCustomVerificationEmail`.
///
/// <p>Adds an email address to the list of identities for your Amazon SES account in the current AWS Region and attempts to verify it. As a result of executing this operation, a customized verification email is sent to the specified address.</p>
/// <p>To use this operation, you must first create a custom verification email template. For more information about creating and using custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p>
/// <p>You can execute this operation no more than once per second.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SendCustomVerificationEmailFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::send_custom_verification_email::builders::SendCustomVerificationEmailInputBuilder,
}
impl SendCustomVerificationEmailFluentBuilder {
    /// Creates a new `SendCustomVerificationEmail`.
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
            crate::operation::send_custom_verification_email::SendCustomVerificationEmail,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::send_custom_verification_email::SendCustomVerificationEmailError,
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
        crate::operation::send_custom_verification_email::SendCustomVerificationEmailOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::send_custom_verification_email::SendCustomVerificationEmailError,
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
        crate::operation::send_custom_verification_email::SendCustomVerificationEmailOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::send_custom_verification_email::SendCustomVerificationEmailError,
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
            crate::operation::send_custom_verification_email::SendCustomVerificationEmail,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::send_custom_verification_email::SendCustomVerificationEmailError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The email address to verify.</p>
    pub fn email_address(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.email_address(input.into());
        self
    }
    /// <p>The email address to verify.</p>
    pub fn set_email_address(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_email_address(input);
        self
    }
    /// <p>The name of the custom verification email template to use when sending the verification email.</p>
    pub fn template_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.template_name(input.into());
        self
    }
    /// <p>The name of the custom verification email template to use when sending the verification email.</p>
    pub fn set_template_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_template_name(input);
        self
    }
    /// <p>Name of a configuration set to use when sending the verification email.</p>
    pub fn configuration_set_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.configuration_set_name(input.into());
        self
    }
    /// <p>Name of a configuration set to use when sending the verification email.</p>
    pub fn set_configuration_set_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_configuration_set_name(input);
        self
    }
}
