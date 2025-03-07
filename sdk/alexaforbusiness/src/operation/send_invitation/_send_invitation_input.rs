// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SendInvitationInput {
    /// <p>The ARN of the user to whom to send an invitation. Required.</p>
    #[doc(hidden)]
    pub user_arn: ::std::option::Option<::std::string::String>,
}
impl SendInvitationInput {
    /// <p>The ARN of the user to whom to send an invitation. Required.</p>
    pub fn user_arn(&self) -> ::std::option::Option<&str> {
        self.user_arn.as_deref()
    }
}
impl SendInvitationInput {
    /// Creates a new builder-style object to manufacture [`SendInvitationInput`](crate::operation::send_invitation::SendInvitationInput).
    pub fn builder() -> crate::operation::send_invitation::builders::SendInvitationInputBuilder {
        crate::operation::send_invitation::builders::SendInvitationInputBuilder::default()
    }
}

/// A builder for [`SendInvitationInput`](crate::operation::send_invitation::SendInvitationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SendInvitationInputBuilder {
    pub(crate) user_arn: ::std::option::Option<::std::string::String>,
}
impl SendInvitationInputBuilder {
    /// <p>The ARN of the user to whom to send an invitation. Required.</p>
    pub fn user_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the user to whom to send an invitation. Required.</p>
    pub fn set_user_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`SendInvitationInput`](crate::operation::send_invitation::SendInvitationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::send_invitation::SendInvitationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::send_invitation::SendInvitationInput {
            user_arn: self.user_arn,
        })
    }
}
