// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_account_settings::_update_account_settings_output::UpdateAccountSettingsOutputBuilder;

pub use crate::operation::update_account_settings::_update_account_settings_input::UpdateAccountSettingsInputBuilder;

/// Fluent builder constructing a request to `UpdateAccountSettings`.
///
/// <p>Turns on or turns off optional features in Resource Groups.</p>
/// <p>The preceding example shows that the request to turn on group lifecycle events is <code>IN_PROGRESS</code>. You can call the <code>GetAccountSettings</code> operation to check for completion by looking for <code>GroupLifecycleEventsStatus</code> to change to <code>ACTIVE</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateAccountSettingsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_account_settings::builders::UpdateAccountSettingsInputBuilder,
}
impl UpdateAccountSettingsFluentBuilder {
    /// Creates a new `UpdateAccountSettings`.
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
            crate::operation::update_account_settings::UpdateAccountSettings,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_account_settings::UpdateAccountSettingsError,
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
        crate::operation::update_account_settings::UpdateAccountSettingsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_account_settings::UpdateAccountSettingsError,
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
        crate::operation::update_account_settings::UpdateAccountSettingsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_account_settings::UpdateAccountSettingsError,
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
            crate::operation::update_account_settings::UpdateAccountSettings,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_account_settings::UpdateAccountSettingsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Specifies whether you want to turn <a href="https://docs.aws.amazon.com/ARG/latest/userguide/monitor-groups.html">group lifecycle events</a> on or off.</p>
    pub fn group_lifecycle_events_desired_status(
        mut self,
        input: crate::types::GroupLifecycleEventsDesiredStatus,
    ) -> Self {
        self.inner = self.inner.group_lifecycle_events_desired_status(input);
        self
    }
    /// <p>Specifies whether you want to turn <a href="https://docs.aws.amazon.com/ARG/latest/userguide/monitor-groups.html">group lifecycle events</a> on or off.</p>
    pub fn set_group_lifecycle_events_desired_status(
        mut self,
        input: ::std::option::Option<crate::types::GroupLifecycleEventsDesiredStatus>,
    ) -> Self {
        self.inner = self.inner.set_group_lifecycle_events_desired_status(input);
        self
    }
}
