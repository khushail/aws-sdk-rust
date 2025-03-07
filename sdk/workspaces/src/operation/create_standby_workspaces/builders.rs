// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_standby_workspaces::_create_standby_workspaces_output::CreateStandbyWorkspacesOutputBuilder;

pub use crate::operation::create_standby_workspaces::_create_standby_workspaces_input::CreateStandbyWorkspacesInputBuilder;

/// Fluent builder constructing a request to `CreateStandbyWorkspaces`.
///
/// <p>Creates a standby WorkSpace in a secondary Region.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateStandbyWorkspacesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::create_standby_workspaces::builders::CreateStandbyWorkspacesInputBuilder,
}
impl CreateStandbyWorkspacesFluentBuilder {
    /// Creates a new `CreateStandbyWorkspaces`.
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
            crate::operation::create_standby_workspaces::CreateStandbyWorkspaces,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_standby_workspaces::CreateStandbyWorkspacesError,
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
        crate::operation::create_standby_workspaces::CreateStandbyWorkspacesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_standby_workspaces::CreateStandbyWorkspacesError,
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
        crate::operation::create_standby_workspaces::CreateStandbyWorkspacesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_standby_workspaces::CreateStandbyWorkspacesError,
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
            crate::operation::create_standby_workspaces::CreateStandbyWorkspaces,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_standby_workspaces::CreateStandbyWorkspacesError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Region of the primary WorkSpace.</p>
    pub fn primary_region(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.primary_region(input.into());
        self
    }
    /// <p>The Region of the primary WorkSpace.</p>
    pub fn set_primary_region(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_primary_region(input);
        self
    }
    /// Appends an item to `StandbyWorkspaces`.
    ///
    /// To override the contents of this collection use [`set_standby_workspaces`](Self::set_standby_workspaces).
    ///
    /// <p>Information about the standby WorkSpace to be created.</p>
    pub fn standby_workspaces(mut self, input: crate::types::StandbyWorkspace) -> Self {
        self.inner = self.inner.standby_workspaces(input);
        self
    }
    /// <p>Information about the standby WorkSpace to be created.</p>
    pub fn set_standby_workspaces(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::StandbyWorkspace>>,
    ) -> Self {
        self.inner = self.inner.set_standby_workspaces(input);
        self
    }
}
