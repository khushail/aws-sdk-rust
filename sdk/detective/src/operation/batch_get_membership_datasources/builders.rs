// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_get_membership_datasources::_batch_get_membership_datasources_output::BatchGetMembershipDatasourcesOutputBuilder;

pub use crate::operation::batch_get_membership_datasources::_batch_get_membership_datasources_input::BatchGetMembershipDatasourcesInputBuilder;

/// Fluent builder constructing a request to `BatchGetMembershipDatasources`.
///
/// <p>Gets information on the data source package history for an account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchGetMembershipDatasourcesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::batch_get_membership_datasources::builders::BatchGetMembershipDatasourcesInputBuilder,
}
impl BatchGetMembershipDatasourcesFluentBuilder {
    /// Creates a new `BatchGetMembershipDatasources`.
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
            crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasources,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesError,
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
        crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesError,
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
        crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesError,
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
            crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasources,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Appends an item to `GraphArns`.
    ///
    /// To override the contents of this collection use [`set_graph_arns`](Self::set_graph_arns).
    ///
    /// <p>The ARN of the behavior graph.</p>
    pub fn graph_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.graph_arns(input.into());
        self
    }
    /// <p>The ARN of the behavior graph.</p>
    pub fn set_graph_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_graph_arns(input);
        self
    }
}
