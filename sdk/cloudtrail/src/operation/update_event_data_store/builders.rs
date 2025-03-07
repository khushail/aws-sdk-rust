// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_event_data_store::_update_event_data_store_output::UpdateEventDataStoreOutputBuilder;

pub use crate::operation::update_event_data_store::_update_event_data_store_input::UpdateEventDataStoreInputBuilder;

/// Fluent builder constructing a request to `UpdateEventDataStore`.
///
/// <p>Updates an event data store. The required <code>EventDataStore</code> value is an ARN or the ID portion of the ARN. Other parameters are optional, but at least one optional parameter must be specified, or CloudTrail throws an error. <code>RetentionPeriod</code> is in days, and valid values are integers between 90 and 2557. By default, <code>TerminationProtection</code> is enabled.</p>
/// <p>For event data stores for CloudTrail events, <code>AdvancedEventSelectors</code> includes or excludes management and data events in your event data store. For more information about <code>AdvancedEventSelectors</code>, see <code>PutEventSelectorsRequest$AdvancedEventSelectors</code>. </p>
/// <p> For event data stores for Config configuration items, Audit Manager evidence, or non-Amazon Web Services events, <code>AdvancedEventSelectors</code> includes events of that type in your event data store.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateEventDataStoreFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_event_data_store::builders::UpdateEventDataStoreInputBuilder,
}
impl UpdateEventDataStoreFluentBuilder {
    /// Creates a new `UpdateEventDataStore`.
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
            crate::operation::update_event_data_store::UpdateEventDataStore,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_event_data_store::UpdateEventDataStoreError,
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
        crate::operation::update_event_data_store::UpdateEventDataStoreOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_event_data_store::UpdateEventDataStoreError,
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
        crate::operation::update_event_data_store::UpdateEventDataStoreOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_event_data_store::UpdateEventDataStoreError,
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
            crate::operation::update_event_data_store::UpdateEventDataStore,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_event_data_store::UpdateEventDataStoreError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ARN (or the ID suffix of the ARN) of the event data store that you want to update.</p>
    pub fn event_data_store(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.event_data_store(input.into());
        self
    }
    /// <p>The ARN (or the ID suffix of the ARN) of the event data store that you want to update.</p>
    pub fn set_event_data_store(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_event_data_store(input);
        self
    }
    /// <p>The event data store name.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The event data store name.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// Appends an item to `AdvancedEventSelectors`.
    ///
    /// To override the contents of this collection use [`set_advanced_event_selectors`](Self::set_advanced_event_selectors).
    ///
    /// <p>The advanced event selectors used to select events for the event data store. You can configure up to five advanced event selectors for each event data store.</p>
    pub fn advanced_event_selectors(mut self, input: crate::types::AdvancedEventSelector) -> Self {
        self.inner = self.inner.advanced_event_selectors(input);
        self
    }
    /// <p>The advanced event selectors used to select events for the event data store. You can configure up to five advanced event selectors for each event data store.</p>
    pub fn set_advanced_event_selectors(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AdvancedEventSelector>>,
    ) -> Self {
        self.inner = self.inner.set_advanced_event_selectors(input);
        self
    }
    /// <p>Specifies whether an event data store collects events from all regions, or only from the region in which it was created.</p>
    pub fn multi_region_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.multi_region_enabled(input);
        self
    }
    /// <p>Specifies whether an event data store collects events from all regions, or only from the region in which it was created.</p>
    pub fn set_multi_region_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_multi_region_enabled(input);
        self
    }
    /// <p>Specifies whether an event data store collects events logged for an organization in Organizations.</p>
    pub fn organization_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.organization_enabled(input);
        self
    }
    /// <p>Specifies whether an event data store collects events logged for an organization in Organizations.</p>
    pub fn set_organization_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_organization_enabled(input);
        self
    }
    /// <p>The retention period, in days.</p>
    pub fn retention_period(mut self, input: i32) -> Self {
        self.inner = self.inner.retention_period(input);
        self
    }
    /// <p>The retention period, in days.</p>
    pub fn set_retention_period(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_retention_period(input);
        self
    }
    /// <p>Indicates that termination protection is enabled and the event data store cannot be automatically deleted.</p>
    pub fn termination_protection_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.termination_protection_enabled(input);
        self
    }
    /// <p>Indicates that termination protection is enabled and the event data store cannot be automatically deleted.</p>
    pub fn set_termination_protection_enabled(
        mut self,
        input: ::std::option::Option<bool>,
    ) -> Self {
        self.inner = self.inner.set_termination_protection_enabled(input);
        self
    }
    /// <p>Specifies the KMS key ID to use to encrypt the events delivered by CloudTrail. The value can be an alias name prefixed by <code>alias/</code>, a fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique identifier.</p> <important>
    /// <p>Disabling or deleting the KMS key, or removing CloudTrail permissions on the key, prevents CloudTrail from logging events to the event data store, and prevents users from querying the data in the event data store that was encrypted with the key. After you associate an event data store with a KMS key, the KMS key cannot be removed or changed. Before you disable or delete a KMS key that you are using with an event data store, delete or back up your event data store.</p>
    /// </important>
    /// <p>CloudTrail also supports KMS multi-Region keys. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// <p>Examples:</p>
    /// <ul>
    /// <li> <p> <code>alias/MyAliasName</code> </p> </li>
    /// <li> <p> <code>arn:aws:kms:us-east-2:123456789012:alias/MyAliasName</code> </p> </li>
    /// <li> <p> <code>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p> <code>12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// </ul>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_id(input.into());
        self
    }
    /// <p>Specifies the KMS key ID to use to encrypt the events delivered by CloudTrail. The value can be an alias name prefixed by <code>alias/</code>, a fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique identifier.</p> <important>
    /// <p>Disabling or deleting the KMS key, or removing CloudTrail permissions on the key, prevents CloudTrail from logging events to the event data store, and prevents users from querying the data in the event data store that was encrypted with the key. After you associate an event data store with a KMS key, the KMS key cannot be removed or changed. Before you disable or delete a KMS key that you are using with an event data store, delete or back up your event data store.</p>
    /// </important>
    /// <p>CloudTrail also supports KMS multi-Region keys. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// <p>Examples:</p>
    /// <ul>
    /// <li> <p> <code>alias/MyAliasName</code> </p> </li>
    /// <li> <p> <code>arn:aws:kms:us-east-2:123456789012:alias/MyAliasName</code> </p> </li>
    /// <li> <p> <code>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p> <code>12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// </ul>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_id(input);
        self
    }
}
