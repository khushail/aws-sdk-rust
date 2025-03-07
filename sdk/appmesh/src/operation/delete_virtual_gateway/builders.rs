// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_virtual_gateway::_delete_virtual_gateway_output::DeleteVirtualGatewayOutputBuilder;

pub use crate::operation::delete_virtual_gateway::_delete_virtual_gateway_input::DeleteVirtualGatewayInputBuilder;

/// Fluent builder constructing a request to `DeleteVirtualGateway`.
///
/// <p>Deletes an existing virtual gateway. You cannot delete a virtual gateway if any gateway routes are associated to it.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteVirtualGatewayFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_virtual_gateway::builders::DeleteVirtualGatewayInputBuilder,
}
impl DeleteVirtualGatewayFluentBuilder {
    /// Creates a new `DeleteVirtualGateway`.
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
            crate::operation::delete_virtual_gateway::DeleteVirtualGateway,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_virtual_gateway::DeleteVirtualGatewayError,
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
        crate::operation::delete_virtual_gateway::DeleteVirtualGatewayOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_virtual_gateway::DeleteVirtualGatewayError,
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
        crate::operation::delete_virtual_gateway::DeleteVirtualGatewayOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_virtual_gateway::DeleteVirtualGatewayError,
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
            crate::operation::delete_virtual_gateway::DeleteVirtualGateway,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_virtual_gateway::DeleteVirtualGatewayError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the virtual gateway to delete.</p>
    pub fn virtual_gateway_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.virtual_gateway_name(input.into());
        self
    }
    /// <p>The name of the virtual gateway to delete.</p>
    pub fn set_virtual_gateway_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_virtual_gateway_name(input);
        self
    }
    /// <p>The name of the service mesh to delete the virtual gateway from.</p>
    pub fn mesh_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.mesh_name(input.into());
        self
    }
    /// <p>The name of the service mesh to delete the virtual gateway from.</p>
    pub fn set_mesh_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_mesh_name(input);
        self
    }
    /// <p>The Amazon Web Services IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    pub fn mesh_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.mesh_owner(input.into());
        self
    }
    /// <p>The Amazon Web Services IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    pub fn set_mesh_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_mesh_owner(input);
        self
    }
}
