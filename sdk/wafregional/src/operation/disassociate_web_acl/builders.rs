// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_web_acl::_disassociate_web_acl_output::DisassociateWebAclOutputBuilder;

pub use crate::operation::disassociate_web_acl::_disassociate_web_acl_input::DisassociateWebAclInputBuilder;

/// Fluent builder constructing a request to `DisassociateWebACL`.
///
/// <note>
/// <p>This is <b>AWS WAF Classic Regional</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p>
/// <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p>
/// </note>
/// <p>Removes a web ACL from the specified resource, either an application load balancer or Amazon API Gateway stage.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateWebACLFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_web_acl::builders::DisassociateWebAclInputBuilder,
}
impl DisassociateWebACLFluentBuilder {
    /// Creates a new `DisassociateWebACL`.
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
            crate::operation::disassociate_web_acl::DisassociateWebACL,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_web_acl::DisassociateWebACLError,
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
        crate::operation::disassociate_web_acl::DisassociateWebAclOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_web_acl::DisassociateWebACLError,
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
        crate::operation::disassociate_web_acl::DisassociateWebAclOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_web_acl::DisassociateWebACLError,
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
            crate::operation::disassociate_web_acl::DisassociateWebACL,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_web_acl::DisassociateWebACLError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ARN (Amazon Resource Name) of the resource from which the web ACL is being removed, either an application load balancer or Amazon API Gateway stage.</p>
    /// <p>The ARN should be in one of the following formats:</p>
    /// <ul>
    /// <li> <p>For an Application Load Balancer: <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/app/<i>load-balancer-name</i>/<i>load-balancer-id</i> </code> </p> </li>
    /// <li> <p>For an Amazon API Gateway stage: <code>arn:aws:apigateway:<i>region</i>::/restapis/<i>api-id</i>/stages/<i>stage-name</i> </code> </p> </li>
    /// </ul>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The ARN (Amazon Resource Name) of the resource from which the web ACL is being removed, either an application load balancer or Amazon API Gateway stage.</p>
    /// <p>The ARN should be in one of the following formats:</p>
    /// <ul>
    /// <li> <p>For an Application Load Balancer: <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/app/<i>load-balancer-name</i>/<i>load-balancer-id</i> </code> </p> </li>
    /// <li> <p>For an Amazon API Gateway stage: <code>arn:aws:apigateway:<i>region</i>::/restapis/<i>api-id</i>/stages/<i>stage-name</i> </code> </p> </li>
    /// </ul>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
}
