// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_folder_permissions::_update_folder_permissions_output::UpdateFolderPermissionsOutputBuilder;

pub use crate::operation::update_folder_permissions::_update_folder_permissions_input::UpdateFolderPermissionsInputBuilder;

/// Fluent builder constructing a request to `UpdateFolderPermissions`.
///
/// <p>Updates permissions of a folder.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateFolderPermissionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::update_folder_permissions::builders::UpdateFolderPermissionsInputBuilder,
}
impl UpdateFolderPermissionsFluentBuilder {
    /// Creates a new `UpdateFolderPermissions`.
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
            crate::operation::update_folder_permissions::UpdateFolderPermissions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_folder_permissions::UpdateFolderPermissionsError,
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
        crate::operation::update_folder_permissions::UpdateFolderPermissionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_folder_permissions::UpdateFolderPermissionsError,
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
        crate::operation::update_folder_permissions::UpdateFolderPermissionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_folder_permissions::UpdateFolderPermissionsError,
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
            crate::operation::update_folder_permissions::UpdateFolderPermissions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_folder_permissions::UpdateFolderPermissionsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID for the Amazon Web Services account that contains the folder to update.</p>
    pub fn aws_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID for the Amazon Web Services account that contains the folder to update.</p>
    pub fn set_aws_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The ID of the folder.</p>
    pub fn folder_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.folder_id(input.into());
        self
    }
    /// <p>The ID of the folder.</p>
    pub fn set_folder_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_folder_id(input);
        self
    }
    /// Appends an item to `GrantPermissions`.
    ///
    /// To override the contents of this collection use [`set_grant_permissions`](Self::set_grant_permissions).
    ///
    /// <p>The permissions that you want to grant on a resource.</p>
    pub fn grant_permissions(mut self, input: crate::types::ResourcePermission) -> Self {
        self.inner = self.inner.grant_permissions(input);
        self
    }
    /// <p>The permissions that you want to grant on a resource.</p>
    pub fn set_grant_permissions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ResourcePermission>>,
    ) -> Self {
        self.inner = self.inner.set_grant_permissions(input);
        self
    }
    /// Appends an item to `RevokePermissions`.
    ///
    /// To override the contents of this collection use [`set_revoke_permissions`](Self::set_revoke_permissions).
    ///
    /// <p>The permissions that you want to revoke from a resource.</p>
    pub fn revoke_permissions(mut self, input: crate::types::ResourcePermission) -> Self {
        self.inner = self.inner.revoke_permissions(input);
        self
    }
    /// <p>The permissions that you want to revoke from a resource.</p>
    pub fn set_revoke_permissions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ResourcePermission>>,
    ) -> Self {
        self.inner = self.inner.set_revoke_permissions(input);
        self
    }
}
