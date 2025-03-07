// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A studio member is an association of a user from your studio identity source to elevated permissions that they are granted in the studio.</p>
/// <p>When you add a user to your studio using the Nimble Studio console, they are given access to the studio's IAM Identity Center application and are given access to log in to the Nimble Studio portal. These users have the permissions provided by the studio's user IAM role and do not appear in the studio membership collection. Only studio admins appear in studio membership.</p>
/// <p>When you add a user to studio membership with the ADMIN persona, upon logging in to the Nimble Studio portal, they are granted permissions specified by the Studio's Admin IAM role.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StudioMembership {
    /// <p>The ID of the identity store.</p>
    #[doc(hidden)]
    pub identity_store_id: ::std::option::Option<::std::string::String>,
    /// <p>The persona.</p>
    #[doc(hidden)]
    pub persona: ::std::option::Option<crate::types::StudioPersona>,
    /// <p>The principal ID.</p>
    #[doc(hidden)]
    pub principal_id: ::std::option::Option<::std::string::String>,
    /// <p>The Active Directory Security Identifier for this user, if available.</p>
    #[doc(hidden)]
    pub sid: ::std::option::Option<::std::string::String>,
}
impl StudioMembership {
    /// <p>The ID of the identity store.</p>
    pub fn identity_store_id(&self) -> ::std::option::Option<&str> {
        self.identity_store_id.as_deref()
    }
    /// <p>The persona.</p>
    pub fn persona(&self) -> ::std::option::Option<&crate::types::StudioPersona> {
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
impl StudioMembership {
    /// Creates a new builder-style object to manufacture [`StudioMembership`](crate::types::StudioMembership).
    pub fn builder() -> crate::types::builders::StudioMembershipBuilder {
        crate::types::builders::StudioMembershipBuilder::default()
    }
}

/// A builder for [`StudioMembership`](crate::types::StudioMembership).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StudioMembershipBuilder {
    pub(crate) identity_store_id: ::std::option::Option<::std::string::String>,
    pub(crate) persona: ::std::option::Option<crate::types::StudioPersona>,
    pub(crate) principal_id: ::std::option::Option<::std::string::String>,
    pub(crate) sid: ::std::option::Option<::std::string::String>,
}
impl StudioMembershipBuilder {
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
    pub fn persona(mut self, input: crate::types::StudioPersona) -> Self {
        self.persona = ::std::option::Option::Some(input);
        self
    }
    /// <p>The persona.</p>
    pub fn set_persona(
        mut self,
        input: ::std::option::Option<crate::types::StudioPersona>,
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
    /// Consumes the builder and constructs a [`StudioMembership`](crate::types::StudioMembership).
    pub fn build(self) -> crate::types::StudioMembership {
        crate::types::StudioMembership {
            identity_store_id: self.identity_store_id,
            persona: self.persona,
            principal_id: self.principal_id,
            sid: self.sid,
        }
    }
}
