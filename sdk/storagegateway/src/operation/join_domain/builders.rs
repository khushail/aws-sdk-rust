// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::join_domain::_join_domain_output::JoinDomainOutputBuilder;

pub use crate::operation::join_domain::_join_domain_input::JoinDomainInputBuilder;

/// Fluent builder constructing a request to `JoinDomain`.
///
/// <p>Adds a file gateway to an Active Directory domain. This operation is only supported for file gateways that support the SMB file protocol.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct JoinDomainFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::join_domain::builders::JoinDomainInputBuilder,
}
impl JoinDomainFluentBuilder {
    /// Creates a new `JoinDomain`.
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
            crate::operation::join_domain::JoinDomain,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::join_domain::JoinDomainError>,
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
        crate::operation::join_domain::JoinDomainOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::join_domain::JoinDomainError>,
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
        crate::operation::join_domain::JoinDomainOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::join_domain::JoinDomainError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::join_domain::JoinDomain,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::join_domain::JoinDomainError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn gateway_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.gateway_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn set_gateway_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_gateway_arn(input);
        self
    }
    /// <p>The name of the domain that you want the gateway to join.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The name of the domain that you want the gateway to join.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The organizational unit (OU) is a container in an Active Directory that can hold users, groups, computers, and other OUs and this parameter specifies the OU that the gateway will join within the AD domain.</p>
    pub fn organizational_unit(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.organizational_unit(input.into());
        self
    }
    /// <p>The organizational unit (OU) is a container in an Active Directory that can hold users, groups, computers, and other OUs and this parameter specifies the OU that the gateway will join within the AD domain.</p>
    pub fn set_organizational_unit(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_organizational_unit(input);
        self
    }
    /// Appends an item to `DomainControllers`.
    ///
    /// To override the contents of this collection use [`set_domain_controllers`](Self::set_domain_controllers).
    ///
    /// <p>List of IPv4 addresses, NetBIOS names, or host names of your domain server. If you need to specify the port number include it after the colon (“:”). For example, <code>mydc.mydomain.com:389</code>.</p>
    pub fn domain_controllers(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.domain_controllers(input.into());
        self
    }
    /// <p>List of IPv4 addresses, NetBIOS names, or host names of your domain server. If you need to specify the port number include it after the colon (“:”). For example, <code>mydc.mydomain.com:389</code>.</p>
    pub fn set_domain_controllers(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_domain_controllers(input);
        self
    }
    /// <p>Specifies the time in seconds, in which the <code>JoinDomain</code> operation must complete. The default is 20 seconds.</p>
    pub fn timeout_in_seconds(mut self, input: i32) -> Self {
        self.inner = self.inner.timeout_in_seconds(input);
        self
    }
    /// <p>Specifies the time in seconds, in which the <code>JoinDomain</code> operation must complete. The default is 20 seconds.</p>
    pub fn set_timeout_in_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_timeout_in_seconds(input);
        self
    }
    /// <p>Sets the user name of user who has permission to add the gateway to the Active Directory domain. The domain user account should be enabled to join computers to the domain. For example, you can use the domain administrator account or an account with delegated permissions to join computers to the domain.</p>
    pub fn user_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_name(input.into());
        self
    }
    /// <p>Sets the user name of user who has permission to add the gateway to the Active Directory domain. The domain user account should be enabled to join computers to the domain. For example, you can use the domain administrator account or an account with delegated permissions to join computers to the domain.</p>
    pub fn set_user_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_name(input);
        self
    }
    /// <p>Sets the password of the user who has permission to add the gateway to the Active Directory domain.</p>
    pub fn password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.password(input.into());
        self
    }
    /// <p>Sets the password of the user who has permission to add the gateway to the Active Directory domain.</p>
    pub fn set_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_password(input);
        self
    }
}
