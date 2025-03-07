// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the data needed by RDP clients such as the Microsoft Remote Desktop Connection to log in to the instance.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TemporaryCredential {
    /// <p>The user name.</p>
    #[doc(hidden)]
    pub username: ::std::option::Option<::std::string::String>,
    /// <p>The password.</p>
    #[doc(hidden)]
    pub password: ::std::option::Option<::std::string::String>,
    /// <p>The length of time (in minutes) that the grant is valid. When the grant expires, at the end of this period, the user will no longer be able to use the credentials to log in. If they are logged in at the time, they will be automatically logged out.</p>
    #[doc(hidden)]
    pub valid_for_in_minutes: ::std::option::Option<i32>,
    /// <p>The instance's AWS OpsWorks Stacks ID.</p>
    #[doc(hidden)]
    pub instance_id: ::std::option::Option<::std::string::String>,
}
impl TemporaryCredential {
    /// <p>The user name.</p>
    pub fn username(&self) -> ::std::option::Option<&str> {
        self.username.as_deref()
    }
    /// <p>The password.</p>
    pub fn password(&self) -> ::std::option::Option<&str> {
        self.password.as_deref()
    }
    /// <p>The length of time (in minutes) that the grant is valid. When the grant expires, at the end of this period, the user will no longer be able to use the credentials to log in. If they are logged in at the time, they will be automatically logged out.</p>
    pub fn valid_for_in_minutes(&self) -> ::std::option::Option<i32> {
        self.valid_for_in_minutes
    }
    /// <p>The instance's AWS OpsWorks Stacks ID.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
}
impl TemporaryCredential {
    /// Creates a new builder-style object to manufacture [`TemporaryCredential`](crate::types::TemporaryCredential).
    pub fn builder() -> crate::types::builders::TemporaryCredentialBuilder {
        crate::types::builders::TemporaryCredentialBuilder::default()
    }
}

/// A builder for [`TemporaryCredential`](crate::types::TemporaryCredential).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TemporaryCredentialBuilder {
    pub(crate) username: ::std::option::Option<::std::string::String>,
    pub(crate) password: ::std::option::Option<::std::string::String>,
    pub(crate) valid_for_in_minutes: ::std::option::Option<i32>,
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
}
impl TemporaryCredentialBuilder {
    /// <p>The user name.</p>
    pub fn username(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.username = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user name.</p>
    pub fn set_username(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.username = input;
        self
    }
    /// <p>The password.</p>
    pub fn password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.password = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The password.</p>
    pub fn set_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.password = input;
        self
    }
    /// <p>The length of time (in minutes) that the grant is valid. When the grant expires, at the end of this period, the user will no longer be able to use the credentials to log in. If they are logged in at the time, they will be automatically logged out.</p>
    pub fn valid_for_in_minutes(mut self, input: i32) -> Self {
        self.valid_for_in_minutes = ::std::option::Option::Some(input);
        self
    }
    /// <p>The length of time (in minutes) that the grant is valid. When the grant expires, at the end of this period, the user will no longer be able to use the credentials to log in. If they are logged in at the time, they will be automatically logged out.</p>
    pub fn set_valid_for_in_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.valid_for_in_minutes = input;
        self
    }
    /// <p>The instance's AWS OpsWorks Stacks ID.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The instance's AWS OpsWorks Stacks ID.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// Consumes the builder and constructs a [`TemporaryCredential`](crate::types::TemporaryCredential).
    pub fn build(self) -> crate::types::TemporaryCredential {
        crate::types::TemporaryCredential {
            username: self.username,
            password: self.password,
            valid_for_in_minutes: self.valid_for_in_minutes,
            instance_id: self.instance_id,
        }
    }
}
