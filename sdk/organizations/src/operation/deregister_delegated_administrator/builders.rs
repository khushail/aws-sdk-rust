// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deregister_delegated_administrator::_deregister_delegated_administrator_output::DeregisterDelegatedAdministratorOutputBuilder;

pub use crate::operation::deregister_delegated_administrator::_deregister_delegated_administrator_input::DeregisterDelegatedAdministratorInputBuilder;

/// Fluent builder constructing a request to `DeregisterDelegatedAdministrator`.
///
/// <p>Removes the specified member Amazon Web Services account as a delegated administrator for the specified Amazon Web Services service.</p> <important>
/// <p>Deregistering a delegated administrator can have unintended impacts on the functionality of the enabled Amazon Web Services service. See the documentation for the enabled service before you deregister a delegated administrator so that you understand any potential impacts.</p>
/// </important>
/// <p>You can run this action only for Amazon Web Services services that support this feature. For a current list of services that support it, see the column <i>Supports Delegated Administrator</i> in the table at <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services_list.html">Amazon Web Services Services that you can use with Organizations</a> in the <i>Organizations User Guide.</i> </p>
/// <p>This operation can be called only from the organization's management account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeregisterDelegatedAdministratorFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::deregister_delegated_administrator::builders::DeregisterDelegatedAdministratorInputBuilder,
}
impl DeregisterDelegatedAdministratorFluentBuilder {
    /// Creates a new `DeregisterDelegatedAdministrator`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::deregister_delegated_administrator::DeregisterDelegatedAdministrator, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::deregister_delegated_administrator::DeregisterDelegatedAdministratorError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::deregister_delegated_administrator::DeregisterDelegatedAdministratorOutput, ::aws_smithy_http::result::SdkError<crate::operation::deregister_delegated_administrator::DeregisterDelegatedAdministratorError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::deregister_delegated_administrator::DeregisterDelegatedAdministratorOutput, ::aws_smithy_http::result::SdkError<crate::operation::deregister_delegated_administrator::DeregisterDelegatedAdministratorError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::deregister_delegated_administrator::DeregisterDelegatedAdministrator, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::deregister_delegated_administrator::DeregisterDelegatedAdministratorError>
    >{
        self.customize_middleware().await
    }
    /// <p>The account ID number of the member account in the organization that you want to deregister as a delegated administrator.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The account ID number of the member account in the organization that you want to deregister as a delegated administrator.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The service principal name of an Amazon Web Services service for which the account is a delegated administrator.</p>
    /// <p>Delegated administrator privileges are revoked for only the specified Amazon Web Services service from the member account. If the specified service is the only service for which the member account is a delegated administrator, the operation also revokes Organizations read action permissions.</p>
    pub fn service_principal(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.service_principal(input.into());
        self
    }
    /// <p>The service principal name of an Amazon Web Services service for which the account is a delegated administrator.</p>
    /// <p>Delegated administrator privileges are revoked for only the specified Amazon Web Services service from the member account. If the specified service is the only service for which the member account is a delegated administrator, the operation also revokes Organizations read action permissions.</p>
    pub fn set_service_principal(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_service_principal(input);
        self
    }
}
