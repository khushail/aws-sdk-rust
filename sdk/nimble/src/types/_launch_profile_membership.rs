// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Studio admins can use launch profile membership to delegate launch profile access to studio users in the Nimble Studio portal without writing or maintaining complex IAM policies. A launch profile member is a user association from your studio identity source who is granted permissions to a launch profile.</p>
/// <p>A launch profile member (type USER) provides the following permissions to that launch profile:</p>
/// <ul>
/// <li> <p>GetLaunchProfile</p> </li>
/// <li> <p>GetLaunchProfileInitialization</p> </li>
/// <li> <p>GetLaunchProfileMembers</p> </li>
/// <li> <p>GetLaunchProfileMember</p> </li>
/// <li> <p>CreateStreamingSession</p> </li>
/// <li> <p>GetLaunchProfileDetails</p> </li>
/// </ul>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LaunchProfileMembership {
    /// <p>The ID of the identity store.</p>
    #[doc(hidden)]
    pub identity_store_id: ::std::option::Option<::std::string::String>,
    /// <p>The persona.</p>
    #[doc(hidden)]
    pub persona: ::std::option::Option<crate::types::LaunchProfilePersona>,
    /// <p>The principal ID.</p>
    #[doc(hidden)]
    pub principal_id: ::std::option::Option<::std::string::String>,
    /// <p>The Active Directory Security Identifier for this user, if available.</p>
    #[doc(hidden)]
    pub sid: ::std::option::Option<::std::string::String>,
}
impl LaunchProfileMembership {
    /// <p>The ID of the identity store.</p>
    pub fn identity_store_id(&self) -> ::std::option::Option<&str> {
        self.identity_store_id.as_deref()
    }
    /// <p>The persona.</p>
    pub fn persona(&self) -> ::std::option::Option<&crate::types::LaunchProfilePersona> {
        self.persona.as_ref()
    }
    /// <p>The principal ID.</p>
    pub fn principal_id(&self) -> ::std::option::Option<&str> {
        self.principal_id.as_deref()
    }
    /// <p>The Active Directory Security Identifier for this user, if available.</p>
    pub fn sid(&self) -> ::std::option::Option<&str> {
        self.sid.as_deref()
    }
}
impl LaunchProfileMembership {
    /// Creates a new builder-style object to manufacture [`LaunchProfileMembership`](crate::types::LaunchProfileMembership).
    pub fn builder() -> crate::types::builders::LaunchProfileMembershipBuilder {
        crate::types::builders::LaunchProfileMembershipBuilder::default()
    }
}

/// A builder for [`LaunchProfileMembership`](crate::types::LaunchProfileMembership).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LaunchProfileMembershipBuilder {
    pub(crate) identity_store_id: ::std::option::Option<::std::string::String>,
    pub(crate) persona: ::std::option::Option<crate::types::LaunchProfilePersona>,
    pub(crate) principal_id: ::std::option::Option<::std::string::String>,
    pub(crate) sid: ::std::option::Option<::std::string::String>,
}
impl LaunchProfileMembershipBuilder {
    /// <p>The ID of the identity store.</p>
    pub fn identity_store_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.identity_store_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the identity store.</p>
    pub fn set_identity_store_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.identity_store_id = input;
        self
    }
    /// <p>The persona.</p>
    pub fn persona(mut self, input: crate::types::LaunchProfilePersona) -> Self {
        self.persona = ::std::option::Option::Some(input);
        self
    }
    /// <p>The persona.</p>
    pub fn set_persona(
        mut self,
        input: ::std::option::Option<crate::types::LaunchProfilePersona>,
    ) -> Self {
        self.persona = input;
        self
    }
    /// <p>The principal ID.</p>
    pub fn principal_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.principal_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The principal ID.</p>
    pub fn set_principal_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.principal_id = input;
        self
    }
    /// <p>The Active Directory Security Identifier for this user, if available.</p>
    pub fn sid(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sid = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Active Directory Security Identifier for this user, if available.</p>
    pub fn set_sid(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sid = input;
        self
    }
    /// Consumes the builder and constructs a [`LaunchProfileMembership`](crate::types::LaunchProfileMembership).
    pub fn build(self) -> crate::types::LaunchProfileMembership {
        crate::types::LaunchProfileMembership {
            identity_store_id: self.identity_store_id,
            persona: self.persona,
            principal_id: self.principal_id,
            sid: self.sid,
        }
    }
}
