// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DetachVerifiedAccessTrustProviderInput {
    /// <p>The ID of the Verified Access instance.</p>
    #[doc(hidden)]
    pub verified_access_instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Verified Access trust provider.</p>
    #[doc(hidden)]
    pub verified_access_trust_provider_id: ::std::option::Option<::std::string::String>,
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
}
impl DetachVerifiedAccessTrustProviderInput {
    /// <p>The ID of the Verified Access instance.</p>
    pub fn verified_access_instance_id(&self) -> ::std::option::Option<&str> {
        self.verified_access_instance_id.as_deref()
    }
    /// <p>The ID of the Verified Access trust provider.</p>
    pub fn verified_access_trust_provider_id(&self) -> ::std::option::Option<&str> {
        self.verified_access_trust_provider_id.as_deref()
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl DetachVerifiedAccessTrustProviderInput {
    /// Creates a new builder-style object to manufacture [`DetachVerifiedAccessTrustProviderInput`](crate::operation::detach_verified_access_trust_provider::DetachVerifiedAccessTrustProviderInput).
    pub fn builder() -> crate::operation::detach_verified_access_trust_provider::builders::DetachVerifiedAccessTrustProviderInputBuilder{
        crate::operation::detach_verified_access_trust_provider::builders::DetachVerifiedAccessTrustProviderInputBuilder::default()
    }
}

/// A builder for [`DetachVerifiedAccessTrustProviderInput`](crate::operation::detach_verified_access_trust_provider::DetachVerifiedAccessTrustProviderInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DetachVerifiedAccessTrustProviderInputBuilder {
    pub(crate) verified_access_instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) verified_access_trust_provider_id: ::std::option::Option<::std::string::String>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl DetachVerifiedAccessTrustProviderInputBuilder {
    /// <p>The ID of the Verified Access instance.</p>
    pub fn verified_access_instance_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.verified_access_instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Verified Access instance.</p>
    pub fn set_verified_access_instance_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.verified_access_instance_id = input;
        self
    }
    /// <p>The ID of the Verified Access trust provider.</p>
    pub fn verified_access_trust_provider_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.verified_access_trust_provider_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Verified Access trust provider.</p>
    pub fn set_verified_access_trust_provider_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.verified_access_trust_provider_id = input;
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`DetachVerifiedAccessTrustProviderInput`](crate::operation::detach_verified_access_trust_provider::DetachVerifiedAccessTrustProviderInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::detach_verified_access_trust_provider::DetachVerifiedAccessTrustProviderInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::detach_verified_access_trust_provider::DetachVerifiedAccessTrustProviderInput {
                verified_access_instance_id: self.verified_access_instance_id
                ,
                verified_access_trust_provider_id: self.verified_access_trust_provider_id
                ,
                client_token: self.client_token
                ,
                dry_run: self.dry_run
                ,
            }
        )
    }
}
