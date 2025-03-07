// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_thing::_create_thing_output::CreateThingOutputBuilder;

pub use crate::operation::create_thing::_create_thing_input::CreateThingInputBuilder;

/// Fluent builder constructing a request to `CreateThing`.
///
/// <p>Creates a thing record in the registry. If this call is made multiple times using the same thing name and configuration, the call will succeed. If this call is made with the same thing name but different configuration a <code>ResourceAlreadyExistsException</code> is thrown.</p> <note>
/// <p>This is a control plane operation. See <a href="https://docs.aws.amazon.com/iot/latest/developerguide/iot-authorization.html">Authorization</a> for information about authorizing control plane actions.</p>
/// </note>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">CreateThing</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateThingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_thing::builders::CreateThingInputBuilder,
}
impl CreateThingFluentBuilder {
    /// Creates a new `CreateThing`.
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
            crate::operation::create_thing::CreateThing,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_thing::CreateThingError>,
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
        crate::operation::create_thing::CreateThingOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_thing::CreateThingError>,
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
        crate::operation::create_thing::CreateThingOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_thing::CreateThingError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_thing::CreateThing,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_thing::CreateThingError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the thing to create.</p>
    /// <p>You can't change a thing's name after you create it. To change a thing's name, you must create a new thing, give it the new name, and then delete the old thing.</p>
    pub fn thing_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.thing_name(input.into());
        self
    }
    /// <p>The name of the thing to create.</p>
    /// <p>You can't change a thing's name after you create it. To change a thing's name, you must create a new thing, give it the new name, and then delete the old thing.</p>
    pub fn set_thing_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_thing_name(input);
        self
    }
    /// <p>The name of the thing type associated with the new thing.</p>
    pub fn thing_type_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.thing_type_name(input.into());
        self
    }
    /// <p>The name of the thing type associated with the new thing.</p>
    pub fn set_thing_type_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_thing_type_name(input);
        self
    }
    /// <p>The attribute payload, which consists of up to three name/value pairs in a JSON document. For example:</p>
    /// <p> <code>{\"attributes\":{\"string1\":\"string2\"}}</code> </p>
    pub fn attribute_payload(mut self, input: crate::types::AttributePayload) -> Self {
        self.inner = self.inner.attribute_payload(input);
        self
    }
    /// <p>The attribute payload, which consists of up to three name/value pairs in a JSON document. For example:</p>
    /// <p> <code>{\"attributes\":{\"string1\":\"string2\"}}</code> </p>
    pub fn set_attribute_payload(
        mut self,
        input: ::std::option::Option<crate::types::AttributePayload>,
    ) -> Self {
        self.inner = self.inner.set_attribute_payload(input);
        self
    }
    /// <p>The name of the billing group the thing will be added to.</p>
    pub fn billing_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.billing_group_name(input.into());
        self
    }
    /// <p>The name of the billing group the thing will be added to.</p>
    pub fn set_billing_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_billing_group_name(input);
        self
    }
}
