// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_extension_association::_update_extension_association_output::UpdateExtensionAssociationOutputBuilder;

pub use crate::operation::update_extension_association::_update_extension_association_input::UpdateExtensionAssociationInputBuilder;

/// Fluent builder constructing a request to `UpdateExtensionAssociation`.
///
/// <p>Updates an association. For more information about extensions and associations, see <a href="https://docs.aws.amazon.com/appconfig/latest/userguide/working-with-appconfig-extensions.html">Working with AppConfig extensions</a> in the <i>AppConfig User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateExtensionAssociationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_extension_association::builders::UpdateExtensionAssociationInputBuilder,
}
impl UpdateExtensionAssociationFluentBuilder {
    /// Creates a new `UpdateExtensionAssociation`.
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
            crate::operation::update_extension_association::UpdateExtensionAssociation,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_extension_association::UpdateExtensionAssociationError,
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
        crate::operation::update_extension_association::UpdateExtensionAssociationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_extension_association::UpdateExtensionAssociationError,
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
        crate::operation::update_extension_association::UpdateExtensionAssociationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_extension_association::UpdateExtensionAssociationError,
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
            crate::operation::update_extension_association::UpdateExtensionAssociation,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_extension_association::UpdateExtensionAssociationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The system-generated ID for the association.</p>
    pub fn extension_association_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.extension_association_id(input.into());
        self
    }
    /// <p>The system-generated ID for the association.</p>
    pub fn set_extension_association_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_extension_association_id(input);
        self
    }
    /// Adds a key-value pair to `Parameters`.
    ///
    /// To override the contents of this collection use [`set_parameters`](Self::set_parameters).
    ///
    /// <p>The parameter names and values defined in the extension.</p>
    pub fn parameters(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.parameters(k.into(), v.into());
        self
    }
    /// <p>The parameter names and values defined in the extension.</p>
    pub fn set_parameters(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_parameters(input);
        self
    }
}
