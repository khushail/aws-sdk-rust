// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_sip_media_application_call::_create_sip_media_application_call_output::CreateSipMediaApplicationCallOutputBuilder;

pub use crate::operation::create_sip_media_application_call::_create_sip_media_application_call_input::CreateSipMediaApplicationCallInputBuilder;

/// Fluent builder constructing a request to `CreateSipMediaApplicationCall`.
///
/// <p>Creates an outbound call to a phone number from the phone number specified in the request, and it invokes the endpoint of the specified <code>sipMediaApplicationId</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateSipMediaApplicationCallFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::create_sip_media_application_call::builders::CreateSipMediaApplicationCallInputBuilder,
}
impl CreateSipMediaApplicationCallFluentBuilder {
    /// Creates a new `CreateSipMediaApplicationCall`.
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
            crate::operation::create_sip_media_application_call::CreateSipMediaApplicationCall,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_sip_media_application_call::CreateSipMediaApplicationCallError,
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
        crate::operation::create_sip_media_application_call::CreateSipMediaApplicationCallOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_sip_media_application_call::CreateSipMediaApplicationCallError,
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
        crate::operation::create_sip_media_application_call::CreateSipMediaApplicationCallOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_sip_media_application_call::CreateSipMediaApplicationCallError,
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
            crate::operation::create_sip_media_application_call::CreateSipMediaApplicationCall,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_sip_media_application_call::CreateSipMediaApplicationCallError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The phone number that a user calls from. This is a phone number in your Amazon Chime SDK phone number inventory.</p>
    pub fn from_phone_number(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.from_phone_number(input.into());
        self
    }
    /// <p>The phone number that a user calls from. This is a phone number in your Amazon Chime SDK phone number inventory.</p>
    pub fn set_from_phone_number(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_from_phone_number(input);
        self
    }
    /// <p>The phone number that the service should call.</p>
    pub fn to_phone_number(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.to_phone_number(input.into());
        self
    }
    /// <p>The phone number that the service should call.</p>
    pub fn set_to_phone_number(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_to_phone_number(input);
        self
    }
    /// <p>The ID of the SIP media application.</p>
    pub fn sip_media_application_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.sip_media_application_id(input.into());
        self
    }
    /// <p>The ID of the SIP media application.</p>
    pub fn set_sip_media_application_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_sip_media_application_id(input);
        self
    }
    /// Adds a key-value pair to `SipHeaders`.
    ///
    /// To override the contents of this collection use [`set_sip_headers`](Self::set_sip_headers).
    ///
    /// <p>The SIP headers added to an outbound call leg.</p>
    pub fn sip_headers(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.sip_headers(k.into(), v.into());
        self
    }
    /// <p>The SIP headers added to an outbound call leg.</p>
    pub fn set_sip_headers(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_sip_headers(input);
        self
    }
    /// Adds a key-value pair to `ArgumentsMap`.
    ///
    /// To override the contents of this collection use [`set_arguments_map`](Self::set_arguments_map).
    ///
    /// <p>Context passed to a CreateSipMediaApplication API call. For example, you could pass key-value pairs such as: <code>"FirstName": "John", "LastName": "Doe"</code> </p>
    pub fn arguments_map(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.arguments_map(k.into(), v.into());
        self
    }
    /// <p>Context passed to a CreateSipMediaApplication API call. For example, you could pass key-value pairs such as: <code>"FirstName": "John", "LastName": "Doe"</code> </p>
    pub fn set_arguments_map(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_arguments_map(input);
        self
    }
}
