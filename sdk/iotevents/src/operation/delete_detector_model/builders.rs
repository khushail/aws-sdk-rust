// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_detector_model::_delete_detector_model_output::DeleteDetectorModelOutputBuilder;

pub use crate::operation::delete_detector_model::_delete_detector_model_input::DeleteDetectorModelInputBuilder;

/// Fluent builder constructing a request to `DeleteDetectorModel`.
///
/// <p>Deletes a detector model. Any active instances of the detector model are also deleted.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteDetectorModelFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_detector_model::builders::DeleteDetectorModelInputBuilder,
}
impl DeleteDetectorModelFluentBuilder {
    /// Creates a new `DeleteDetectorModel`.
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
            crate::operation::delete_detector_model::DeleteDetectorModel,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_detector_model::DeleteDetectorModelError,
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
        crate::operation::delete_detector_model::DeleteDetectorModelOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_detector_model::DeleteDetectorModelError,
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
        crate::operation::delete_detector_model::DeleteDetectorModelOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_detector_model::DeleteDetectorModelError,
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
            crate::operation::delete_detector_model::DeleteDetectorModel,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_detector_model::DeleteDetectorModelError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the detector model to be deleted.</p>
    pub fn detector_model_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.detector_model_name(input.into());
        self
    }
    /// <p>The name of the detector model to be deleted.</p>
    pub fn set_detector_model_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_detector_model_name(input);
        self
    }
}
