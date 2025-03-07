// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Identifiers for the federated user that is associated with the credentials.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FederatedUser {
    /// <p>The string that identifies the federated user associated with the credentials, similar to the unique ID of an IAM user.</p>
    #[doc(hidden)]
    pub federated_user_id: ::std::option::Option<::std::string::String>,
    /// <p>The ARN that specifies the federated user that is associated with the credentials. For more information about ARNs and how to use them in policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM Identifiers</a> in the <i>IAM User Guide</i>. </p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
}
impl FederatedUser {
    /// <p>The string that identifies the federated user associated with the credentials, similar to the unique ID of an IAM user.</p>
    pub fn federated_user_id(&self) -> ::std::option::Option<&str> {
        self.federated_user_id.as_deref()
    }
    /// <p>The ARN that specifies the federated user that is associated with the credentials. For more information about ARNs and how to use them in policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM Identifiers</a> in the <i>IAM User Guide</i>. </p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
}
impl FederatedUser {
    /// Creates a new builder-style object to manufacture [`FederatedUser`](crate::types::FederatedUser).
    pub fn builder() -> crate::types::builders::FederatedUserBuilder {
        crate::types::builders::FederatedUserBuilder::default()
    }
}

/// A builder for [`FederatedUser`](crate::types::FederatedUser).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FederatedUserBuilder {
    pub(crate) federated_user_id: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
}
impl FederatedUserBuilder {
    /// <p>The string that identifies the federated user associated with the credentials, similar to the unique ID of an IAM user.</p>
    pub fn federated_user_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.federated_user_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The string that identifies the federated user associated with the credentials, similar to the unique ID of an IAM user.</p>
    pub fn set_federated_user_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.federated_user_id = input;
        self
    }
    /// <p>The ARN that specifies the federated user that is associated with the credentials. For more information about ARNs and how to use them in policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM Identifiers</a> in the <i>IAM User Guide</i>. </p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN that specifies the federated user that is associated with the credentials. For more information about ARNs and how to use them in policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM Identifiers</a> in the <i>IAM User Guide</i>. </p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Consumes the builder and constructs a [`FederatedUser`](crate::types::FederatedUser).
    pub fn build(self) -> crate::types::FederatedUser {
        crate::types::FederatedUser {
            federated_user_id: self.federated_user_id,
            arn: self.arn,
        }
    }
}
