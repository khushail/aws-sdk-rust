// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_phone_number_contact_flow::_disassociate_phone_number_contact_flow_output::DisassociatePhoneNumberContactFlowOutputBuilder;

pub use crate::operation::disassociate_phone_number_contact_flow::_disassociate_phone_number_contact_flow_input::DisassociatePhoneNumberContactFlowInputBuilder;

/// Fluent builder constructing a request to `DisassociatePhoneNumberContactFlow`.
///
/// <p>Removes the flow association from a phone number claimed to your Amazon Connect instance.</p> <important>
/// <p>If the number is claimed to a traffic distribution group, and you are calling this API using an instance in the Amazon Web Services Region where the traffic distribution group was created, you can use either a full phone number ARN or UUID value for the <code>PhoneNumberId</code> URI request parameter. However, if the number is claimed to a traffic distribution group and you are calling this API using an instance in the alternate Amazon Web Services Region associated with the traffic distribution group, you must provide a full phone number ARN. If a UUID is provided in this scenario, you will receive a <code>ResourceNotFoundException</code>.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociatePhoneNumberContactFlowFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::disassociate_phone_number_contact_flow::builders::DisassociatePhoneNumberContactFlowInputBuilder,
}
impl DisassociatePhoneNumberContactFlowFluentBuilder {
    /// Creates a new `DisassociatePhoneNumberContactFlow`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::disassociate_phone_number_contact_flow::DisassociatePhoneNumberContactFlow, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::disassociate_phone_number_contact_flow::DisassociatePhoneNumberContactFlowError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::disassociate_phone_number_contact_flow::DisassociatePhoneNumberContactFlowOutput, ::aws_smithy_http::result::SdkError<crate::operation::disassociate_phone_number_contact_flow::DisassociatePhoneNumberContactFlowError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::disassociate_phone_number_contact_flow::DisassociatePhoneNumberContactFlowOutput, ::aws_smithy_http::result::SdkError<crate::operation::disassociate_phone_number_contact_flow::DisassociatePhoneNumberContactFlowError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::disassociate_phone_number_contact_flow::DisassociatePhoneNumberContactFlow, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::disassociate_phone_number_contact_flow::DisassociatePhoneNumberContactFlowError>
    >{
        self.customize_middleware().await
    }
    /// <p>A unique identifier for the phone number.</p>
    pub fn phone_number_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.phone_number_id(input.into());
        self
    }
    /// <p>A unique identifier for the phone number.</p>
    pub fn set_phone_number_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_phone_number_id(input);
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
}
