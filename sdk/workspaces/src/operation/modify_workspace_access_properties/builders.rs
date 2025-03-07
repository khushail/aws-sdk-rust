// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_workspace_access_properties::_modify_workspace_access_properties_output::ModifyWorkspaceAccessPropertiesOutputBuilder;

pub use crate::operation::modify_workspace_access_properties::_modify_workspace_access_properties_input::ModifyWorkspaceAccessPropertiesInputBuilder;

/// Fluent builder constructing a request to `ModifyWorkspaceAccessProperties`.
///
/// <p>Specifies which devices and operating systems users can use to access their WorkSpaces. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/update-directory-details.html#control-device-access"> Control Device Access</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyWorkspaceAccessPropertiesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::modify_workspace_access_properties::builders::ModifyWorkspaceAccessPropertiesInputBuilder,
}
impl ModifyWorkspaceAccessPropertiesFluentBuilder {
    /// Creates a new `ModifyWorkspaceAccessProperties`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::modify_workspace_access_properties::ModifyWorkspaceAccessProperties, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::modify_workspace_access_properties::ModifyWorkspaceAccessPropertiesError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::modify_workspace_access_properties::ModifyWorkspaceAccessPropertiesOutput, ::aws_smithy_http::result::SdkError<crate::operation::modify_workspace_access_properties::ModifyWorkspaceAccessPropertiesError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::modify_workspace_access_properties::ModifyWorkspaceAccessPropertiesOutput, ::aws_smithy_http::result::SdkError<crate::operation::modify_workspace_access_properties::ModifyWorkspaceAccessPropertiesError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::modify_workspace_access_properties::ModifyWorkspaceAccessProperties, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::modify_workspace_access_properties::ModifyWorkspaceAccessPropertiesError>
    >{
        self.customize_middleware().await
    }
    /// <p>The identifier of the directory.</p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>The identifier of the directory.</p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// <p>The device types and operating systems to enable or disable for access.</p>
    pub fn workspace_access_properties(
        mut self,
        input: crate::types::WorkspaceAccessProperties,
    ) -> Self {
        self.inner = self.inner.workspace_access_properties(input);
        self
    }
    /// <p>The device types and operating systems to enable or disable for access.</p>
    pub fn set_workspace_access_properties(
        mut self,
        input: ::std::option::Option<crate::types::WorkspaceAccessProperties>,
    ) -> Self {
        self.inner = self.inner.set_workspace_access_properties(input);
        self
    }
}
