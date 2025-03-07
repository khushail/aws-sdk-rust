// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_trust::_create_trust_output::CreateTrustOutputBuilder;

pub use crate::operation::create_trust::_create_trust_input::CreateTrustInputBuilder;

/// Fluent builder constructing a request to `CreateTrust`.
///
/// <p>Directory Service for Microsoft Active Directory allows you to configure trust relationships. For example, you can establish a trust between your Managed Microsoft AD directory, and your existing self-managed Microsoft Active Directory. This would allow you to provide users and groups access to resources in either domain, with a single set of credentials.</p>
/// <p>This action initiates the creation of the Amazon Web Services side of a trust relationship between an Managed Microsoft AD directory and an external domain. You can create either a forest trust or an external trust.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateTrustFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_trust::builders::CreateTrustInputBuilder,
}
impl CreateTrustFluentBuilder {
    /// Creates a new `CreateTrust`.
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
            crate::operation::create_trust::CreateTrust,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_trust::CreateTrustError>,
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
        crate::operation::create_trust::CreateTrustOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_trust::CreateTrustError>,
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
        crate::operation::create_trust::CreateTrustOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_trust::CreateTrustError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_trust::CreateTrust,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_trust::CreateTrustError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The Directory ID of the Managed Microsoft AD directory for which to establish the trust relationship.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The Directory ID of the Managed Microsoft AD directory for which to establish the trust relationship.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// <p>The Fully Qualified Domain Name (FQDN) of the external domain for which to create the trust relationship.</p>
    pub fn remote_domain_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.remote_domain_name(input.into());
        self
    }
    /// <p>The Fully Qualified Domain Name (FQDN) of the external domain for which to create the trust relationship.</p>
    pub fn set_remote_domain_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_remote_domain_name(input);
        self
    }
    /// <p>The trust password. The must be the same password that was used when creating the trust relationship on the external domain.</p>
    pub fn trust_password(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.trust_password(input.into());
        self
    }
    /// <p>The trust password. The must be the same password that was used when creating the trust relationship on the external domain.</p>
    pub fn set_trust_password(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_trust_password(input);
        self
    }
    /// <p>The direction of the trust relationship.</p>
    pub fn trust_direction(mut self, input: crate::types::TrustDirection) -> Self {
        self.inner = self.inner.trust_direction(input);
        self
    }
    /// <p>The direction of the trust relationship.</p>
    pub fn set_trust_direction(
        mut self,
        input: ::std::option::Option<crate::types::TrustDirection>,
    ) -> Self {
        self.inner = self.inner.set_trust_direction(input);
        self
    }
    /// <p>The trust relationship type. <code>Forest</code> is the default.</p>
    pub fn trust_type(mut self, input: crate::types::TrustType) -> Self {
        self.inner = self.inner.trust_type(input);
        self
    }
    /// <p>The trust relationship type. <code>Forest</code> is the default.</p>
    pub fn set_trust_type(mut self, input: ::std::option::Option<crate::types::TrustType>) -> Self {
        self.inner = self.inner.set_trust_type(input);
        self
    }
    /// Appends an item to `ConditionalForwarderIpAddrs`.
    ///
    /// To override the contents of this collection use [`set_conditional_forwarder_ip_addrs`](Self::set_conditional_forwarder_ip_addrs).
    ///
    /// <p>The IP addresses of the remote DNS server associated with RemoteDomainName.</p>
    pub fn conditional_forwarder_ip_addrs(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.conditional_forwarder_ip_addrs(input.into());
        self
    }
    /// <p>The IP addresses of the remote DNS server associated with RemoteDomainName.</p>
    pub fn set_conditional_forwarder_ip_addrs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_conditional_forwarder_ip_addrs(input);
        self
    }
    /// <p>Optional parameter to enable selective authentication for the trust.</p>
    pub fn selective_auth(mut self, input: crate::types::SelectiveAuth) -> Self {
        self.inner = self.inner.selective_auth(input);
        self
    }
    /// <p>Optional parameter to enable selective authentication for the trust.</p>
    pub fn set_selective_auth(
        mut self,
        input: ::std::option::Option<crate::types::SelectiveAuth>,
    ) -> Self {
        self.inner = self.inner.set_selective_auth(input);
        self
    }
}
