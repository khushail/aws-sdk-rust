// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_capacity_provider::_delete_capacity_provider_output::DeleteCapacityProviderOutputBuilder;

pub use crate::operation::delete_capacity_provider::_delete_capacity_provider_input::DeleteCapacityProviderInputBuilder;

/// Fluent builder constructing a request to `DeleteCapacityProvider`.
///
/// <p>Deletes the specified capacity provider.</p> <note>
/// <p>The <code>FARGATE</code> and <code>FARGATE_SPOT</code> capacity providers are reserved and can't be deleted. You can disassociate them from a cluster using either the <code>PutClusterCapacityProviders</code> API or by deleting the cluster.</p>
/// </note>
/// <p>Prior to a capacity provider being deleted, the capacity provider must be removed from the capacity provider strategy from all services. The <code>UpdateService</code> API can be used to remove a capacity provider from a service's capacity provider strategy. When updating a service, the <code>forceNewDeployment</code> option can be used to ensure that any tasks using the Amazon EC2 instance capacity provided by the capacity provider are transitioned to use the capacity from the remaining capacity providers. Only capacity providers that aren't associated with a cluster can be deleted. To remove a capacity provider from a cluster, you can either use <code>PutClusterCapacityProviders</code> or delete the cluster.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteCapacityProviderFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_capacity_provider::builders::DeleteCapacityProviderInputBuilder,
}
impl DeleteCapacityProviderFluentBuilder {
    /// Creates a new `DeleteCapacityProvider`.
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
            crate::operation::delete_capacity_provider::DeleteCapacityProvider,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_capacity_provider::DeleteCapacityProviderError,
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
        crate::operation::delete_capacity_provider::DeleteCapacityProviderOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_capacity_provider::DeleteCapacityProviderError,
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
        crate::operation::delete_capacity_provider::DeleteCapacityProviderOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_capacity_provider::DeleteCapacityProviderError,
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
            crate::operation::delete_capacity_provider::DeleteCapacityProvider,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_capacity_provider::DeleteCapacityProviderError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the capacity provider to delete.</p>
    pub fn capacity_provider(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.capacity_provider(input.into());
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the capacity provider to delete.</p>
    pub fn set_capacity_provider(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_capacity_provider(input);
        self
    }
}
