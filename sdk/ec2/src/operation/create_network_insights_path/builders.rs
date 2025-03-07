// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_network_insights_path::_create_network_insights_path_output::CreateNetworkInsightsPathOutputBuilder;

pub use crate::operation::create_network_insights_path::_create_network_insights_path_input::CreateNetworkInsightsPathInputBuilder;

/// Fluent builder constructing a request to `CreateNetworkInsightsPath`.
///
/// <p>Creates a path to analyze for reachability.</p>
/// <p>Reachability Analyzer enables you to analyze and debug network reachability between two resources in your virtual private cloud (VPC). For more information, see the <a href="https://docs.aws.amazon.com/vpc/latest/reachability/">Reachability Analyzer Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateNetworkInsightsPathFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::create_network_insights_path::builders::CreateNetworkInsightsPathInputBuilder,
}
impl CreateNetworkInsightsPathFluentBuilder {
    /// Creates a new `CreateNetworkInsightsPath`.
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
            crate::operation::create_network_insights_path::CreateNetworkInsightsPath,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_network_insights_path::CreateNetworkInsightsPathError,
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
        crate::operation::create_network_insights_path::CreateNetworkInsightsPathOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_network_insights_path::CreateNetworkInsightsPathError,
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
        crate::operation::create_network_insights_path::CreateNetworkInsightsPathOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_network_insights_path::CreateNetworkInsightsPathError,
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
            crate::operation::create_network_insights_path::CreateNetworkInsightsPath,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_network_insights_path::CreateNetworkInsightsPathError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The IP address of the source.</p>
    pub fn source_ip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_ip(input.into());
        self
    }
    /// <p>The IP address of the source.</p>
    pub fn set_source_ip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_ip(input);
        self
    }
    /// <p>The IP address of the destination.</p>
    pub fn destination_ip(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.destination_ip(input.into());
        self
    }
    /// <p>The IP address of the destination.</p>
    pub fn set_destination_ip(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_destination_ip(input);
        self
    }
    /// <p>The ID or ARN of the source. If the resource is in another account, you must specify an ARN.</p>
    pub fn source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source(input.into());
        self
    }
    /// <p>The ID or ARN of the source. If the resource is in another account, you must specify an ARN.</p>
    pub fn set_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source(input);
        self
    }
    /// <p>The ID or ARN of the destination. If the resource is in another account, you must specify an ARN.</p>
    pub fn destination(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.destination(input.into());
        self
    }
    /// <p>The ID or ARN of the destination. If the resource is in another account, you must specify an ARN.</p>
    pub fn set_destination(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_destination(input);
        self
    }
    /// <p>The protocol.</p>
    pub fn protocol(mut self, input: crate::types::Protocol) -> Self {
        self.inner = self.inner.protocol(input);
        self
    }
    /// <p>The protocol.</p>
    pub fn set_protocol(mut self, input: ::std::option::Option<crate::types::Protocol>) -> Self {
        self.inner = self.inner.set_protocol(input);
        self
    }
    /// <p>The destination port.</p>
    pub fn destination_port(mut self, input: i32) -> Self {
        self.inner = self.inner.destination_port(input);
        self
    }
    /// <p>The destination port.</p>
    pub fn set_destination_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_destination_port(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to add to the path.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to add to the path.</p>
    pub fn set_tag_specifications(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the source. If you specify this parameter, you can't specify the parameters for the source IP address or the destination port.</p>
    pub fn filter_at_source(mut self, input: crate::types::PathRequestFilter) -> Self {
        self.inner = self.inner.filter_at_source(input);
        self
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the source. If you specify this parameter, you can't specify the parameters for the source IP address or the destination port.</p>
    pub fn set_filter_at_source(
        mut self,
        input: ::std::option::Option<crate::types::PathRequestFilter>,
    ) -> Self {
        self.inner = self.inner.set_filter_at_source(input);
        self
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the destination. If you specify this parameter, you can't specify the parameter for the destination IP address.</p>
    pub fn filter_at_destination(mut self, input: crate::types::PathRequestFilter) -> Self {
        self.inner = self.inner.filter_at_destination(input);
        self
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the destination. If you specify this parameter, you can't specify the parameter for the destination IP address.</p>
    pub fn set_filter_at_destination(
        mut self,
        input: ::std::option::Option<crate::types::PathRequestFilter>,
    ) -> Self {
        self.inner = self.inner.set_filter_at_destination(input);
        self
    }
}
