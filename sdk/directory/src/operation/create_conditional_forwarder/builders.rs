// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_conditional_forwarder::_create_conditional_forwarder_output::CreateConditionalForwarderOutputBuilder;

pub use crate::operation::create_conditional_forwarder::_create_conditional_forwarder_input::CreateConditionalForwarderInputBuilder;

/// Fluent builder constructing a request to `CreateConditionalForwarder`.
///
/// <p>Creates a conditional forwarder associated with your Amazon Web Services directory. Conditional forwarders are required in order to set up a trust relationship with another domain. The conditional forwarder points to the trusted domain.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateConditionalForwarderFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::create_conditional_forwarder::builders::CreateConditionalForwarderInputBuilder,
}
impl CreateConditionalForwarderFluentBuilder {
    /// Creates a new `CreateConditionalForwarder`.
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
            crate::operation::create_conditional_forwarder::CreateConditionalForwarder,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_conditional_forwarder::CreateConditionalForwarderError,
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
        crate::operation::create_conditional_forwarder::CreateConditionalForwarderOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_conditional_forwarder::CreateConditionalForwarderError,
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
        crate::operation::create_conditional_forwarder::CreateConditionalForwarderOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_conditional_forwarder::CreateConditionalForwarderError,
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
            crate::operation::create_conditional_forwarder::CreateConditionalForwarder,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_conditional_forwarder::CreateConditionalForwarderError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The directory ID of the Amazon Web Services directory for which you are creating the conditional forwarder.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The directory ID of the Amazon Web Services directory for which you are creating the conditional forwarder.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// <p>The fully qualified domain name (FQDN) of the remote domain with which you will set up a trust relationship.</p>
    pub fn remote_domain_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.remote_domain_name(input.into());
        self
    }
    /// <p>The fully qualified domain name (FQDN) of the remote domain with which you will set up a trust relationship.</p>
    pub fn set_remote_domain_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_remote_domain_name(input);
        self
    }
    /// Appends an item to `DnsIpAddrs`.
    ///
    /// To override the contents of this collection use [`set_dns_ip_addrs`](Self::set_dns_ip_addrs).
    ///
    /// <p>The IP addresses of the remote DNS server associated with RemoteDomainName.</p>
    pub fn dns_ip_addrs(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dns_ip_addrs(input.into());
        self
    }
    /// <p>The IP addresses of the remote DNS server associated with RemoteDomainName.</p>
    pub fn set_dns_ip_addrs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_dns_ip_addrs(input);
        self
    }
}
