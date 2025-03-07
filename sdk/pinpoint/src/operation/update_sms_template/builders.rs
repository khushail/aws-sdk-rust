// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_sms_template::_update_sms_template_output::UpdateSmsTemplateOutputBuilder;

pub use crate::operation::update_sms_template::_update_sms_template_input::UpdateSmsTemplateInputBuilder;

/// Fluent builder constructing a request to `UpdateSmsTemplate`.
///
/// <p>Updates an existing message template for messages that are sent through the SMS channel.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateSmsTemplateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_sms_template::builders::UpdateSmsTemplateInputBuilder,
}
impl UpdateSmsTemplateFluentBuilder {
    /// Creates a new `UpdateSmsTemplate`.
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
            crate::operation::update_sms_template::UpdateSmsTemplate,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_sms_template::UpdateSmsTemplateError,
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
        crate::operation::update_sms_template::UpdateSmsTemplateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_sms_template::UpdateSmsTemplateError,
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
        crate::operation::update_sms_template::UpdateSmsTemplateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_sms_template::UpdateSmsTemplateError,
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
            crate::operation::update_sms_template::UpdateSmsTemplate,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_sms_template::UpdateSmsTemplateError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Specifies whether to save the updates as a new version of the message template. Valid values are: true, save the updates as a new version; and, false, save the updates to (overwrite) the latest existing version of the template.</p>
    /// <p>If you don't specify a value for this parameter, Amazon Pinpoint saves the updates to (overwrites) the latest existing version of the template. If you specify a value of true for this parameter, don't specify a value for the version parameter. Otherwise, an error will occur.</p>
    pub fn create_new_version(mut self, input: bool) -> Self {
        self.inner = self.inner.create_new_version(input);
        self
    }
    /// <p>Specifies whether to save the updates as a new version of the message template. Valid values are: true, save the updates as a new version; and, false, save the updates to (overwrite) the latest existing version of the template.</p>
    /// <p>If you don't specify a value for this parameter, Amazon Pinpoint saves the updates to (overwrites) the latest existing version of the template. If you specify a value of true for this parameter, don't specify a value for the version parameter. Otherwise, an error will occur.</p>
    pub fn set_create_new_version(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_create_new_version(input);
        self
    }
    /// <p>Specifies the content and settings for a message template that can be used in text messages that are sent through the SMS channel.</p>
    pub fn sms_template_request(mut self, input: crate::types::SmsTemplateRequest) -> Self {
        self.inner = self.inner.sms_template_request(input);
        self
    }
    /// <p>Specifies the content and settings for a message template that can be used in text messages that are sent through the SMS channel.</p>
    pub fn set_sms_template_request(
        mut self,
        input: ::std::option::Option<crate::types::SmsTemplateRequest>,
    ) -> Self {
        self.inner = self.inner.set_sms_template_request(input);
        self
    }
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    pub fn template_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.template_name(input.into());
        self
    }
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    pub fn set_template_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_template_name(input);
        self
    }
    /// <p>The unique identifier for the version of the message template to update, retrieve information about, or delete. To retrieve identifiers and other information for all the versions of a template, use the
    /// <link linkend="templates-template-name-template-type-versions">Template Versions resource.</p>
    /// <p>If specified, this value must match the identifier for an existing template version. If specified for an update operation, this value must match the identifier for the latest existing version of the template. This restriction helps ensure that race conditions don't occur.</p>
    /// <p>If you don't specify a value for this parameter, Amazon Pinpoint does the following:</p>
    /// <ul>
    /// <li><p>For a get operation, retrieves information about the active version of the template.</p></li>
    /// <li><p>For an update operation, saves the updates to (overwrites) the latest existing version of the template, if the create-new-version parameter isn't used or is set to false.</p></li>
    /// <li><p>For a delete operation, deletes the template, including all versions of the template.</p></li>
    /// </ul>
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.version(input.into());
        self
    }
    /// <p>The unique identifier for the version of the message template to update, retrieve information about, or delete. To retrieve identifiers and other information for all the versions of a template, use the
    /// <link linkend="templates-template-name-template-type-versions">Template Versions resource.</p>
    /// <p>If specified, this value must match the identifier for an existing template version. If specified for an update operation, this value must match the identifier for the latest existing version of the template. This restriction helps ensure that race conditions don't occur.</p>
    /// <p>If you don't specify a value for this parameter, Amazon Pinpoint does the following:</p>
    /// <ul>
    /// <li><p>For a get operation, retrieves information about the active version of the template.</p></li>
    /// <li><p>For an update operation, saves the updates to (overwrites) the latest existing version of the template, if the create-new-version parameter isn't used or is set to false.</p></li>
    /// <li><p>For a delete operation, deletes the template, including all versions of the template.</p></li>
    /// </ul>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_version(input);
        self
    }
}
