// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_organization_configuration::_update_organization_configuration_output::UpdateOrganizationConfigurationOutputBuilder;

pub use crate::operation::update_organization_configuration::_update_organization_configuration_input::UpdateOrganizationConfigurationInputBuilder;

/// Fluent builder constructing a request to `UpdateOrganizationConfiguration`.
///
/// <p>Used to update the configuration related to Organizations. Can only be called from a Security Hub administrator account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateOrganizationConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_organization_configuration::builders::UpdateOrganizationConfigurationInputBuilder,
}
impl UpdateOrganizationConfigurationFluentBuilder {
    /// Creates a new `UpdateOrganizationConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::update_organization_configuration::UpdateOrganizationConfiguration, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::update_organization_configuration::UpdateOrganizationConfigurationError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::update_organization_configuration::UpdateOrganizationConfigurationOutput, ::aws_smithy_http::result::SdkError<crate::operation::update_organization_configuration::UpdateOrganizationConfigurationError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::update_organization_configuration::UpdateOrganizationConfigurationOutput, ::aws_smithy_http::result::SdkError<crate::operation::update_organization_configuration::UpdateOrganizationConfigurationError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::update_organization_configuration::UpdateOrganizationConfiguration, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::update_organization_configuration::UpdateOrganizationConfigurationError>
    >{
        self.customize_middleware().await
    }
    /// <p>Whether to automatically enable Security Hub for new accounts in the organization.</p>
    /// <p>By default, this is <code>false</code>, and new accounts are not added automatically.</p>
    /// <p>To automatically enable Security Hub for new accounts, set this to <code>true</code>.</p>
    pub fn auto_enable(mut self, input: bool) -> Self {
        self.inner = self.inner.auto_enable(input);
        self
    }
    /// <p>Whether to automatically enable Security Hub for new accounts in the organization.</p>
    /// <p>By default, this is <code>false</code>, and new accounts are not added automatically.</p>
    /// <p>To automatically enable Security Hub for new accounts, set this to <code>true</code>.</p>
    pub fn set_auto_enable(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_auto_enable(input);
        self
    }
    /// <p>Whether to automatically enable Security Hub <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards-enable-disable.html">default standards</a> for new member accounts in the organization.</p>
    /// <p>By default, this parameter is equal to <code>DEFAULT</code>, and new member accounts are automatically enabled with default Security Hub standards.</p>
    /// <p>To opt out of enabling default standards for new member accounts, set this parameter equal to <code>NONE</code>.</p>
    pub fn auto_enable_standards(mut self, input: crate::types::AutoEnableStandards) -> Self {
        self.inner = self.inner.auto_enable_standards(input);
        self
    }
    /// <p>Whether to automatically enable Security Hub <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards-enable-disable.html">default standards</a> for new member accounts in the organization.</p>
    /// <p>By default, this parameter is equal to <code>DEFAULT</code>, and new member accounts are automatically enabled with default Security Hub standards.</p>
    /// <p>To opt out of enabling default standards for new member accounts, set this parameter equal to <code>NONE</code>.</p>
    pub fn set_auto_enable_standards(
        mut self,
        input: ::std::option::Option<crate::types::AutoEnableStandards>,
    ) -> Self {
        self.inner = self.inner.set_auto_enable_standards(input);
        self
    }
}
