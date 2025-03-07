// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateVerifiedAccessGroupInput {
    /// <p>The ID of the Verified Access instance.</p>
    #[doc(hidden)]
    pub verified_access_instance_id: ::std::option::Option<::std::string::String>,
    /// <p>A description for the Verified Access group.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The Verified Access policy document.</p>
    #[doc(hidden)]
    pub policy_document: ::std::option::Option<::std::string::String>,
    /// <p>The tags to assign to the Verified Access group.</p>
    #[doc(hidden)]
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
}
impl CreateVerifiedAccessGroupInput {
    /// <p>The ID of the Verified Access instance.</p>
    pub fn verified_access_instance_id(&self) -> ::std::option::Option<&str> {
        self.verified_access_instance_id.as_deref()
    }
    /// <p>A description for the Verified Access group.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The Verified Access policy document.</p>
    pub fn policy_document(&self) -> ::std::option::Option<&str> {
        self.policy_document.as_deref()
    }
    /// <p>The tags to assign to the Verified Access group.</p>
    pub fn tag_specifications(&self) -> ::std::option::Option<&[crate::types::TagSpecification]> {
        self.tag_specifications.as_deref()
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
impl CreateVerifiedAccessGroupInput {
    /// Creates a new builder-style object to manufacture [`CreateVerifiedAccessGroupInput`](crate::operation::create_verified_access_group::CreateVerifiedAccessGroupInput).
    pub fn builder() -> crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupInputBuilder{
        crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupInputBuilder::default()
    }
}

/// A builder for [`CreateVerifiedAccessGroupInput`](crate::operation::create_verified_access_group::CreateVerifiedAccessGroupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateVerifiedAccessGroupInputBuilder {
    pub(crate) verified_access_instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) policy_document: ::std::option::Option<::std::string::String>,
    pub(crate) tag_specifications:
        ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl CreateVerifiedAccessGroupInputBuilder {
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
    /// <p>A description for the Verified Access group.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description for the Verified Access group.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The Verified Access policy document.</p>
    pub fn policy_document(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.policy_document = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Verified Access policy document.</p>
    pub fn set_policy_document(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.policy_document = input;
        self
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to assign to the Verified Access group.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to assign to the Verified Access group.</p>
    pub fn set_tag_specifications(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.tag_specifications = input;
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
    /// Consumes the builder and constructs a [`CreateVerifiedAccessGroupInput`](crate::operation::create_verified_access_group::CreateVerifiedAccessGroupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_verified_access_group::CreateVerifiedAccessGroupInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_verified_access_group::CreateVerifiedAccessGroupInput {
                verified_access_instance_id: self.verified_access_instance_id,
                description: self.description,
                policy_document: self.policy_document,
                tag_specifications: self.tag_specifications,
                client_token: self.client_token,
                dry_run: self.dry_run,
            },
        )
    }
}
