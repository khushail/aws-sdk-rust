// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_ssl_policies::_describe_ssl_policies_output::DescribeSslPoliciesOutputBuilder;

pub use crate::operation::describe_ssl_policies::_describe_ssl_policies_input::DescribeSslPoliciesInputBuilder;

/// Fluent builder constructing a request to `DescribeSSLPolicies`.
///
/// <p>Describes the specified policies or all policies used for SSL negotiation.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/create-https-listener.html#describe-ssl-policies">Security policies</a> in the <i>Application Load Balancers Guide</i> or <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/create-tls-listener.html#describe-ssl-policies">Security policies</a> in the <i>Network Load Balancers Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeSSLPoliciesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_ssl_policies::builders::DescribeSslPoliciesInputBuilder,
}
impl DescribeSSLPoliciesFluentBuilder {
    /// Creates a new `DescribeSSLPolicies`.
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
            crate::operation::describe_ssl_policies::DescribeSSLPolicies,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_ssl_policies::DescribeSSLPoliciesError,
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
        crate::operation::describe_ssl_policies::DescribeSslPoliciesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_ssl_policies::DescribeSSLPoliciesError,
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
        crate::operation::describe_ssl_policies::DescribeSslPoliciesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_ssl_policies::DescribeSSLPoliciesError,
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
            crate::operation::describe_ssl_policies::DescribeSSLPolicies,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_ssl_policies::DescribeSSLPoliciesError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Appends an item to `Names`.
    ///
    /// To override the contents of this collection use [`set_names`](Self::set_names).
    ///
    /// <p>The names of the policies.</p>
    pub fn names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.names(input.into());
        self
    }
    /// <p>The names of the policies.</p>
    pub fn set_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_names(input);
        self
    }
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>The maximum number of results to return with this call.</p>
    pub fn page_size(mut self, input: i32) -> Self {
        self.inner = self.inner.page_size(input);
        self
    }
    /// <p>The maximum number of results to return with this call.</p>
    pub fn set_page_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_page_size(input);
        self
    }
    /// <p> The type of load balancer. The default lists the SSL policies for all load balancers.</p>
    pub fn load_balancer_type(mut self, input: crate::types::LoadBalancerTypeEnum) -> Self {
        self.inner = self.inner.load_balancer_type(input);
        self
    }
    /// <p> The type of load balancer. The default lists the SSL policies for all load balancers.</p>
    pub fn set_load_balancer_type(
        mut self,
        input: ::std::option::Option<crate::types::LoadBalancerTypeEnum>,
    ) -> Self {
        self.inner = self.inner.set_load_balancer_type(input);
        self
    }
}
