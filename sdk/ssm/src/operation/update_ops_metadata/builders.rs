// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_ops_metadata::_update_ops_metadata_output::UpdateOpsMetadataOutputBuilder;

pub use crate::operation::update_ops_metadata::_update_ops_metadata_input::UpdateOpsMetadataInputBuilder;

/// Fluent builder constructing a request to `UpdateOpsMetadata`.
///
/// <p>Amazon Web Services Systems Manager calls this API operation when you edit OpsMetadata in Application Manager.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateOpsMetadataFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_ops_metadata::builders::UpdateOpsMetadataInputBuilder,
}
impl UpdateOpsMetadataFluentBuilder {
    /// Creates a new `UpdateOpsMetadata`.
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
            crate::operation::update_ops_metadata::UpdateOpsMetadata,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_ops_metadata::UpdateOpsMetadataError,
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
        crate::operation::update_ops_metadata::UpdateOpsMetadataOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_ops_metadata::UpdateOpsMetadataError,
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
        crate::operation::update_ops_metadata::UpdateOpsMetadataOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_ops_metadata::UpdateOpsMetadataError,
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
            crate::operation::update_ops_metadata::UpdateOpsMetadata,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_ops_metadata::UpdateOpsMetadataError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Resource Name (ARN) of the OpsMetadata Object to update.</p>
    pub fn ops_metadata_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.ops_metadata_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the OpsMetadata Object to update.</p>
    pub fn set_ops_metadata_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_ops_metadata_arn(input);
        self
    }
    /// Adds a key-value pair to `MetadataToUpdate`.
    ///
    /// To override the contents of this collection use [`set_metadata_to_update`](Self::set_metadata_to_update).
    ///
    /// <p>Metadata to add to an OpsMetadata object.</p>
    pub fn metadata_to_update(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::MetadataValue,
    ) -> Self {
        self.inner = self.inner.metadata_to_update(k.into(), v);
        self
    }
    /// <p>Metadata to add to an OpsMetadata object.</p>
    pub fn set_metadata_to_update(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::MetadataValue>,
        >,
    ) -> Self {
        self.inner = self.inner.set_metadata_to_update(input);
        self
    }
    /// Appends an item to `KeysToDelete`.
    ///
    /// To override the contents of this collection use [`set_keys_to_delete`](Self::set_keys_to_delete).
    ///
    /// <p>The metadata keys to delete from the OpsMetadata object. </p>
    pub fn keys_to_delete(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.keys_to_delete(input.into());
        self
    }
    /// <p>The metadata keys to delete from the OpsMetadata object. </p>
    pub fn set_keys_to_delete(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_keys_to_delete(input);
        self
    }
}
