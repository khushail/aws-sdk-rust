// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TestAuthorizationInput {
    /// <p>The principal. Valid principals are CertificateArn (arn:aws:iot:<i>region</i>:<i>accountId</i>:cert/<i>certificateId</i>), thingGroupArn (arn:aws:iot:<i>region</i>:<i>accountId</i>:thinggroup/<i>groupName</i>) and CognitoId (<i>region</i>:<i>id</i>).</p>
    #[doc(hidden)]
    pub principal: ::std::option::Option<::std::string::String>,
    /// <p>The Cognito identity pool ID.</p>
    #[doc(hidden)]
    pub cognito_identity_pool_id: ::std::option::Option<::std::string::String>,
    /// <p>A list of authorization info objects. Simulating authorization will create a response for each <code>authInfo</code> object in the list.</p>
    #[doc(hidden)]
    pub auth_infos: ::std::option::Option<::std::vec::Vec<crate::types::AuthInfo>>,
    /// <p>The MQTT client ID.</p>
    #[doc(hidden)]
    pub client_id: ::std::option::Option<::std::string::String>,
    /// <p>When testing custom authorization, the policies specified here are treated as if they are attached to the principal being authorized.</p>
    #[doc(hidden)]
    pub policy_names_to_add: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>When testing custom authorization, the policies specified here are treated as if they are not attached to the principal being authorized.</p>
    #[doc(hidden)]
    pub policy_names_to_skip: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl TestAuthorizationInput {
    /// <p>The principal. Valid principals are CertificateArn (arn:aws:iot:<i>region</i>:<i>accountId</i>:cert/<i>certificateId</i>), thingGroupArn (arn:aws:iot:<i>region</i>:<i>accountId</i>:thinggroup/<i>groupName</i>) and CognitoId (<i>region</i>:<i>id</i>).</p>
    pub fn principal(&self) -> ::std::option::Option<&str> {
        self.principal.as_deref()
    }
    /// <p>The Cognito identity pool ID.</p>
    pub fn cognito_identity_pool_id(&self) -> ::std::option::Option<&str> {
        self.cognito_identity_pool_id.as_deref()
    }
    /// <p>A list of authorization info objects. Simulating authorization will create a response for each <code>authInfo</code> object in the list.</p>
    pub fn auth_infos(&self) -> ::std::option::Option<&[crate::types::AuthInfo]> {
        self.auth_infos.as_deref()
    }
    /// <p>The MQTT client ID.</p>
    pub fn client_id(&self) -> ::std::option::Option<&str> {
        self.client_id.as_deref()
    }
    /// <p>When testing custom authorization, the policies specified here are treated as if they are attached to the principal being authorized.</p>
    pub fn policy_names_to_add(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.policy_names_to_add.as_deref()
    }
    /// <p>When testing custom authorization, the policies specified here are treated as if they are not attached to the principal being authorized.</p>
    pub fn policy_names_to_skip(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.policy_names_to_skip.as_deref()
    }
}
impl TestAuthorizationInput {
    /// Creates a new builder-style object to manufacture [`TestAuthorizationInput`](crate::operation::test_authorization::TestAuthorizationInput).
    pub fn builder() -> crate::operation::test_authorization::builders::TestAuthorizationInputBuilder
    {
        crate::operation::test_authorization::builders::TestAuthorizationInputBuilder::default()
    }
}

/// A builder for [`TestAuthorizationInput`](crate::operation::test_authorization::TestAuthorizationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TestAuthorizationInputBuilder {
    pub(crate) principal: ::std::option::Option<::std::string::String>,
    pub(crate) cognito_identity_pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) auth_infos: ::std::option::Option<::std::vec::Vec<crate::types::AuthInfo>>,
    pub(crate) client_id: ::std::option::Option<::std::string::String>,
    pub(crate) policy_names_to_add: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) policy_names_to_skip: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl TestAuthorizationInputBuilder {
    /// <p>The principal. Valid principals are CertificateArn (arn:aws:iot:<i>region</i>:<i>accountId</i>:cert/<i>certificateId</i>), thingGroupArn (arn:aws:iot:<i>region</i>:<i>accountId</i>:thinggroup/<i>groupName</i>) and CognitoId (<i>region</i>:<i>id</i>).</p>
    pub fn principal(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.principal = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The principal. Valid principals are CertificateArn (arn:aws:iot:<i>region</i>:<i>accountId</i>:cert/<i>certificateId</i>), thingGroupArn (arn:aws:iot:<i>region</i>:<i>accountId</i>:thinggroup/<i>groupName</i>) and CognitoId (<i>region</i>:<i>id</i>).</p>
    pub fn set_principal(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.principal = input;
        self
    }
    /// <p>The Cognito identity pool ID.</p>
    pub fn cognito_identity_pool_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.cognito_identity_pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Cognito identity pool ID.</p>
    pub fn set_cognito_identity_pool_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.cognito_identity_pool_id = input;
        self
    }
    /// Appends an item to `auth_infos`.
    ///
    /// To override the contents of this collection use [`set_auth_infos`](Self::set_auth_infos).
    ///
    /// <p>A list of authorization info objects. Simulating authorization will create a response for each <code>authInfo</code> object in the list.</p>
    pub fn auth_infos(mut self, input: crate::types::AuthInfo) -> Self {
        let mut v = self.auth_infos.unwrap_or_default();
        v.push(input);
        self.auth_infos = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of authorization info objects. Simulating authorization will create a response for each <code>authInfo</code> object in the list.</p>
    pub fn set_auth_infos(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AuthInfo>>,
    ) -> Self {
        self.auth_infos = input;
        self
    }
    /// <p>The MQTT client ID.</p>
    pub fn client_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The MQTT client ID.</p>
    pub fn set_client_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_id = input;
        self
    }
    /// Appends an item to `policy_names_to_add`.
    ///
    /// To override the contents of this collection use [`set_policy_names_to_add`](Self::set_policy_names_to_add).
    ///
    /// <p>When testing custom authorization, the policies specified here are treated as if they are attached to the principal being authorized.</p>
    pub fn policy_names_to_add(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.policy_names_to_add.unwrap_or_default();
        v.push(input.into());
        self.policy_names_to_add = ::std::option::Option::Some(v);
        self
    }
    /// <p>When testing custom authorization, the policies specified here are treated as if they are attached to the principal being authorized.</p>
    pub fn set_policy_names_to_add(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.policy_names_to_add = input;
        self
    }
    /// Appends an item to `policy_names_to_skip`.
    ///
    /// To override the contents of this collection use [`set_policy_names_to_skip`](Self::set_policy_names_to_skip).
    ///
    /// <p>When testing custom authorization, the policies specified here are treated as if they are not attached to the principal being authorized.</p>
    pub fn policy_names_to_skip(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.policy_names_to_skip.unwrap_or_default();
        v.push(input.into());
        self.policy_names_to_skip = ::std::option::Option::Some(v);
        self
    }
    /// <p>When testing custom authorization, the policies specified here are treated as if they are not attached to the principal being authorized.</p>
    pub fn set_policy_names_to_skip(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.policy_names_to_skip = input;
        self
    }
    /// Consumes the builder and constructs a [`TestAuthorizationInput`](crate::operation::test_authorization::TestAuthorizationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::test_authorization::TestAuthorizationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::test_authorization::TestAuthorizationInput {
                principal: self.principal,
                cognito_identity_pool_id: self.cognito_identity_pool_id,
                auth_infos: self.auth_infos,
                client_id: self.client_id,
                policy_names_to_add: self.policy_names_to_add,
                policy_names_to_skip: self.policy_names_to_skip,
            },
        )
    }
}
