// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateParticipantConnectionInput {
    /// <p>Type of connection information required. This can be omitted if <code>ConnectParticipant</code> is <code>true</code>.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<::std::vec::Vec<crate::types::ConnectionType>>,
    /// <p>This is a header parameter.</p>
    /// <p>The ParticipantToken as obtained from <a href="https://docs.aws.amazon.com/connect/latest/APIReference/API_StartChatContact.html">StartChatContact</a> API response.</p>
    #[doc(hidden)]
    pub participant_token: ::std::option::Option<::std::string::String>,
    /// <p>Amazon Connect Participant is used to mark the participant as connected for customer participant in message streaming, as well as for agent or manager participant in non-streaming chats.</p>
    #[doc(hidden)]
    pub connect_participant: ::std::option::Option<bool>,
}
impl CreateParticipantConnectionInput {
    /// <p>Type of connection information required. This can be omitted if <code>ConnectParticipant</code> is <code>true</code>.</p>
    pub fn r#type(&self) -> ::std::option::Option<&[crate::types::ConnectionType]> {
        self.r#type.as_deref()
    }
    /// <p>This is a header parameter.</p>
    /// <p>The ParticipantToken as obtained from <a href="https://docs.aws.amazon.com/connect/latest/APIReference/API_StartChatContact.html">StartChatContact</a> API response.</p>
    pub fn participant_token(&self) -> ::std::option::Option<&str> {
        self.participant_token.as_deref()
    }
    /// <p>Amazon Connect Participant is used to mark the participant as connected for customer participant in message streaming, as well as for agent or manager participant in non-streaming chats.</p>
    pub fn connect_participant(&self) -> ::std::option::Option<bool> {
        self.connect_participant
    }
}
impl CreateParticipantConnectionInput {
    /// Creates a new builder-style object to manufacture [`CreateParticipantConnectionInput`](crate::operation::create_participant_connection::CreateParticipantConnectionInput).
    pub fn builder() -> crate::operation::create_participant_connection::builders::CreateParticipantConnectionInputBuilder{
        crate::operation::create_participant_connection::builders::CreateParticipantConnectionInputBuilder::default()
    }
}

/// A builder for [`CreateParticipantConnectionInput`](crate::operation::create_participant_connection::CreateParticipantConnectionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateParticipantConnectionInputBuilder {
    pub(crate) r#type: ::std::option::Option<::std::vec::Vec<crate::types::ConnectionType>>,
    pub(crate) participant_token: ::std::option::Option<::std::string::String>,
    pub(crate) connect_participant: ::std::option::Option<bool>,
}
impl CreateParticipantConnectionInputBuilder {
    /// Appends an item to `r#type`.
    ///
    /// To override the contents of this collection use [`set_type`](Self::set_type).
    ///
    /// <p>Type of connection information required. This can be omitted if <code>ConnectParticipant</code> is <code>true</code>.</p>
    pub fn r#type(mut self, input: crate::types::ConnectionType) -> Self {
        let mut v = self.r#type.unwrap_or_default();
        v.push(input);
        self.r#type = ::std::option::Option::Some(v);
        self
    }
    /// <p>Type of connection information required. This can be omitted if <code>ConnectParticipant</code> is <code>true</code>.</p>
    pub fn set_type(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ConnectionType>>,
    ) -> Self {
        self.r#type = input;
        self
    }
    /// <p>This is a header parameter.</p>
    /// <p>The ParticipantToken as obtained from <a href="https://docs.aws.amazon.com/connect/latest/APIReference/API_StartChatContact.html">StartChatContact</a> API response.</p>
    pub fn participant_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.participant_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>This is a header parameter.</p>
    /// <p>The ParticipantToken as obtained from <a href="https://docs.aws.amazon.com/connect/latest/APIReference/API_StartChatContact.html">StartChatContact</a> API response.</p>
    pub fn set_participant_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.participant_token = input;
        self
    }
    /// <p>Amazon Connect Participant is used to mark the participant as connected for customer participant in message streaming, as well as for agent or manager participant in non-streaming chats.</p>
    pub fn connect_participant(mut self, input: bool) -> Self {
        self.connect_participant = ::std::option::Option::Some(input);
        self
    }
    /// <p>Amazon Connect Participant is used to mark the participant as connected for customer participant in message streaming, as well as for agent or manager participant in non-streaming chats.</p>
    pub fn set_connect_participant(mut self, input: ::std::option::Option<bool>) -> Self {
        self.connect_participant = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateParticipantConnectionInput`](crate::operation::create_participant_connection::CreateParticipantConnectionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_participant_connection::CreateParticipantConnectionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_participant_connection::CreateParticipantConnectionInput {
                r#type: self.r#type,
                participant_token: self.participant_token,
                connect_participant: self.connect_participant,
            },
        )
    }
}
