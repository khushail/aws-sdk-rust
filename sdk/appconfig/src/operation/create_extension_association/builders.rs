// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_extension_association::_create_extension_association_output::CreateExtensionAssociationOutputBuilder;

pub use crate::operation::create_extension_association::_create_extension_association_input::CreateExtensionAssociationInputBuilder;

/// Fluent builder constructing a request to `CreateExtensionAssociation`.
///
/// <p>When you create an extension or configure an Amazon Web Services authored extension, you associate the extension with an AppConfig application, environment, or configuration profile. For example, you can choose to run the <code>AppConfig deployment events to Amazon SNS</code> Amazon Web Services authored extension and receive notifications on an Amazon SNS topic anytime a configuration deployment is started for a specific application. Defining which extension to associate with an AppConfig resource is called an <i>extension association</i>. An extension association is a specified relationship between an extension and an AppConfig resource, such as an application or a configuration profile. For more information about extensions and associations, see <a href="https://docs.aws.amazon.com/appconfig/latest/userguide/working-with-appconfig-extensions.html">Working with AppConfig extensions</a> in the <i>AppConfig User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateExtensionAssociationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::create_extension_association::builders::CreateExtensionAssociationInputBuilder,
}
impl CreateExtensionAssociationFluentBuilder {
    /// Creates a new `CreateExtensionAssociation`.
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
            crate::operation::create_extension_association::CreateExtensionAssociation,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_extension_association::CreateExtensionAssociationError,
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
        crate::operation::create_extension_association::CreateExtensionAssociationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_extension_association::CreateExtensionAssociationError,
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
        crate::operation::create_extension_association::CreateExtensionAssociationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_extension_association::CreateExtensionAssociationError,
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
            crate::operation::create_extension_association::CreateExtensionAssociation,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_extension_association::CreateExtensionAssociationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name, the ID, or the Amazon Resource Name (ARN) of the extension.</p>
    pub fn extension_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.extension_identifier(input.into());
        self
    }
    /// <p>The name, the ID, or the Amazon Resource Name (ARN) of the extension.</p>
    pub fn set_extension_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_extension_identifier(input);
        self
    }
    /// <p>The version number of the extension. If not specified, AppConfig uses the maximum version of the extension.</p>
    pub fn extension_version_number(mut self, input: i32) -> Self {
        self.inner = self.inner.extension_version_number(input);
        self
    }
    /// <p>The version number of the extension. If not specified, AppConfig uses the maximum version of the extension.</p>
    pub fn set_extension_version_number(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_extension_version_number(input);
        self
    }
    /// <p>The ARN of an application, configuration profile, or environment.</p>
    pub fn resource_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.resource_identifier(input.into());
        self
    }
    /// <p>The ARN of an application, configuration profile, or environment.</p>
    pub fn set_resource_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_resource_identifier(input);
        self
    }
    /// Adds a key-value pair to `Parameters`.
    ///
    /// To override the contents of this collection use [`set_parameters`](Self::set_parameters).
    ///
    /// <p>The parameter names and values defined in the extensions. Extension parameters marked <code>Required</code> must be entered for this field.</p>
    pub fn parameters(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.parameters(k.into(), v.into());
        self
    }
    /// <p>The parameter names and values defined in the extensions. Extension parameters marked <code>Required</code> must be entered for this field.</p>
    pub fn set_parameters(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_parameters(input);
        self
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Adds one or more tags for the specified extension association. Tags are metadata that help you categorize resources in different ways, for example, by purpose, owner, or environment. Each tag consists of a key and an optional value, both of which you define. </p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Adds one or more tags for the specified extension association. Tags are metadata that help you categorize resources in different ways, for example, by purpose, owner, or environment. Each tag consists of a key and an optional value, both of which you define. </p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
