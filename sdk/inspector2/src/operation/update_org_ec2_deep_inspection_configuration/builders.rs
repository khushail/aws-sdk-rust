// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_org_ec2_deep_inspection_configuration::_update_org_ec2_deep_inspection_configuration_output::UpdateOrgEc2DeepInspectionConfigurationOutputBuilder;

pub use crate::operation::update_org_ec2_deep_inspection_configuration::_update_org_ec2_deep_inspection_configuration_input::UpdateOrgEc2DeepInspectionConfigurationInputBuilder;

/// Fluent builder constructing a request to `UpdateOrgEc2DeepInspectionConfiguration`.
///
/// <p>Updates the Amazon Inspector deep inspection custom paths for your organization. You must be an Amazon Inspector delegated administrator to use this API.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateOrgEc2DeepInspectionConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_org_ec2_deep_inspection_configuration::builders::UpdateOrgEc2DeepInspectionConfigurationInputBuilder,
}
impl UpdateOrgEc2DeepInspectionConfigurationFluentBuilder {
    /// Creates a new `UpdateOrgEc2DeepInspectionConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::update_org_ec2_deep_inspection_configuration::UpdateOrgEc2DeepInspectionConfiguration, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::update_org_ec2_deep_inspection_configuration::UpdateOrgEc2DeepInspectionConfigurationError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::update_org_ec2_deep_inspection_configuration::UpdateOrgEc2DeepInspectionConfigurationOutput, ::aws_smithy_http::result::SdkError<crate::operation::update_org_ec2_deep_inspection_configuration::UpdateOrgEc2DeepInspectionConfigurationError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::update_org_ec2_deep_inspection_configuration::UpdateOrgEc2DeepInspectionConfigurationOutput, ::aws_smithy_http::result::SdkError<crate::operation::update_org_ec2_deep_inspection_configuration::UpdateOrgEc2DeepInspectionConfigurationError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::update_org_ec2_deep_inspection_configuration::UpdateOrgEc2DeepInspectionConfiguration, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::update_org_ec2_deep_inspection_configuration::UpdateOrgEc2DeepInspectionConfigurationError>
    >{
        self.customize_middleware().await
    }
    /// Appends an item to `orgPackagePaths`.
    ///
    /// To override the contents of this collection use [`set_org_package_paths`](Self::set_org_package_paths).
    ///
    /// <p>The Amazon Inspector deep inspection custom paths you are adding for your organization.</p>
    pub fn org_package_paths(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.org_package_paths(input.into());
        self
    }
    /// <p>The Amazon Inspector deep inspection custom paths you are adding for your organization.</p>
    pub fn set_org_package_paths(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_org_package_paths(input);
        self
    }
}
