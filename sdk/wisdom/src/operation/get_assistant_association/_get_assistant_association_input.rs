// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAssistantAssociationInput {
    /// <p>The identifier of the assistant association. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    #[doc(hidden)]
    pub assistant_association_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the Wisdom assistant. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    #[doc(hidden)]
    pub assistant_id: ::std::option::Option<::std::string::String>,
}
impl GetAssistantAssociationInput {
    /// <p>The identifier of the assistant association. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn assistant_association_id(&self) -> ::std::option::Option<&str> {
        self.assistant_association_id.as_deref()
    }
    /// <p>The identifier of the Wisdom assistant. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn assistant_id(&self) -> ::std::option::Option<&str> {
        self.assistant_id.as_deref()
    }
}
impl GetAssistantAssociationInput {
    /// Creates a new builder-style object to manufacture [`GetAssistantAssociationInput`](crate::operation::get_assistant_association::GetAssistantAssociationInput).
    pub fn builder(
    ) -> crate::operation::get_assistant_association::builders::GetAssistantAssociationInputBuilder
    {
        crate::operation::get_assistant_association::builders::GetAssistantAssociationInputBuilder::default()
    }
}

/// A builder for [`GetAssistantAssociationInput`](crate::operation::get_assistant_association::GetAssistantAssociationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetAssistantAssociationInputBuilder {
    pub(crate) assistant_association_id: ::std::option::Option<::std::string::String>,
    pub(crate) assistant_id: ::std::option::Option<::std::string::String>,
}
impl GetAssistantAssociationInputBuilder {
    /// <p>The identifier of the assistant association. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn assistant_association_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.assistant_association_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the assistant association. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn set_assistant_association_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.assistant_association_id = input;
        self
    }
    /// <p>The identifier of the Wisdom assistant. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn assistant_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.assistant_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the Wisdom assistant. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn set_assistant_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.assistant_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetAssistantAssociationInput`](crate::operation::get_assistant_association::GetAssistantAssociationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_assistant_association::GetAssistantAssociationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_assistant_association::GetAssistantAssociationInput {
                assistant_association_id: self.assistant_association_id,
                assistant_id: self.assistant_id,
            },
        )
    }
}
