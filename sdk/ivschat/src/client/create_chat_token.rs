// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateChatToken`](crate::operation::create_chat_token::builders::CreateChatTokenFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`room_identifier(impl ::std::convert::Into<String>)`](crate::operation::create_chat_token::builders::CreateChatTokenFluentBuilder::room_identifier) / [`set_room_identifier(Option<String>)`](crate::operation::create_chat_token::builders::CreateChatTokenFluentBuilder::set_room_identifier): <p>Identifier of the room that the client is trying to access. Currently this must be an ARN. </p>
    ///   - [`user_id(impl ::std::convert::Into<String>)`](crate::operation::create_chat_token::builders::CreateChatTokenFluentBuilder::user_id) / [`set_user_id(Option<String>)`](crate::operation::create_chat_token::builders::CreateChatTokenFluentBuilder::set_user_id): <p>Application-provided ID that uniquely identifies the user associated with this token. This can be any UTF-8 encoded text.</p>
    ///   - [`capabilities(Vec<ChatTokenCapability>)`](crate::operation::create_chat_token::builders::CreateChatTokenFluentBuilder::capabilities) / [`set_capabilities(Option<Vec<ChatTokenCapability>>)`](crate::operation::create_chat_token::builders::CreateChatTokenFluentBuilder::set_capabilities): <p>Set of capabilities that the user is allowed to perform in the room. Default: None (the capability to view messages is implicitly included in all requests).</p>
    ///   - [`session_duration_in_minutes(i32)`](crate::operation::create_chat_token::builders::CreateChatTokenFluentBuilder::session_duration_in_minutes) / [`set_session_duration_in_minutes(i32)`](crate::operation::create_chat_token::builders::CreateChatTokenFluentBuilder::set_session_duration_in_minutes): <p>Session duration (in minutes), after which the session expires. Default: 60 (1 hour).</p>
    ///   - [`attributes(HashMap<String, String>)`](crate::operation::create_chat_token::builders::CreateChatTokenFluentBuilder::attributes) / [`set_attributes(Option<HashMap<String, String>>)`](crate::operation::create_chat_token::builders::CreateChatTokenFluentBuilder::set_attributes): <p>Application-provided attributes to encode into the token and attach to a chat session. Map keys and values can contain UTF-8 encoded text. The maximum length of this field is 1 KB total.</p>
    /// - On success, responds with [`CreateChatTokenOutput`](crate::operation::create_chat_token::CreateChatTokenOutput) with field(s):
    ///   - [`token(Option<String>)`](crate::operation::create_chat_token::CreateChatTokenOutput::token): <p>The issued client token, encrypted.</p>
    ///   - [`token_expiration_time(Option<DateTime>)`](crate::operation::create_chat_token::CreateChatTokenOutput::token_expiration_time): <p>Time after which the token is no longer valid and cannot be used to connect to a room. This is an ISO 8601 timestamp; <i>note that this is returned as a string</i>.</p>
    ///   - [`session_expiration_time(Option<DateTime>)`](crate::operation::create_chat_token::CreateChatTokenOutput::session_expiration_time): <p>Time after which an end user's session is no longer valid. This is an ISO 8601 timestamp; <i>note that this is returned as a string</i>.</p>
    /// - On failure, responds with [`SdkError<CreateChatTokenError>`](crate::operation::create_chat_token::CreateChatTokenError)
    pub fn create_chat_token(
        &self,
    ) -> crate::operation::create_chat_token::builders::CreateChatTokenFluentBuilder {
        crate::operation::create_chat_token::builders::CreateChatTokenFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
