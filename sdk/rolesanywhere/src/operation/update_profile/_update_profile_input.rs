// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateProfileInput {
    /// <p>The unique identifier of the profile.</p>
    #[doc(hidden)]
    pub profile_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the profile.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>A session policy that applies to the trust boundary of the vended session credentials. </p>
    #[doc(hidden)]
    pub session_policy: ::std::option::Option<::std::string::String>,
    /// <p>A list of IAM roles that this profile can assume in a temporary credential request.</p>
    #[doc(hidden)]
    pub role_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A list of managed policy ARNs that apply to the vended session credentials. </p>
    #[doc(hidden)]
    pub managed_policy_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p> The number of seconds the vended session credentials are valid for. </p>
    #[doc(hidden)]
    pub duration_seconds: ::std::option::Option<i32>,
}
impl UpdateProfileInput {
    /// <p>The unique identifier of the profile.</p>
    pub fn profile_id(&self) -> ::std::option::Option<&str> {
        self.profile_id.as_deref()
    }
    /// <p>The name of the profile.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>A session policy that applies to the trust boundary of the vended session credentials. </p>
    pub fn session_policy(&self) -> ::std::option::Option<&str> {
        self.session_policy.as_deref()
    }
    /// <p>A list of IAM roles that this profile can assume in a temporary credential request.</p>
    pub fn role_arns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.role_arns.as_deref()
    }
    /// <p>A list of managed policy ARNs that apply to the vended session credentials. </p>
    pub fn managed_policy_arns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.managed_policy_arns.as_deref()
    }
    /// <p> The number of seconds the vended session credentials are valid for. </p>
    pub fn duration_seconds(&self) -> ::std::option::Option<i32> {
        self.duration_seconds
    }
}
impl UpdateProfileInput {
    /// Creates a new builder-style object to manufacture [`UpdateProfileInput`](crate::operation::update_profile::UpdateProfileInput).
    pub fn builder() -> crate::operation::update_profile::builders::UpdateProfileInputBuilder {
        crate::operation::update_profile::builders::UpdateProfileInputBuilder::default()
    }
}

/// A builder for [`UpdateProfileInput`](crate::operation::update_profile::UpdateProfileInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateProfileInputBuilder {
    pub(crate) profile_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) session_policy: ::std::option::Option<::std::string::String>,
    pub(crate) role_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) managed_policy_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) duration_seconds: ::std::option::Option<i32>,
}
impl UpdateProfileInputBuilder {
    /// <p>The unique identifier of the profile.</p>
    pub fn profile_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.profile_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the profile.</p>
    pub fn set_profile_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.profile_id = input;
        self
    }
    /// <p>The name of the profile.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the profile.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>A session policy that applies to the trust boundary of the vended session credentials. </p>
    pub fn session_policy(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.session_policy = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A session policy that applies to the trust boundary of the vended session credentials. </p>
    pub fn set_session_policy(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.session_policy = input;
        self
    }
    /// Appends an item to `role_arns`.
    ///
    /// To override the contents of this collection use [`set_role_arns`](Self::set_role_arns).
    ///
    /// <p>A list of IAM roles that this profile can assume in a temporary credential request.</p>
    pub fn role_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.role_arns.unwrap_or_default();
        v.push(input.into());
        self.role_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of IAM roles that this profile can assume in a temporary credential request.</p>
    pub fn set_role_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.role_arns = input;
        self
    }
    /// Appends an item to `managed_policy_arns`.
    ///
    /// To override the contents of this collection use [`set_managed_policy_arns`](Self::set_managed_policy_arns).
    ///
    /// <p>A list of managed policy ARNs that apply to the vended session credentials. </p>
    pub fn managed_policy_arns(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.managed_policy_arns.unwrap_or_default();
        v.push(input.into());
        self.managed_policy_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of managed policy ARNs that apply to the vended session credentials. </p>
    pub fn set_managed_policy_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.managed_policy_arns = input;
        self
    }
    /// <p> The number of seconds the vended session credentials are valid for. </p>
    pub fn duration_seconds(mut self, input: i32) -> Self {
        self.duration_seconds = ::std::option::Option::Some(input);
        self
    }
    /// <p> The number of seconds the vended session credentials are valid for. </p>
    pub fn set_duration_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.duration_seconds = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateProfileInput`](crate::operation::update_profile::UpdateProfileInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_profile::UpdateProfileInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_profile::UpdateProfileInput {
            profile_id: self.profile_id,
            name: self.name,
            session_policy: self.session_policy,
            role_arns: self.role_arns,
            managed_policy_arns: self.managed_policy_arns,
            duration_seconds: self.duration_seconds,
        })
    }
}
