// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container element that identifies who initiated the multipart upload. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Initiator {
    /// <p>If the principal is an Amazon Web Services account, it provides the Canonical User ID. If the principal is an IAM User, it provides a user ARN value.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>Name of the Principal.</p>
    #[doc(hidden)]
    pub display_name: ::std::option::Option<::std::string::String>,
}
impl Initiator {
    /// <p>If the principal is an Amazon Web Services account, it provides the Canonical User ID. If the principal is an IAM User, it provides a user ARN value.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>Name of the Principal.</p>
    pub fn display_name(&self) -> ::std::option::Option<&str> {
        self.display_name.as_deref()
    }
}
impl Initiator {
    /// Creates a new builder-style object to manufacture [`Initiator`](crate::types::Initiator).
    pub fn builder() -> crate::types::builders::InitiatorBuilder {
        crate::types::builders::InitiatorBuilder::default()
    }
}

/// A builder for [`Initiator`](crate::types::Initiator).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct InitiatorBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) display_name: ::std::option::Option<::std::string::String>,
}
impl InitiatorBuilder {
    /// <p>If the principal is an Amazon Web Services account, it provides the Canonical User ID. If the principal is an IAM User, it provides a user ARN value.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If the principal is an Amazon Web Services account, it provides the Canonical User ID. If the principal is an IAM User, it provides a user ARN value.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>Name of the Principal.</p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.display_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of the Principal.</p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.display_name = input;
        self
    }
    /// Consumes the builder and constructs a [`Initiator`](crate::types::Initiator).
    pub fn build(self) -> crate::types::Initiator {
        crate::types::Initiator {
            id: self.id,
            display_name: self.display_name,
        }
    }
}
